use rendering::html;
use templating::mustache::MustacheParser;

use crate::{
    core::formatting::Formatters,
    parsing::{inline_parser, processing},
};
use std::fs;

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
