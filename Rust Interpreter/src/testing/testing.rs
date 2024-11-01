#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]

#[cfg(test)]
pub mod test_lib {

    use crate::commands::{Expr, ExprArena};

    use crate::parser::{Parser, *};
    use crate::writers::lisp_like_writer;
    use alias_data::AliasData;
    use std::collections::HashSet;

    pub fn assert_step_inner(
        parser: &mut Parser,
        exp_result: ParserResult,
        exp_state: &str,
        exp_word: &str,
        file: &str,
        line: u32,
    ) {
        let result = parser.step();
        let state = parser.get_last_state_name();
        let word = std::str::from_utf8(parser.get_last_word().0).unwrap();
        let message = &format!(
            "result was {:?}(\"{}\",\"{}\"). expected {:?}(\"{}\",\"{}\") at {}:{}",
            result, state, word, exp_result, exp_state, exp_word, file, line
        );

        assert_eq!(result, exp_result, "{}", message);
        assert_eq!(state, exp_state, "{}", message);
        assert_eq!(word, exp_word, "{}", message);
    }

    pub fn new_slice(str: &str, start: usize) -> Slice {
        Slice {
            str: str.as_ref(),
            pos: start,
        }
    }

    pub fn new_sub_slice(str: &str, start: usize) -> Slice {
        let bytes: &[u8] = str.as_ref();
        Slice {
            str: &bytes[start..],
            pos: start,
        }
    }

    // pub fn new_env<'a>(
    //     vars: &'a mut VarSet,
    //     expr: &'a mut Expr,
    //     locs: Option<Vec<usize>>,
    //     aliases: &'a AliasData,
    //     last_stat:Option<&'a mut Expr>
    // ) -> Environment<'a> {
    //     Environment {
    //         vars,
    //         expr,
    //         last_stat,
    //         locs,
    //         global_index: 0,
    //         aliases,
    //     }
    // }

    pub fn assert_result(parser: &mut Parser) -> ParserResult {
        loop {
            let result = parser.step();
            if result.is_end() {
                return result;
            }
        }
    }
    pub fn run_to_completion(parser: &mut Parser) -> ParserResult {
        loop {
            let result = parser.step();
            if matches!(result, ParserResult::NoInput) {
                return result;
            }
        }
    }

    pub fn run_parser(input: Vec<u8>, title: bool) -> ParsedData<'static> {
        let text = input.to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { title });
        run_to_completion(&mut parser);
        parser.into_data()
    }

    pub fn get_lisp(data: &ParsedData) -> String {
        lisp_like_writer::write(&data.exprs, &data.stat_starts)
    }
}

// use crate::testing::test_lib::*;

macro_rules! add_vars {
    ($parser:ident, $var:expr) => {
        $parser.vars.insert($var.as_bytes().to_vec());
    };
    ($parser:ident, $var:expr, $($vars:expr),+) => {
        $crate::testing::add_vars! ($parser, $var );
        $crate::testing::add_vars! ($parser, $($vars), + );
    };
}
pub(crate) use add_vars;

macro_rules! assert_step {
    ($parser:ident,$step_result:ident,$state:expr,$word:expr) => {
        $crate::testing::test_lib::assert_step_inner(
            &mut $parser,
            ParserResult::$step_result,
            $state,
            $word,
            file!(),
            line!(),
        );
    };
}

pub(crate) use assert_step;

macro_rules! run_parser {
    ($input:expr) => {{
        $crate::testing::test_lib::run_parser($input.to_vec(), false)
    }};
    ($input:expr, $title:expr) => {{
        $crate::testing::test_lib::run_parser($input.to_vec(), $title)
    }};
}

pub(crate) use run_parser;

macro_rules! check_lisp {
    ($data:expr,$result:expr) => {{
        assert_eq!($crate::testing::test_lib::get_lisp(&$data), $result);
    }};
}

pub(crate) use check_lisp;
