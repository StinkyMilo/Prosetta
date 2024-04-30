use super::*;

type BuildInSetUp = fn(num: u16, index: usize) -> MatchResult;

#[derive(Debug)]
pub struct BuiltinData {
    pub names: &'static [&'static [u8]],
    pub func: BuildInSetUp,
    pub is_expr: bool,
}

const EXPR_COMS: [&'static [u8]; 5] = [
    "num".as_bytes(),
    "mu".as_bytes(),
    "and".as_bytes(),
    "sub".as_bytes(),
    "lit".as_bytes(),
];
const STAT_COMS: [&'static [u8]; 3] = ["eq".as_bytes(), "pi".as_bytes(), "li".as_bytes()];

pub const EXPR_DATA: BuiltinData = BuiltinData {
    names: &EXPR_COMS,
    func: setup_expr,
    is_expr: true,
};

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

pub const STAT_DATA: BuiltinData = BuiltinData {
    names: &STAT_COMS,
    func: setup_stat,
    is_expr: false,
};

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
