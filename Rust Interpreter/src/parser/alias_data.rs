use std::collections::HashSet;

use super::*;

const BASE_EXPR_ALIASES: &[&'static [u8]] = &[
    b"int", b"lit", // number makers
    b"add", b"sub", b"tim", b"ide", b"mod", b"log", b"exp", // number operators
    b"les", b"mor", b"als", b"oth", b"par", b"inv", // boolean operators
];

const LIST_EXPR_ALIASES: &[&'static [u8]] = &[b"lis", b"fin", b"ind", b"cou"];
const GRAPH_EXPR_ALIASES: &[&'static [u8]] = &[b"col"];
const FRAME_EXPR_ALIASES: &[&'static [u8]] = &[b"fra"];

const BASE_STAT_ALIASES: &[&'static [u8]] = &[b"was", b"pri", b"whe", b"whi", b"els", b"not"];

const LIST_STAT_ALIASES: &[&'static [u8]] = &[b"fre", b"del", b"app", b"rep"];
const FUNC_STAT_ALIASES: &[&'static [u8]] = &[b"fun", b"ret"];
const GRAPH_STAT_ALIASES: &[&'static [u8]] = &[
    b"arc", b"rec", //shapes
    b"mov", b"tur", b"lin", b"bez", //turtle
    b"sto", b"fil", b"pen", // shape modifiers
];

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
            b"col" => get_state!(color::ColorState::new()),
            b"lis" => get_state!(list::ListState::new()),
            b"fin" => get_state!(find::FindState::new()),
            b"ind" => get_state!(index::IndexState::new()),
            b"cou" => get_state!(len::LengthState::new()),
            b"fra" => get_state!(frame::FrameState::new()),
            _ => unreachable!("Got unknown alias {}", std::str::from_utf8(alias).unwrap()),
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
            b"bez" => get_state!(bezier::BezierState::new()),
            b"was" => get_state!(assign::AssignState::new()),
            b"rec" => get_state!(rect::RectState::new()),
            b"pri" => get_state!(print::PrintState::new()),
            b"whe" => get_state!(if_stat::IfState::new()),
            b"whi" => get_state!(while_stat::WhileState::new()),
            b"els" => get_state!(else_stat::ElseState::new()),
            b"sto" => get_state!(stroke::StrokeState::new()),
            b"fil" => get_state!(fill::FillState::new()),
            b"mov" => get_state!(move_to::MoveToState::new()),
            b"pen" => get_state!(line_width::LineWidthState::new()),
            b"tur" => get_state!(rotate::RotateState::new()),
            b"fun" => get_state!(function::FunctionState::new()),
            b"ret" => get_state!(return_stat::ReturnState::new()),
            b"app" => get_state!(append::AppendState::new()),
            b"del" => get_state!(delete::DeleteState::new()),
            b"rep" => get_state!(replace::ReplaceState::new()),
            b"fre" => get_state!(foreach::ForEachState::new()),
            b"not" => get_state!(not::NotState::new()),
            _ => unreachable!("Got unknown alias {}", std::str::from_utf8(alias).unwrap()),
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
        default_continue: false,
        state_name: "NoneStat",
    };
    pub const STAT_CONT: StaticAliasData = StaticAliasData {
        aliases: |data| &data.stat,
        func: get_stat_state,
        is_expr: false,
        default_continue: true,
        state_name: "NoneStatCont",
    };
}

impl AliasData {
    pub fn new(imports: &mut dyn Iterator<Item = &Import>) -> Self {
        let mut added = HashSet::new();
        let mut expr_vec = Vec::from(BASE_EXPR_ALIASES);
        let mut stat_vec = Vec::from(BASE_STAT_ALIASES);

        for import in imports {
            if !added.contains(import) {
                added.insert(*import);
                let (exprs_aliases, stat_aliases) = Self::get_aliases(*import);
                expr_vec.extend_from_slice(&exprs_aliases);
                stat_vec.extend_from_slice(&stat_aliases);
            }
        }

        Self {
            expr: expr_vec,
            stat: stat_vec,
        }
    }

    pub fn all() -> Self {
        Self::new(&mut Import::get_all().into_iter().map(|e| &e.0))
    }

    pub fn none() -> Self {
        Self::new(&mut std::iter::empty())
    }

    fn get_aliases(import: Import) -> (&'static [&'static [u8]], &'static [&'static [u8]]) {
        match import {
            Import::List => (LIST_EXPR_ALIASES, LIST_STAT_ALIASES),
            Import::Func => (&[], FUNC_STAT_ALIASES),
            Import::Graph => (GRAPH_EXPR_ALIASES, GRAPH_STAT_ALIASES),
            Import::Frame => (FRAME_EXPR_ALIASES, &[]),
        }
    }
}
