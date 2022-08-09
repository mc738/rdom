use crate::{
    core::formatting::Formatters,
    parsing::{inline_parser, processing},
};
use std::fs;

use crate::parsing::block_parser::Input;

pub mod core;
pub mod parsing;

fn main() {
    //let t1 = "Hello, ***World!***".to_string();

    //let t2 = "How are you, `max`?".to_string();

    //let r1 = inline_parser::parse_inline_content(t1);

    //let r2 = inline_parser::parse_inline_content(t2);

    //println!("{:?}", r1);

    //println!("{:?}", r2);

    let raw = fs::read_to_string("/home/max/Projects/rdom/examples/example1.md").unwrap();

    let doc: Vec<&str> = raw.split("\n").collect();

    let input = Input::new(doc);

    let formatters = Formatters::empty();

    //println!("{:?}", input);

    let tokens = input.parse_blocks(&formatters);

    let blocks = processing::process_tokens(tokens);

    for x in 0..blocks.len() {
        println!("{:?}", blocks[x]);
    }

    //let _ = blocks.into_iter().map(|b| println!("{:?}", b));

    //println!("{:?}", blocks);

    //let lt = LineType::new("``` d";

    //println!("{:?}", &"```"[0..3]);

    //println!("{:?}", lt);
}
