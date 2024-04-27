use std::{
    io::{self, Write},
    mem,
};

use crate::parser::{Parser, ParserResult};

mod bounds_writer;
mod commands;
mod linq_like_writer;
mod parser;
mod processing_writer;
//mod testing;

//use crate::commands::*;

fn print(parser: &Parser) {
    println!(
        "   parsed: {}",
        linq_like_writer::write_one(&parser.exprs, *parser.stat_starts.last().unwrap())
    );
    std::io::stdout().flush().unwrap();
}
fn main() {
    println!("size of parser: {}", mem::size_of::<Parser>());
    // println!(
    //     "name of eq: {}",
    //     format!("{:#?}",(&commands::Expr::Eq {
    //         locs: vec![0, 1],
    //         name_start: 7,
    //         name: "inch".as_bytes().to_vec(),
    //         value_index: 1
    //     }))
    // );
    println!("Input text to be parsed:");
    //let mut buf= Vec::new();
    //testing::test_ast1();
    let mut input = io::stdin().lock();
    //let stats = parser_state::parse(&mut input);
    //println!("{}",processing_writer::write(&stats))
    //let s:String = Default::default();

    let mut parser = Parser::new(&mut input);

    //parser.vars.insert("inch".as_bytes().to_vec());

    //let mut result = parser.step();

    loop {
        match parser.step() {
            ParserResult::MatchedLine(_) | ParserResult::FailedLine(_) => print(&parser),
            ParserResult::NoInput => break,
            _ => {}
        }
    }
    //println!();
    //println!("== {:?}", parser.exprs.vec);
    println!(
        "   whole program:\n{}",
        linq_like_writer::write(&parser.exprs, &parser.stat_starts)
    );
    println!(
        "   java program:\n{}",
        processing_writer::write(&parser.exprs, &parser.stat_starts)
    );
}

// while !matches!(
//     result,
//     ParserResult::NoInput | ParserResult::MatchedLine(_) | ParserResult::FailedLine(_)
// ) {
//     result = parser.step();
// }
// //print!("{:?},", result);
// print!(
//     "{}",
//     linq_like_writer::write(&parser.exprs, &parser.stat_starts)
// );
// std::io::stdout().flush().unwrap();
