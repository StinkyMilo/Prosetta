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
    pub aliases: AliasSelector,
    pub func: BuildInSetUp,
    pub is_expr: bool,
    pub default_continue: bool,
    pub state_name: &'static str,
}

#[derive(Debug)]
pub struct AliasData {
    pub expr: Vec<&'static [u8]>,
    pub stat: Vec<&'static [u8]>,
}
pub type AliasNames = Vec<&'static [u8]>;
type AliasSelector = fn(&AliasData) -> &AliasNames;

impl AliasData {
    pub const EXPR: BuiltinData = BuiltinData {
        aliases: |data| &data.expr,
        func: setup_expr,
        is_expr: true,
        default_continue: false,
        state_name: "NoneExpr",
    };
    pub const EXPR_CONT: BuiltinData = BuiltinData {
        aliases: |data| &data.expr,
        func: setup_expr,
        is_expr: true,
        default_continue: true,
        state_name: "NoneExprCont",
    };
    pub const STAT: BuiltinData = BuiltinData {
        aliases: |data| &data.stat,
        func: setup_stat,
        is_expr: false,
        default_continue: true,
        state_name: "NoneStat",
    };
}

impl AliasData {
    pub fn new(flags: ParserFlags) -> Self {
        let mut expr_vec = Vec::from(BASE_EXPR_ALIASES);

        if flags.not {
            expr_vec.push(NOT_ALIAS);
        }

        let stat_vec = Vec::from(STAT_ALIASES);

        AliasData {
            expr: expr_vec,
            stat: stat_vec,
        }
    }
}
