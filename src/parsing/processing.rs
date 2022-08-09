use crate::core::documents::{
    Block, CodeBlock, HeaderBlock, HeaderLevel, ImageBlock, ListBlock, ListItem, ParagraphBlock,
    Style,
};

use super::block_parser::BlockToken;
use super::inline_parser;

fn create_header_block(s: String, style: Style) -> HeaderBlock {
    let mut i: usize = 0;

    loop {
        match s.chars().nth(i) {
            Some(c) if c == '#' => {
                i = i + 1;
            }
            Some(_) => {
                break;
            }
            None => {
                break;
            }
        }
    }

    let (level, indexed) = match i {
        1 => (HeaderLevel::H1, true),
        2 => (HeaderLevel::H2, true),
        3 => (HeaderLevel::H3, false),
        4 => (HeaderLevel::H4, false),
        5 => (HeaderLevel::H5, false),
        6 => (HeaderLevel::H6, false),
        _ => (HeaderLevel::H6, false),
    };

    let content = inline_parser::parse_inline_content(s[i..s.len()].trim().to_string());

    HeaderBlock::new(style, level, content, indexed)
}

fn create_paragraph_block(s: String, style: Style) -> ParagraphBlock {
    ParagraphBlock::new(style, inline_parser::parse_inline_content(s))
}

fn create_code_block(s: String, language: Option<String>, style: Style) -> CodeBlock {
    println!("{}", s);
    CodeBlock::new(style, s, language)
}

fn create_image_block(s: String, style: Style) -> ImageBlock {
    let (alt_text, next1) = inline_parser::read_until_char(&s, ']', false, 2);

    let (source, next2) = inline_parser::read_until_char(&s, ' ', false, next1 + 2);

    let (title, next3) = inline_parser::read_until_char(&s, '"', false, next2 + 2);

    let (hw, _) = inline_parser::read_until_char(&s, '}', false, next3 + 3);

    let (height, width) = hw.trim().split(',').fold((None, None), |(h, w), x| {
        let mut ss = x.trim().split(':');

        // ss.nth(0) in match because the first ss.nth(0) consumes item 0.
        match ss.nth(0) {
            Some("height") => (ss.nth(0).map(|v| v.to_string()), w),
            Some("width") => (h, ss.nth(0).map(|v| v.to_string())),
            _ => (h, w),
        }
    });

    ImageBlock::new(style, source, title, alt_text, height, width)
}

fn create_list_item(s: String, style: Style) -> ListItem {
    ListItem::new(style, inline_parser::parse_inline_content(s))
}

pub fn process_tokens(tokens: Vec<BlockToken>) -> Vec<Block> {
    let mut blocks = Vec::<Block>::new();

    let mut i: usize = 0;

    loop {
        match tokens.get(i) {
            Some(BlockToken::Header(s)) => {
                blocks.push(Block::header(create_header_block(
                    s.to_owned(),
                    Style::Default,
                )));
                i = i + 1;
            }
            Some(BlockToken::Paragraph(s)) => {
                blocks.push(Block::paragraph(create_paragraph_block(
                    s.to_owned(),
                    Style::Default,
                )));
                i = i + 1;
            }
            Some(BlockToken::CodeBlock(l, s)) => {
                blocks.push(Block::code(create_code_block(
                    s.to_owned(),
                    l.to_owned(),
                    Style::Default,
                )));
                i = i + 1;
            }
            Some(BlockToken::Image(s)) => {
                blocks.push(Block::image(create_image_block(
                    s.to_owned(),
                    Style::Default,
                )));
                i = i + 1;
            }
            Some(BlockToken::OrderedListItem(s)) => {
                let mut items = Vec::<String>::new();
                items.push(s.to_owned());
                i = i + 1;

                loop {
                    match tokens.get(i) {
                        Some(BlockToken::OrderedListItem(s)) => {
                            items.push(s.to_owned());
                            i = i + 1;
                        }
                        _ => {
                            break;
                        }
                    }
                }

                blocks.push(Block::list(ListBlock::new_ordered(
                    Style::Default,
                    items
                        .into_iter()
                        .map(|s| create_list_item(s, Style::Default))
                        .collect(),
                )));
            }
            Some(BlockToken::UnorderedListItem(s)) => {
                let mut items = Vec::<String>::new();
                items.push(s.to_owned());
                i = i + 1;

                loop {
                    match tokens.get(i) {
                        Some(BlockToken::UnorderedListItem(s)) => {
                            items.push(s.to_owned());
                            i = i + 1;
                        }
                        _ => {
                            break;
                        }
                    }
                }

                blocks.push(Block::list(ListBlock::new_unordered(
                    Style::Default,
                    items
                        .into_iter()
                        .map(|s| create_list_item(s, Style::Default))
                        .collect(),
                )));
            }
            Some(BlockToken::Empty) => {
                i = i + 1;
            }
            Some(BlockToken::Unknown(_)) => {
                i = i + 1;
            }
            None => {
                break;
            }
        }
    }

    blocks
}
