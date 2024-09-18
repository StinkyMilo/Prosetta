#![cfg(feature = "wasm")]

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

use crate::parser::ParserResult;
use parser::ParserSource;
use parser_runner::{run_parser, RunnerFlags};

use crate::writers::syntax_lint::SyntaxLinter;
use crate::writers::syntax_renderers::html_renderer::HTMLRenderer;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// fn setup() {
//     console_error_panic_hook::set_once();
// }
use wasm_bindgen::prelude::*;

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
