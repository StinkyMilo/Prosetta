use std::{
    io::{self, Read},
    mem,
};

use crate::parser::{Parser, ParserResult};

mod bounds_writer;
mod commands;
mod linq_like_writer;
mod parser;
mod processing_writer;
mod testing;

mod error_messages;
//mod testing;

//use crate::commands::*;

// fn print(parser: &Parser) {
//     println!(
//         "   parsed: {}",
//         linq_like_writer::write_one(&parser.exprs, *parser.stat_starts.last().unwrap())
//     );
//     std::io::stdout().flush().unwrap();
// }

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
    crate::testing::add_vars!(parser, "inch", "miles", "furlongs", "longer");
    //let mut result = parser.step();

    // loop {
    //     match parser.step() {
    //         ParserResult::MatchedLine => print(&parser),
    //         ParserResult::FailedLine => println!("   parse failed"),
    //         ParserResult::NoInput => break,
    //         _ => {}
    //     }
    // }

    loop {
        match parser.step() {
            ParserResult::NoInput => break,
            state => println!(
                "assert_step!(parser, {:?}, \"{}\", \"{}\");",
                state,
                parser.get_state(),
                std::str::from_utf8(parser.get_word()).unwrap()
            ),
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
    let _ = input.read(&mut [0u8]).unwrap();
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

//eq eleven num cabbagehead h h
//eq twelve and eleven num i h h h
//eq nice mu and eleven twelve h num dad h h h
