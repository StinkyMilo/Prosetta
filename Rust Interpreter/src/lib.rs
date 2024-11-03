#![cfg(feature = "wasm")]

use crate::parser::{Parser, ParserFlags};

// #[path = "testing/testing.rs"]
// mod testing;

mod parser_runner;

mod commands;
mod parser;
mod writers;

use crate::parser::{ParsedData, ParserResult};
use parser::ParserSource;

use crate::writers::javascript_writer;
use crate::writers::word_trigger_writer;
use crate::writers::syntax_lint::SyntaxLinter;
use crate::writers::syntax_renderers::{
    html_renderer::HTMLRenderer,
    line_renderer::{Highlight, LineRenderer},
};
use cap::Cap;
use std::alloc;

// // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// // allocator.
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// fn setup() {
//     console_error_panic_hook::set_once();
// }
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());

#[wasm_bindgen]
pub fn get_heap_size() -> usize {
    ALLOCATOR.allocated()
}

#[wasm_bindgen]
pub struct ParserRunner;

#[wasm_bindgen]
pub struct ParserRunnerData {
    data: ParsedData<'static>,
}

#[wasm_bindgen]
impl ParserRunner {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self
    }

    pub fn run_to_completion(&mut self, source: &str) -> ParserRunnerData {
        let mut parser = Parser::new(
            ParserSource::from_string(source.as_bytes().to_vec()),
            ParserFlags { title: true },
        );

        loop {
            if parser.step() == ParserResult::NoInput {
                break;
            };
        }

        ParserRunnerData {
            data: parser.into_data(),
        }
    }
}

#[wasm_bindgen]
impl ParserRunnerData {
    pub fn get_javascript(&self) -> String {
        javascript_writer::write(&self.data.exprs, &self.data.stat_starts)
    }
    pub fn get_html(&self) -> String {
        let iter = self.data.source.get_iter();
        let mut lint = SyntaxLinter::<HTMLRenderer>::new();
        lint.write(&self.data.exprs, &self.data.stat_starts, iter);
        String::from_utf8_lossy(&lint.into_data()).to_string()
    }
    pub fn get_highlights(&self) -> Vec<Highlight> {
        let iter = self.data.source.get_iter();
        let mut lint = SyntaxLinter::<LineRenderer>::new();
        lint.write(&self.data.exprs, &self.data.stat_starts, iter);
        lint.into_data()
    }
    pub fn get_triggers(&self) -> String {
        word_trigger_writer::write(&self.data.trigger_word_data)
    }
}

//wasm-pack build . -F wasm
