use std::{io, mem};

use crate::parser::{Parser, ParserResult};

mod commands;
mod linq_like_writer;
mod parser;
mod processing_writer;
mod testing;

//use crate::commands::*;
fn main() {
    println!("size of parser: {}", mem::size_of::<Parser>());
    println!(
        "name of eq: {}",
        format!("{:#?}",(&commands::Expr::Eq {
            locs: vec![0, 1],
            name_start: 7,
            name: "inch".as_bytes().to_vec(),
            value_index: 1
        }))
    );
    println!("Input text to be parsed:");
    //let mut buf= Vec::new();
    //testing::test_ast1();
    let mut input = io::stdin().lock();
    //let stats = parser_state::parse(&mut input);
    //println!("{}",processing_writer::write(&stats))
    //let s:String = Default::default();
    let mut _parser = Parser::new(&mut input);
    // while !matches!(parser.step(), ) {}
    // println!(
    //     "{}",
    //     processing_writer::write(&parser.exprs, &parser.stat_starts)
    // )
}
