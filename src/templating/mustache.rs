use std::{collections::HashMap, fmt::format};

struct ParsableInput {
    input: String,
    open_delimiter: String,
    closing_delimiter: String,
    position: usize,
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub enum MustacheValue {
    Scalar(String),
    Object(HashMap<String, MustacheValue>),
    Array(Vec<HashMap<String, MustacheValue>>),
    Lamba(),
    // TODO implement lambas.
}

#[derive(Debug)]
pub struct MustacheData {
    values: HashMap<String, MustacheValue>,
    // TODO implement partials.
}

pub struct MustacheParser {
    input: ParsableInput,
}

#[derive(Debug)]
pub struct TokenCollection {
    name: String,
    inverted: bool,
    tokens: Vec<TokenCollectionItem>,
}

#[derive(Debug)]
pub enum TokenCollectionItem {
    Token(MustacheToken),
    InnerCollection(TokenCollection),
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

    pub fn can_have_leading_new_line(&self) -> bool {
        match self {
            MustacheToken::SectionStart(_) => true,
            MustacheToken::InvertedSectionStart(_) => true,
            MustacheToken::Unmodified(_) => false,
            MustacheToken::Comment(_) => true,
            MustacheToken::Partial(_) => true,
            MustacheToken::SetDelimiter(_) => true,
            MustacheToken::EscapedVariable(_) => false,
            MustacheToken::NonEscapedVariable(_) => false,
            MustacheToken::SectionEnd(_) => true,
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

                            /*if token.can_have_leading_new_line() {
                                match self.input.get_char(pos - 1) {
                                    Some(c) if c == '\n' => {
                                        next = pos - 2;
                                    }
                                    _ => next = pos - 1,
                                };

                                match self.input.get_char(end_index + 2) {
                                    Some(c) if c == '\n' => last_split = end_index + 3,
                                    _ => last_split = end_index + 2,
                                };
                                //println!("*** pre {:?}", self.input.get_char(pos - 1));
                                //println!("*** post {:?}", self.input.get_char(end_index + 2));
                            }*/

                            tokens.push(MustacheToken::Unmodified(
                                self.input
                                    .get_slice(last_split, pos)
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
    pub fn collect(&self) -> TokenCollection {
        collect_tokens("__main".to_string(), self.tokens.clone(), false)
    }

    pub fn replace(&self, data: MustacheData) -> String {
        let tc = self.collect();

        tc.process(&data.values)
    }
}

impl MustacheData {
    pub fn new(values: HashMap<String, MustacheValue>) -> MustacheData {
        MustacheData { values }
    }
}

impl TokenCollection {
    pub fn print(&self, indent: String) {
        println!(
            "{} *** Scope name: {} (is inverted: {})",
            indent, self.name, self.inverted
        );
        for i in 0..self.tokens.len() {
            match &self.tokens[i] {
                TokenCollectionItem::Token(t) => {
                    println!("{}{:?}", indent, t)
                }
                TokenCollectionItem::InnerCollection(ic) => {
                    ic.print(format!("{}    ", indent));
                }
            }
        }
        println!("{}*** End scope {}", indent, self.name)
    }

    pub fn process(&self, values: &HashMap<String, MustacheValue>) -> String {
        let mut result = String::new();

        for i in 0..self.tokens.len() {
            match &self.tokens[i] {
                TokenCollectionItem::Token(token) => match token {
                    MustacheToken::Unmodified(v) => result.push_str(&v),
                    MustacheToken::EscapedVariable(v) => match values.get(v) {
                        Some(mv) => match mv {
                            MustacheValue::Scalar(sv) => result.push_str(sv),
                            MustacheValue::Object(_) => {}
                            MustacheValue::Array(_) => {}
                            MustacheValue::Lamba() => {}
                        },
                        None => {}
                    },
                    MustacheToken::NonEscapedVariable(v) => match values.get(v) {
                        Some(mv) => match mv {
                            MustacheValue::Scalar(sv) => {
                                // TODO encode.
                                result.push_str(sv);
                            }
                            MustacheValue::Object(_) => {}
                            MustacheValue::Array(_) => {}
                            MustacheValue::Lamba() => {}
                        },
                        None => {}
                    },
                    MustacheToken::SectionStart(_) => {
                        // Should already be handled.
                    }
                    MustacheToken::InvertedSectionStart(_) => {
                        // Should already be handled.
                    }
                    MustacheToken::SectionEnd(_) => {
                        // Should already be handled.
                    }
                    MustacheToken::Comment(_) => {}
                    MustacheToken::Partial(_) => todo!(),
                    MustacheToken::SetDelimiter(_) => todo!(),
                },
                TokenCollectionItem::InnerCollection(ic) => {
                    match (ic.inverted, values.get(&ic.name)) {
                        (true, None) => {
                            result.push_str(&ic.process(&HashMap::new()));
                        }
                        (false, Some(mv)) => match mv {
                            MustacheValue::Scalar(_) => {}
                            MustacheValue::Object(ov) => {
                                //let r = ic.process(ov);

                                //match (r.chars().nth(0), r.chars().last()) {
                                //    (Some('\n'), Some('\n')) => result.push_str(&r[1..r.len() - 1]),
                                //    (Some('\n'), Some(_)) => result.push_str(&r[1..]),
                                //    (Some(_), Some('\n')) => result.push_str(&r[..r.len() - 1]),
                                //    _ => result.push_str(&r),
                                //}

                                result.push_str(&ic.process(ov));
                            }
                            MustacheValue::Array(av) => {
                                for ov in av {
                                    //  let r = ic.process(ov);

                                    //  match (r.chars().nth(0), r.chars().last()) {
                                    //      (Some('\n'), Some('\n')) => {
                                    //          result.push_str(&r[1..r.len() - 1])
                                    //      }
                                    //      (Some('\n'), Some(_)) => result.push_str(&r[1..]),
                                    //      (Some(_), Some('\n')) => result.push_str(&r[..r.len() - 1]),
                                    //      _ => result.push_str(&r),
                                    //  }

                                    result.push_str(&ic.process(ov));
                                    //result.push_str(r);
                                }
                            }
                            MustacheValue::Lamba() => todo!(),
                        },
                        (false, None) => {
                            //
                        }
                        (true, Some(_)) => {
                            // Do nothing in inverted sections if data exists.
                        }
                    }
                }
            }
        }

        result
    }
}

pub fn collect_tokens(name: String, tokens: Vec<MustacheToken>, inverted: bool) -> TokenCollection {
    let mut inner_collection = Vec::<MustacheToken>::new();
    let mut collected = Vec::<TokenCollectionItem>::new();
    let mut scope_name: Option<String> = None;
    let mut inner_inverted = false;

    for i in 0..tokens.len() {
        let token = &tokens[i];

        match (&scope_name, token) {
            (None, MustacheToken::Unmodified(_)) => {
                collected.push(TokenCollectionItem::Token(token.clone()))
            }
            (None, MustacheToken::EscapedVariable(_)) => {
                collected.push(TokenCollectionItem::Token(token.clone()))
            }
            (None, MustacheToken::NonEscapedVariable(_)) => {
                collected.push(TokenCollectionItem::Token(token.clone()))
            }
            (None, MustacheToken::SectionStart(ssn)) => {
                scope_name = Some(ssn.clone());
            }
            (None, MustacheToken::InvertedSectionStart(isn)) => {
                scope_name = Some(isn.clone());
                inner_inverted = true;
            }
            (None, MustacheToken::SectionEnd(_)) => todo!(),
            (None, MustacheToken::Comment(_)) => {
                collected.push(TokenCollectionItem::Token(token.clone()))
            }
            (None, MustacheToken::Partial(_)) => {
                collected.push(TokenCollectionItem::Token(token.clone()))
            }
            (None, MustacheToken::SetDelimiter(_)) => {
                collected.push(TokenCollectionItem::Token(token.clone()))
            }
            (Some(_), MustacheToken::Unmodified(_)) => inner_collection.push(token.clone()),
            (Some(_), MustacheToken::EscapedVariable(_)) => inner_collection.push(token.clone()),
            (Some(_), MustacheToken::NonEscapedVariable(_)) => inner_collection.push(token.clone()),
            (Some(_), MustacheToken::SectionStart(_)) => inner_collection.push(token.clone()),
            (Some(_), MustacheToken::InvertedSectionStart(_)) => {
                inner_collection.push(token.clone())
            }
            (Some(sn), MustacheToken::SectionEnd(sen)) if sn == sen => {
                collected.push(TokenCollectionItem::InnerCollection(collect_tokens(
                    sn.clone(),
                    inner_collection.clone(),
                    inner_inverted,
                )));
                scope_name = None;
                inner_collection.clear();
            }
            (Some(_), MustacheToken::SectionEnd(_)) => inner_collection.push(token.clone()),
            (Some(_), MustacheToken::Comment(_)) => inner_collection.push(token.clone()),
            (Some(_), MustacheToken::Partial(_)) => inner_collection.push(token.clone()),
            (Some(_), MustacheToken::SetDelimiter(_)) => inner_collection.push(token.clone()),
        }
    }

    TokenCollection {
        name,
        inverted,
        tokens: collected,
    }
}
