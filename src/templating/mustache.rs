use std::collections::HashMap;

struct ParsableInput {
    input: String,
    open_delimiter: String,
    closing_delimiter: String,
    position: usize,
}

#[derive(Debug)]
pub enum MustacheToken {
    Unmodified(String),
    EscapedVariable(String),
    NonEscapedVariable(String),
    SectionStart(String),
    InvertedSectionStart(String),
    SectionEnd(String),
    Comment(String),
    Partial(String),
    SetDelimiter((String, String)),
}

pub enum MustacheValue {
    Scalar(String),
    Object(HashMap<String, MustacheValue>),
    Array(Vec<MustacheValue>),
    Lamba(),
    // TODO implement lambas.
}

pub struct MustacheData {
    values: HashMap<String, MustacheValue>,
    // TODO implement partials.
}

pub struct MustacheParser {
    input: ParsableInput,
}

#[derive(Debug)]
pub struct MustacheTemplate {
    tokens: Vec<MustacheToken>,
}

impl ParsableInput {
    pub fn set_position(&mut self, i: usize) {
        self.position = i;
    }

    pub fn advance(&mut self) {
        self.set_position(self.position + 1)
    }

    fn in_bounds(&self, i: usize) -> bool {
        i < self.input.len()
    }

    fn is_in_bounds(&self) -> bool {
        self.in_bounds(self.position)
    }

    fn get_char(&self, i: usize) -> Option<char> {
        self.input.chars().nth(i)
    }

    fn check_char(&self, c: char, i: usize) -> bool {
        match self.input.chars().nth(i) {
            Some(fc) => fc == c,
            None => false,
        }
    }

    fn is_two_chars(&self, c1: char, c2: char) -> bool {
        match (
            self.check_char(c1, self.position),
            self.check_char(c2, self.position + 1),
        ) {
            (true, true) => true,
            _ => false,
        }
    }

    fn next_non_nested(&mut self) -> Option<usize> {
        // TODO clean up.
        let mut nest_count: i16 = 0;
        let mut found: Option<usize> = None;

        let (oc1, oc2) = (
            self.open_delimiter.chars().nth(0).unwrap(),
            self.open_delimiter.chars().nth(1).unwrap(),
        );
        let (cc1, cc2) = (
            self.closing_delimiter.chars().nth(0).unwrap(),
            self.closing_delimiter.chars().nth(1).unwrap(),
        );

        loop {
            match (
                self.is_in_bounds(),
                self.is_two_chars(oc1, oc2),
                self.is_two_chars(cc1, cc2),
            ) {
                (true, true, _) => {
                    nest_count += 1;
                    self.advance();
                }
                (true, _, true) => match nest_count == 1 {
                    true => {
                        found = Some(self.position);
                        break;
                    }
                    false => {
                        nest_count -= 1;
                        self.advance();
                    }
                },
                (true, false, false) => {
                    self.advance();
                }
                (false, _, _) => {
                    break;
                }
            }
        }

        found
    }

    pub fn len(&self) -> usize {
        self.input.len()
    }

    pub fn get_slice(&self, start_index: usize, end_index: usize) -> Option<String> {
        match (
            self.in_bounds(start_index),
            self.in_bounds(end_index),
            start_index < end_index,
        ) {
            (true, true, true) => Some(self.input[start_index..end_index].to_string()),
            (false, _, _) => None,
            (_, false, _) => None,
            (_, _, false) => None,
        }
    }

    pub fn current_char(&self) -> char {
        self.get_char(self.position).unwrap_or_else(|| '\u{0000}')
    }
}

impl MustacheToken {
    pub fn create(input: String) -> MustacheToken {
        match input.chars().nth(0) {
            Some(c) if c == '#' => MustacheToken::SectionStart(get_token_name(input)),
            Some(c) if c == '/' => MustacheToken::SectionEnd(get_token_name(input)),
            Some(c) if c == '^' => MustacheToken::InvertedSectionStart(get_token_name(input)),
            Some(c) if c == '!' => MustacheToken::Comment(get_token_name(input)),
            Some(c) if c == '>' => MustacheToken::Partial(get_token_name(input)),
            Some(c) if c == '=' => todo!("Switch delimited to be implemented"),
            Some(c) if c == '&' => MustacheToken::NonEscapedVariable(get_token_name(input)),
            Some(c) if c == '{' && input.chars().nth(input.len() - 2) == Some('}') => {
                MustacheToken::NonEscapedVariable(input[1..input.len() - 2].trim().to_string())
            }
            Some(_) => MustacheToken::EscapedVariable(input),
            None => todo!("Handle blank tokens"),
        }
    }
}

fn get_token_name(input: String) -> String {
    input[1..].trim().to_string()
}

impl MustacheValue {
    pub fn is_array(&self) -> bool {
        match self {
            MustacheValue::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            MustacheValue::Object(_) => true,
            _ => false,
        }
    }

    pub fn is_scalar(&self) -> bool {
        match self {
            MustacheValue::Scalar(_) => true,
            _ => false,
        }
    }

    pub fn is_lamba(&self) -> bool {
        match self {
            MustacheValue::Lamba() => true,
            _ => false,
        }
    }
}

impl MustacheParser {
    pub fn new(input: String) -> MustacheParser {
        MustacheParser {
            input: ParsableInput {
                input,
                open_delimiter: "{{".to_string(),
                closing_delimiter: "}}".to_string(),
                position: 0,
            },
        }
    }

    pub fn run(&mut self) -> MustacheTemplate {
        let mut tokens = Vec::<MustacheToken>::new();
        let mut last_split: usize = 0;

        loop {
            match (self.input.is_in_bounds(), self.input.is_two_chars('{', '{')) {
                (true, true) => {
                    let pos = self.input.position;
                    match self.input.next_non_nested() {
                        Some(end_index) => {
                            let token = MustacheToken::create(
                                self.input
                                    .get_slice(pos + 2, end_index)
                                    .unwrap_or_else(|| "".to_string()),
                            );

                            tokens.push(MustacheToken::Unmodified(
                                self.input
                                    .get_slice(last_split, pos - 1)
                                    .unwrap_or_else(|| "".to_string()),
                            ));
                            tokens.push(token);
                            last_split = end_index + 2;
                            self.input.set_position(last_split);
                        }
                        None => {
                            self.input.advance();
                        }
                    }
                }
                (true, false) => {
                    self.input.advance();
                }
                (false, _) => {
                    tokens.push(MustacheToken::Unmodified(
                        self.input
                            .get_slice(last_split, self.input.len() - 1)
                            .unwrap_or_else(|| "".to_string()),
                    ));
                    break;
                }
            }
        }

        MustacheTemplate { tokens }
    }
}

impl MustacheTemplate {
    pub fn replace(&self, data: MustacheData) -> String {
        let mut result = String::new();

        result
    }
}
