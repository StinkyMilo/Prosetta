use super::*;

const BASE_EXPR_ALIASES: [&'static [u8]; 15] = [
    b"int", b"tim", b"add", b"sub", b"lit", b"ide", b"mod", b"log", b"exp", b"les", b"mor", b"als", b"oth", b"par", b"inv"
];

const NOT_ALIAS: &'static [u8] = b"not";

const STAT_ALIASES: [&'static [u8]; 11] = [b"arc", b"lin", b"was", b"rec", b"pri", b"whe", b"whi", b"els", b"sto", b"fil", b"col"];

///match alias to expr
fn get_expr_state(alias: &'static [u8], index: usize) -> MatchResult {
    MatchResult::ContinueWith(
        index,
        match alias {
            b"add" => get_state!(operator::OperatorState::new_add()),
            b"sub" => get_state!(operator::OperatorState::new_sub()),
            b"tim" => get_state!(operator::OperatorState::new_mult()),
            b"ide" => get_state!(operator::OperatorState::new_div()),
            b"mod" => get_state!(operator::OperatorState::new_mod()),
            b"log" => get_state!(operator::OperatorState::new_log()),
            b"exp" => get_state!(operator::OperatorState::new_exp()),
            b"les" => get_state!(operator::OperatorState::new_less_than()),
            b"mor" => get_state!(operator::OperatorState::new_greater_than()),
            b"als" => get_state!(operator::OperatorState::new_and()),
            b"oth" => get_state!(operator::OperatorState::new_or()),
            b"par" => get_state!(operator::OperatorState::new_equals()),
            b"inv" => get_state!(operator::OperatorState::new_not()),

            b"lit" => get_state!(multi_lit_num::MultiLitNumState::new()),
            b"int" => get_state!(word_num::WordNumState::new()),
            b"not" => get_state!(not::NotState::new()),
            _ => panic!("Got unknown alias {}", std::str::from_utf8(alias).unwrap()),
        },
    )
}

/// match alias to stat
fn get_stat_state(alias: &'static [u8], index: usize) -> MatchResult {
    MatchResult::ContinueWith(
        index,
        match alias {
            b"arc" => get_state!(circle::CircleState::new()),
            b"lin" => get_state!(line::LineState::new()),
            b"was" => get_state!(assign::AssignState::new()),
            b"rec" => get_state!(rect::RectState::new()),
            b"pri" => get_state!(print::PrintState::new()),
            b"whe" => get_state!(ifstatement::IfState::new()),
            b"whi" => get_state!(whilestatement::WhileState::new()),
            b"els" => get_state!(elsestatement::ElseState::new()),
            b"sto" => get_state!(stroke::StrokeState::new()),
            b"fil" => get_state!(fill::FillState::new()),
            b"col" => get_state!(color::ColorState::new()),
            _ => panic!("Got unknown alias {}", std::str::from_utf8(alias).unwrap()),
        },
    )
}
///A vector of alias strings
pub type AliasNames = Vec<&'static [u8]>;
///function to get alias strings from AliasData
type AliasSelector = fn(&AliasData) -> &AliasNames;
///fn to get the continueWith state with the corresponding string
type AliasToState = fn(alias: &'static [u8], index: usize) -> MatchResult;
///static alias
#[derive(Debug)]
pub struct StaticAliasData {
    ///function to get alias strings from AliasData
    pub aliases: AliasSelector,
    ///function to get the matching continueWith state
    pub func: AliasToState,
    ///is this a expr alias data
    pub is_expr: bool,
    ///should this continue on failure of a word
    pub default_continue: bool,
    ///the name of the corresponding state
    pub state_name: &'static str,
}

///holds lists of all alias strings
#[derive(Debug)]
pub struct AliasData {
    pub expr: AliasNames,
    pub stat: AliasNames,
}

///static alias data
impl AliasData {
    pub const EXPR: StaticAliasData = StaticAliasData {
        aliases: |data| &data.expr,
        func: get_expr_state,
        is_expr: true,
        default_continue: false,
        state_name: "NoneExpr",
    };
    pub const EXPR_CONT: StaticAliasData = StaticAliasData {
        aliases: |data| &data.expr,
        func: get_expr_state,
        is_expr: true,
        default_continue: true,
        state_name: "NoneExprCont",
    };
    pub const STAT: StaticAliasData = StaticAliasData {
        aliases: |data| &data.stat,
        func: get_stat_state,
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

        Self {
            expr: expr_vec,
            stat: stat_vec,
        }
    }
}
