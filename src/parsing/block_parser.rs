use crate::core::formatting::Formatters;

#[derive(Debug, Clone, PartialEq)]
pub enum LineType {
    Header,
    Text,
    OrderedListItem,
    UnorderedListItem,
    CodeBlockDelimited,
    Image,
    Empty,
}

#[derive(Clone, Debug)]
pub struct Line {
    number: usize,
    text: String,
    line_type: LineType,
}

#[derive(Debug)]
pub enum BlockToken {
    Paragraph(String),
    Header(String),
    OrderedListItem(String),
    UnorderedListItem(String),
    CodeBlock(Option<String>, String),
    Image(String),
    Empty,
    Unknown(String),
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<Line>,
}

impl LineType {
    pub fn new(line: &str) -> LineType {
        match line {
            s if s.is_empty() => LineType::Empty,
            s if s.len() < 3 => LineType::Text,
            s if s.chars().nth(0) == Some('#') => LineType::Header,
            s if &s[0..3] == "```" => LineType::CodeBlockDelimited,
            s if &s[0..2] == "* " => LineType::UnorderedListItem,
            s if s.chars().nth(0).unwrap_or('\n').is_digit(10)
                && (s.chars().nth(1) == Some('.') || s.chars().nth(2) == Some('.')) =>
            {
                LineType::OrderedListItem
            }
            s if s.chars().nth(0) == Some('!') => LineType::Image,
            _ => LineType::Text,
        }
    }
}

impl Input {
    pub fn new(lines: Vec<&str>) -> Input {
        let ls: Vec<Line> = lines
            .into_iter()
            .enumerate()
            .map(|(i, l)| Line {
                number: i,
                text: l.to_string(),
                line_type: LineType::new(l),
            })
            .collect();

        Input { lines: ls }
    }

