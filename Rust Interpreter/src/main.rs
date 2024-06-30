use std::{
    io::{self, Read},
    mem,
};

use crate::parser::{Parser, ParserFlags};

#[path = "testing/testing.rs"]
mod testing;
//mod playground;

mod parser_runner;

mod commands;
mod parser;
mod writers;

use parser::ParserSource;
use parser_runner::{run_parser, RunnerFlags};


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wasm")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// fn setup() {
//     console_error_panic_hook::set_once();
// }
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn wasm_run_parser(str: String) -> Vec<String> {
    //setup();
    let mut parser = Parser::new(
        ParserSource::from_string(str.into_bytes()),
        ParserFlags { not: true },
    );

    let mut ret = Vec::new();

    loop {
        match parser.step() {
            ParserResult::NoInput => break,
            ParserResult::Matched
            | ParserResult::MatchedLine
            | ParserResult::Failed
            | ParserResult::FailedLine => {
                let data = &parser.data;
                let iter = data.source.get_iter();
                let mut lint = SyntaxLinter::<HTMLRenderer>::new();
                lint.write(&data.exprs, &data.stat_starts, iter);
                ret.push(String::from_utf8(lint.into_string()).unwrap());
            }
            _ => {}
        };
    }

    ret
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

    let parser_flags = ParserFlags {
        not: true, //args.binary_search(&"not".to_string()).is_ok(),
    };

    let vis_flags: RunnerFlags = RunnerFlags {
        assert_steps: true,
        input: true,
        whole_program: true,
        linted: true,
    };

    run_parser(parser_flags, vis_flags, ParserSource::from_stdin());

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

// println!(
//     "name of eq: {}",
//     format!("{:#?}",(&commands::Expr::Eq {
//         locs: vec![0, 1],
//         name_start: 7,
//         name: "inch".as_bytes().to_vec(),
//         value_index: 1
//     }))
// );
//let mut buf= Vec::new();
//testing::test_ast1();
// let mut input =io::stdin().lock();
//let stats = parser_state::parse(&mut input);
//println!("{}",processing_writer::write(&stats))
//let s:String = Default::default();

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

// loop {
//     match parser.step() {
//         ParserResult::NoInput => break,
//         state => println!(
//             "assert_step!(parser, {:?}, \"{}\", \"{}\");",
//             state,
//             parser.get_state(),
//             std::str::from_utf8(parser.get_word()).unwrap()
//         ),
//     }
// }

// let data = parser.into_data();
// let iter = data.source.get_iter();
// println!(
//     "    text input:\n\"{}\"",
//     std::str::from_utf8(iter.cloned().collect::<Vec<_>>().as_slice()).unwrap()
// );
// let iter = data.source.get_iter();
// //input.seek(SeekFrom::Start);
// //println!();
// //println!("== {:?}", parser.exprs.vec);
// println!(
//     "   whole program:\n{}",
//     linq_like_writer::write(&data.exprs, &data.stat_starts)
// );
// let mut lint = writers::syntax_lint::SyntaxLinter::<
//     writers::syntax_renderers::wind_renderer::WindowsRenderer,
// >::new();
// lint.write(&data.exprs, &data.stat_starts, iter);
// println!(
//     "   linted:\n{}",
//     std::str::from_utf8(&lint.into_string()).unwrap()
// );
// println!(
//     "   java program:\n{}",
//     processing_writer::write(&parser.exprs, &parser.stat_starts)
// );
