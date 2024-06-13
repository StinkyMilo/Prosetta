use super::*;

macro_rules! get_state {
    ($state:expr) => {
        Box::new($state) as Box<dyn ParseState>
    };
}

type BuildInSetUp = fn(alias: &'static [u8], index: usize) -> MatchResult;

const BASE_EXPR_ALIASES: [&'static [u8]; 9] = [
    b"int", b"tim", b"add", b"sub", b"lit", b"ide", b"mod", b"log", b"exp",
];

const NOT_ALIAS: &'static [u8] = b"not";

const STAT_ALIASES: [&'static [u8]; 5] = [b"arc", b"lin", b"was", b"rec", b"pri"];

fn setup_expr(alias: &'static [u8], index: usize) -> MatchResult {
    MatchResult::ContinueWith(
        index,
        match alias {
            b"add" => get_state!(operator::OperatorState::new_add()),
            b"sub" => get_state!(operator::OperatorState::new_sub()),
            b"tim" => get_state!(operator::OperatorState::new_mult()),
            b"ide" => get_state!(operator::OperatorState::new_div()),
            b"mod" => get_state!(operator::OperatorState::new_mod()),
            b"exp" => get_state!(operator::OperatorState::new_exp()),
            b"log" => get_state!(operator::OperatorState::new_log()),

            b"lit" => get_state!(num_lit::LitNumState::new()),
            b"int" => get_state!(word_num::WordNumState::new()),
            b"not" => get_state!(not::NotState::new()),
            _ => panic!("Got unknown alias {}", std::str::from_utf8(alias).unwrap()),
        },
    )
}

fn setup_stat(alias: &'static [u8], index: usize) -> MatchResult {
    MatchResult::ContinueWith(
        index,
        match alias {
            b"arc" => get_state!(circle::CircleState::new()),
            b"lin" => get_state!(line::LineState::new()),
            b"was" => get_state!(set::EqState::new()),
            b"rec" => get_state!(rect::RectState::new()),
            b"pri" => get_state!(print::PrintState::new()),
            _ => panic!("Got unknown alias {}", std::str::from_utf8(alias).unwrap()),
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