    pub fn try_get_line(&self, index: usize) -> Option<Line> {
        if self.in_bounds(index) {
            Some(self.lines[index].clone())
        } else {
            None
        }
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn in_bounds(&self, i: usize) -> bool {
        i < self.lines.len()
    }

    pub fn get_line_type(&self, i: usize) -> LineType {
        if self.in_bounds(i) {
            self.lines[i].line_type.clone()
        } else {
            LineType::Empty
        }
    }

    pub fn try_get_until_end_or_type(
        &self,
        curr: usize,
        line_type: LineType,
    ) -> (Vec<Line>, usize) {
        let mut i = curr;
        let mut lines = Vec::new();

        loop {
            if self.in_bounds(i) && self.lines[i].line_type != line_type {
                lines.push(self.lines[i].clone());
                i = i + 1;
            } else {
                break;
            }
        }

        (lines, i)
    }

    pub fn try_get_until_end_or_not_type(
        &self,
        curr: usize,
        line_type: LineType,
    ) -> (Vec<Line>, usize) {
        let mut i = curr;
        let mut lines = Vec::new();

        loop {
            if self.in_bounds(i) && self.lines[i].line_type == line_type {
                lines.push(self.lines[i].clone());
                i = i + 1;
            } else {
                break;
            }
        }

        (lines, i)
    }

    fn format_block_text(lines: Vec<Line>, formatters: &Formatters) -> String {
        let mut s = String::new();

        for l in lines {
            s.push_str(formatters.run_preprocessors(l.text).as_str());
        }

        /*
        lines
            .into_iter()
            .fold(s, |acc, l| acc.push_str(preprocessors.run(l.text).as_str()));
        */

        formatters.run(s)
    }

    pub fn try_parse_paragraph(
        &self,
        curr: usize,
        formatters: &Formatters,
    ) -> Option<(BlockToken, usize)> {
        match self.get_line_type(curr) {
            LineType::Text => {
                let (lines, next) = self.try_get_until_end_or_not_type(curr, LineType::Text);

                Some((
                    BlockToken::Paragraph(Input::format_block_text(lines, formatters)),
                    next,
                ))
            }
            _ => None,
        }
    }

    pub fn try_parse_header(
        &self,
        curr: usize,
        formatters: &Formatters,
    ) -> Option<(BlockToken, usize)> {
        match self.get_line_type(curr) {
            LineType::Header => {
                let lines = vec![self.lines[curr].clone()];
                Some((
                    BlockToken::Header(Input::format_block_text(lines, formatters)),
                    curr + 1,
                ))
            }
            _ => None,
        }
    }

    pub fn try_parse_code_block(
        &self,
        curr: usize,
        formatters: &Formatters,
    ) -> Option<(BlockToken, usize)> {
        match self.get_line_type(curr) {
            LineType::CodeBlockDelimited => {
                let lang = self
                    .try_get_line(curr)
                    .map(|l| l.text.replace('`', "").trim().to_string());

                let (lines, next) =
                    self.try_get_until_end_or_type(curr + 1, LineType::CodeBlockDelimited);

                Some((
                    BlockToken::CodeBlock(lang, Input::format_block_text(lines, formatters)),
                    next,
                ))
            }
            _ => None,
        }
    }

    pub fn try_parse_ordered_list_item(
        &self,
        curr: usize,
        formatters: &Formatters,
    ) -> Option<(BlockToken, usize)> {
        match self.get_line_type(curr) {
            LineType::OrderedListItem => match self.get_line_type(curr + 1) {
                LineType::Text => {
                    let (lines, next) =
                        self.try_get_until_end_or_not_type(curr + 1, LineType::Text);

                    Some((
                        BlockToken::OrderedListItem(Input::format_block_text(lines, formatters)),
                        next,
                    ))
                }
                _ => {
                    let lines = vec![self.lines[curr].clone()];

                    Some((
                        BlockToken::OrderedListItem(Input::format_block_text(lines, formatters)),
                        curr,
                    ))
                }
            },
            _ => None,
        }
    }

    pub fn try_parse_unordered_list_item(
        &self,
        curr: usize,
        formatters: &Formatters,
    ) -> Option<(BlockToken, usize)> {
        match self.get_line_type(curr) {
            LineType::UnorderedListItem => match self.get_line_type(curr + 1) {
                LineType::Text => {
                    let (lines, next) =
                        self.try_get_until_end_or_not_type(curr + 1, LineType::Text);

                    Some((
                        BlockToken::UnorderedListItem(Input::format_block_text(lines, formatters)),
                        next,
                    ))
                }
                _ => {
                    let lines = vec![self.lines[curr].clone()];

                    Some((
                        BlockToken::UnorderedListItem(Input::format_block_text(lines, formatters)),
                        curr,
                    ))
                }
            },
            _ => None,
        }
    }

    pub fn try_parse_image(
        &self,
        curr: usize,
        formatters: &Formatters,
    ) -> Option<(BlockToken, usize)> {
        match self.get_line_type(curr) {
            LineType::Image => Some((BlockToken::Image(self.lines[curr].text.clone()), curr)),
            _ => None,
        }
    }

    pub fn try_parse_empty(&self, curr: usize) -> Option<(BlockToken, usize)> {
        match self.get_line_type(curr) {
            LineType::Empty => Some((BlockToken::Empty, curr)),
            _ => None,
        }
    }

    pub fn try_parse_block(
        &self,
        curr: usize,
        formatters: &Formatters,
    ) -> Option<(BlockToken, usize)> {
        if self.in_bounds(curr) {
            self.try_parse_ordered_list_item(curr, &formatters)
                .or_else(|| self.try_parse_unordered_list_item(curr, &formatters))
                .or_else(|| self.try_parse_header(curr, &formatters))
                .or_else(|| self.try_parse_code_block(curr, &formatters))
                .or_else(|| self.try_parse_image(curr, &formatters))
                .or_else(|| self.try_parse_paragraph(curr, &formatters))
                .or_else(|| self.try_parse_empty(curr))
                .map(|(bt, n)| Some((bt, n + 1)))
                .unwrap_or(None)
        } else {
            // Not strictly needed.
            None
        }
    }

    pub fn parse_blocks(&self, formatters: &Formatters) -> Vec<BlockToken> {
        let mut tokens = Vec::<BlockToken>::new();
        let mut i: usize = 0;

        while self.in_bounds(i) {
            let (token, next) = self
                .try_parse_block(i, formatters)
                .unwrap_or((BlockToken::Unknown(self.lines[i].text.clone()), i + 1));

            tokens.push(token);
            i = next;
        }

        tokens
    }
}
