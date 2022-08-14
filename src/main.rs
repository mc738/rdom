use rendering::html;
use templating::mustache::MustacheParser;

use crate::{
    core::formatting::Formatters,
    parsing::{inline_parser, processing},
    templating::mustache::{MustacheData, MustacheValue},
};
use std::{collections::HashMap, fs};

use crate::parsing::block_parser::Input;

pub mod core;
pub mod parsing;
pub mod rendering;
pub mod templating;

fn main() {
    let template =
        fs::read_to_string("/home/max/Projects/rdom/examples/test_template.mustache").unwrap();

    let mut mustache_parser = MustacheParser::new(template);

    let t = mustache_parser.run();

    let ct = t.collect();

    ct.print("".to_string());
    //println!("{:?}", ct);

    let values: HashMap<String, MustacheValue> = vec![
        (
            "test_ver".to_string(),
            MustacheValue::Scalar("Hello, World!".to_string()),
        ),
        (
            "non_escaped".to_string(),
            MustacheValue::Scalar("<h1>Test!</h1>".to_string()),
        ),
        (
            "section".to_string(),
            MustacheValue::Array(vec![
                vec![
                    (
                        "section_title".to_string(),
                        MustacheValue::Scalar("Section 1".to_string()),
                    ),
                    (
                        "inner_section".to_string(),
                        MustacheValue::Object(
                            vec![(
                                "deep_value".to_string(),
                                MustacheValue::Scalar("lorem 1".to_string()),
                            )]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
                vec![
                    (
                        "section_title".to_string(),
                        MustacheValue::Scalar("Section 2".to_string()),
                    ),
                    (
                        "inner_section".to_string(),
                        MustacheValue::Object(
                            vec![(
                                "deep_value".to_string(),
                                MustacheValue::Scalar("lorem 2".to_string()),
                            )]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
                vec![(
                    "section_title".to_string(),
                    MustacheValue::Scalar("Section 3".to_string()),
                )]
                .into_iter()
                .collect(),
            ]),
        ),
    ]
    .into_iter()
    .collect();

    let data = MustacheData::new(values);

    let result = t.replace(data);

    println!("\n\n\n********************\n\n\n");

    println!("{}", result);

    println!("\n\n\n********************\n\n\n");

    //let t1 = "Hello, ***World!***".to_string();

    //let t2 = "How are you, `max`?".to_string();

    //let r1 = inline_parser::parse_inline_content(t1);

    //let r2 = inline_parser::parse_inline_content(t2);

    //println!("{:?}", r1);

    //println!("{:?}", r2);

    let raw = fs::read_to_string("/home/max/Projects/rdom/examples/example1.md").unwrap();

    let doc: Vec<&str> = raw.split("\n").collect();

    let input = Input::new(doc);

    let formatters = Formatters::default();

    //println!("{:?}", input);

    let tokens = input.parse_blocks(&formatters);

    let blocks = processing::process_tokens(tokens);

    let rendered = html::render(blocks);

    let pd: HashMap<String, MustacheValue> = vec![
        (
            "title".to_string(),
            MustacheValue::Scalar("RDOM demo".to_string()),
        ),
        (
            "styles".to_string(),
            MustacheValue::Array(vec![vec![(
                "url".to_string(),
                MustacheValue::Scalar("./css/style.css".to_string()),
            )]
            .into_iter()
            .collect()]),
        ),
        (
            "scripts".to_string(),
            MustacheValue::Array(vec![vec![(
                "url".to_string(),
                MustacheValue::Scalar("./js/index.js".to_string()),
            )]
            .into_iter()
            .collect()]),
        ),
        (
            "content".to_string(),
            MustacheValue::Scalar(rendered.join("")),
        ),
    ]
    .into_iter()
    .collect();

    let rt = fs::read_to_string("/home/max/Projects/rdom/examples/test_page.mustache").unwrap();

    let mut pp = MustacheParser::new(rt);

    let pt = pp.run();

    let r = pt.replace(MustacheData::new(pd));

    fs::write("/home/max/Projects/rdom/examples/demo_page.html", r);

    //println!("<article>\n    {}\n</article>", rendered.join("\n    "));

    //fs::write(
    //    "/home/max/Projects/example.html",
    //    format!("<article>\n    {}\n</article>", rendered.join("\n    ")),
    //);

    for x in 0..rendered.len() {
        println!("{:?}", rendered[x]);
    }

    //let _ = blocks.into_iter().map(|b| println!("{:?}", b));

    //println!("{:?}", blocks);

    //let lt = LineType::new("``` d";

    //println!("{:?}", &"```"[0..3]);

    //println!("{:?}", lt);
}
