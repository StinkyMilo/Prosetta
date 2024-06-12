use super::*;

type BuildInSetUp = fn(num: u16, index: usize) -> MatchResult;

const BASE_EXPR_ALIASES: [&'static [u8]; 7] =
    [b"int", b"tim", b"add", b"sub", b"lit", b"ide", b"mod"];

const NOT_ALIAS: &'static [u8] = b"not";

const STAT_ALIASES: [&'static [u8]; 5] = [b"arc", b"lin", b"was", b"rec", b"pri"];

fn setup_expr(num: u16, index: usize) -> MatchResult {
    MatchResult::ContinueWith(
        index,
        match num {
            0 => Box::new(num::NumState::new()) as Box<dyn ParseState>,
            1 => Box::new(add_mult::BiFuncState::new_mult()) as Box<dyn ParseState>,
            2 => Box::new(add_mult::BiFuncState::new_add()) as Box<dyn ParseState>,
            3 => Box::new(add_mult::BiFuncState::new_sub()) as Box<dyn ParseState>,
            4 => Box::new(num_lit::LitNumState::new()) as Box<dyn ParseState>,
            _ => unimplemented!(),
        },
    )
}

fn setup_stat(num: u16, index: usize) -> MatchResult {
    MatchResult::ContinueWith(
        index,
        match num {
            0 => Box::new(eq::EqState::new()) as Box<dyn ParseState>,
            1 => Box::new(circle::CircleState::new()),
            2 => Box::new(line::LineState::new()),
            _ => unimplemented!(),
        },
    )
}

#[derive(Debug)]
pub struct BuiltinData {
    pub names: Vec<&'static [u8]>,
    pub func: BuildInSetUp,
    pub is_expr: bool,
    pub default_continue: bool,
    pub state_name: &'static str,
}

#[derive(Debug)]
pub struct AliasData {
    pub expr: BuiltinData,
    pub expr_cont: BuiltinData,
    pub stat: BuiltinData,
}

impl AliasData {
    pub fn new(flags: ParserFlags) -> Self {
        let mut expr_vec = Vec::from(BASE_EXPR_ALIASES);

        if flags.not {
            expr_vec.push(NOT_ALIAS);
        }

        let expr = BuiltinData {
            names: expr_vec,
            func: setup_expr,
            is_expr: true,
            default_continue: false,
            state_name: "NoneExpr",
        };

        let expr_cont = BuiltinData {
            names: expr_vec,
            func: setup_expr,
            is_expr: true,
            default_continue: true,
            state_name: "NoneExprCont",
        };

        let stat = BuiltinData {
            names: Vec::from(BASE_EXPR_ALIASES),
            func: setup_stat,
            is_expr: false,
            default_continue: true,
            state_name: "NoneStat",
        };

        AliasData {
            expr,
            expr_cont,
            stat,
        }
    }
}
