// pub enum ParserVisuals{

// }

use std::time::SystemTime;

use crate::{
    parser::{ParsedData, Parser, ParserFlags, ParserResult, ParserSource},
    writers::{
        javascript_writer, lisp_like_writer,
        syntax_lint::SyntaxLinter,
        syntax_renderers::{line_renderer::LineRenderer, wind_renderer::WindowsRenderer},
    },
};

#[derive(Clone, Copy)]
pub struct RunnerFlags {
    pub assert_steps: bool,
    #[allow(dead_code)]
    pub input: bool,
    pub whole_program: bool,
    pub linted: bool,
    pub line_rendered: bool,
}

pub fn run_state(state: ParserResult, parser: &Parser, parser_flags: RunnerFlags, step_count: u64) {
    let count = if cfg!(feature = "speed") { 1000000 } else { 1 };

    if parser_flags.assert_steps {
        if step_count % count == 0 {
            let (word, index) = parser.get_last_word();
            let (stack, length) = parser.get_parser_stack();
            println!(
                "step:{}\nword:[{}:{}]\nreturn:[{:?}]\nstack:{}[{}]",
                step_count,
                std::str::from_utf8(word).unwrap(),
                index,
                state,
                length,
                stack
            );
        }
    }
}

pub fn run_after(data: ParsedData, parser_flags: RunnerFlags) {
    // if parser_flags.input {
    //     let iter = data.source.get_iter();
    //     println!(
    //         "    text input:\n\"{}\"",
    //         std::str::from_utf8(iter.cloned().collect::<Vec<_>>().as_slice()).unwrap()
    //     );
    // }
    if parser_flags.whole_program {
        println!(
            "   whole program:\n{}",
            lisp_like_writer::write(&data.exprs, &data.stat_starts)
        );
        println!(
            "   JavaScript output:\n{}",
            javascript_writer::write(&data.exprs, &data.stat_starts)
        );
    }
    if parser_flags.linted {
        let iter = data.source.get_iter();
        let mut lint = SyntaxLinter::<WindowsRenderer>::new();
        lint.write(&data.exprs, &data.stat_starts, iter);
        println!(
            "   linted:\n{}",
            std::str::from_utf8(&lint.into_data()).unwrap()
        );
    }
    if parser_flags.line_rendered {
        let iter = data.source.get_iter();
        let mut lint = SyntaxLinter::<LineRenderer>::new();
        lint.write(&data.exprs, &data.stat_starts, iter);
        println!("   line renderered:\n{:?}", lint.into_data());
    }
}

pub fn run_parser(parser_flags: ParserFlags, vis_flags: RunnerFlags, source: ParserSource) {
    // println!("Input text to be parsed:");
    let mut parser = Parser::new(source, parser_flags);
    let mut step_count = 0;
    let start = SystemTime::now();
    loop {
        match parser.step() {
            ParserResult::NoInput => break,
            state => run_state(state, &parser, vis_flags, step_count),
        }
        step_count += 1;
    }
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!(
        "took {} seconds with {} steps",
        duration.as_secs(),
        step_count
    );

    let data = parser.into_data();

    run_after(data, vis_flags);
}
