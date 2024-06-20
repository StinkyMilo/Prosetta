use std::{
    io::{self, Read, Write},
    mem,
};

use crate::parser::{Parser, ParserFlags, ParserResult};

#[path = "testing/testing.rs"]
mod testing;

mod playground;

mod commands;
mod parser;
mod writers;

mod error_messages;

use parser::ParserSource;
use writers::linq_like_writer;

fn print(parser: &Parser) {
    println!(
        "   parsed: {}",
        linq_like_writer::write_one(&parser.data.exprs, *parser.data.stat_starts.last().unwrap())
    );
    std::io::stdout().flush().unwrap();
}

fn main() {
    //playground::print_test();
    //return;

    println!("size of parser: {}", mem::size_of::<Parser>());

    let mut args: Vec<String> = std::env::args().skip(1).collect();

    for e in &mut args {
        e.make_ascii_lowercase();
    }

    args.sort();

    let flags = ParserFlags {
        not: args.binary_search(&"not".to_string()).is_ok(),
    };

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
    // let mut input =io::stdin().lock();
    //let stats = parser_state::parse(&mut input);
    //println!("{}",processing_writer::write(&stats))
    //let s:String = Default::default();

    let mut parser = Parser::new(ParserSource::from_stdin(), flags);

    //parser.vars.insert("inch".as_bytes().to_vec());
    //crate::testing::add_vars!(parser, "inch", "miles", "furlongs", "longer");
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

    let data = parser.into_data();
    let iter = data.source.get_iter();
    println!(
        "    text input:\n\"{}\"",
        std::str::from_utf8(iter.cloned().collect::<Vec<_>>().as_slice()).unwrap()
    );
    let iter = data.source.get_iter();
    //input.seek(SeekFrom::Start);
    //println!();
    //println!("== {:?}", parser.exprs.vec);
    println!(
        "   whole program:\n{}",
        linq_like_writer::write(&data.exprs, &data.stat_starts)
    );
    let mut lint = writers::syntax_lint::SyntaxLinter::<
        writers::syntax_renderers::wind_renderer::WindowsRenderer,
    >::new();
    lint.write(&data.exprs, &data.stat_starts, iter);
    println!(
        "   linted:\n{}",
        std::str::from_utf8(&lint.into_string()).unwrap()
    );
    // println!(
    //     "   java program:\n{}",
    //     processing_writer::write(&parser.exprs, &parser.stat_starts)
    // );
    let _ = io::stdin().read(&mut [0u8]).unwrap();
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
