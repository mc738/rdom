use crate::core::documents::{InlineContent, InlineLink, InlineSpan, InlineText, Style};

fn in_bounds(s: &String, i: usize) -> bool {
    i < s.len()
}

fn get_char(s: &String, i: usize) -> Option<char> {
    s.chars().nth(i)
}

fn look_ahead(s: &String, i: usize) -> Option<char> {
    get_char(s, i)
}

fn look_back(s: &String, i: usize) -> Option<char> {
    get_char(s, i - 1)
}

fn is_control_char(c: char) -> bool {
    c == '_' || c == '*' || c == '`' || c == '['
}

fn compare_look_ahead(s: &String, pattern: &str, i: usize) -> bool {
    if in_bounds(s, i + pattern.len() - 1) {
        let slice = &s[i..(i + pattern.len())];

        slice == pattern
    } else {
        false
    }
}

pub fn read_until_char(s: &String, c: char, inclusive: bool, from: usize) -> (String, usize) {
    let mut i = from;

    loop {
        match s.chars().nth(i) {
            Some(fc) if fc == c => {
                break;
            }
            Some(_) => {
                i = i + 1;
            }
            None => {
                i = s.len() - 1;
                break;
            }
        }
    }

    (s[from..i].to_string(), if inclusive { i + 1 } else { i })
}

fn read_until_control_char(s: &String, from: usize) -> (String, usize) {
    let mut i = from;

    loop {
        match s.chars().nth(i) {
            Some(fc) if is_control_char(fc) => {
                //println!("Control - {}", fc);
                break;
            }
            Some(_) => {
                i = i + 1;
            }
            None => {
                // plus 1 because this removes 1 normally.
                i = s.len() + 1;
                break;
            }
        }
    }

    (s[from..(i - 1)].to_string(), i)
}

fn read_until_string(s: &String, pattern: &str, inclusive: bool, from: usize) -> (String, usize) {
    let mut i = from;

    //let mut chars

    loop {
        match (in_bounds(s, i), compare_look_ahead(&s, pattern, i)) {
            (true, true) => {
                break;
            }
            (true, false) => {
                i = i + 1;
            }
            (false, _) => {
                i = s.len();
                break;
            }
        }
    }

    (
        s[from..i].to_string(),
        if inclusive { i + pattern.len() } else { i },
    )
}

pub fn parse_inline_content(input: String) -> Vec<InlineContent> {
    let mut content = Vec::<InlineContent>::new();
    let mut i: usize = 0;

    loop {
        match input.chars().nth(i) {
            Some(c) if c == '*' => match (look_ahead(&input, i + 1), look_ahead(&input, i + 2)) {
                (None, _) => {
                    // TODO need to handle this!
                    break;
                }
                (Some(_), None) => {
                    // TODO need to handle this!
                    break;
                }
                (Some(c1), Some(c2)) if c1 == '*' && c2 == '*' => {
                    let (v, next) = read_until_string(&input, "***", true, i + 3);

                    content.push(InlineContent::Span(InlineSpan {
                        content: v,
                        style: Style::Ref(vec!["b".to_string(), "i".to_string()]),
                    }));
                    i = next;
                }
                (Some(c1), Some(c2)) if c1 == '*' && c2 != '*' => {
                    let (v, next) = read_until_string(&input, "**", true, i + 2);

                    content.push(InlineContent::Span(InlineSpan {
                        content: v,
                        style: Style::Ref(vec!["b".to_string()]),
                    }));
                    i = next;
                }
                (Some(_), Some(_)) => {
                    let (v, next) = read_until_char(&input, '*', true, i + 1);

                    content.push(InlineContent::Span(InlineSpan {
                        content: v,
                        style: Style::Ref(vec!["i".to_string()]),
                    }));
                    i = next;
                }
            },
            Some(c) if c == '`' => {
                let (v, next) = read_until_char(&input, '`', true, i + 1);

                content.push(InlineContent::Span(InlineSpan {
                    content: v,
                    style: Style::Ref(vec!["code".to_string()]),
                }));

                i = next;
            }
            Some(c) if c == '[' => {
                let (text, next1) = read_until_char(&input, ']', true, i + 1);

                let (url, next2) = read_until_char(&input, ')', true, next1 + 1);

                content.push(InlineContent::Link(InlineLink::new(
                    Style::Default,
                    url,
                    text,
                )));

                i = next2;
            }
            Some(_) => {
                let (v, next) = read_until_control_char(&input, i);

                content.push(InlineContent::Text(InlineText { content: v }));

                i = next;
            }
            None => {
                break;
            }
        }
    }

    content
}
