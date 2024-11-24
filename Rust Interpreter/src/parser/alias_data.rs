use std::collections::HashSet;

use super::*;

pub type StatTrigger = &'static [u8];
pub type ExprTrigger = (StatTrigger, Types);

const BASE_EXPR_ALIASES: &[ExprTrigger] = &[
    // number makers
    (b"int", Types::Number),
    (b"lit", Types::Number),
    // number operators
    (b"add", Types::Number),
    (b"sub", Types::Number),
    (b"tim", Types::Number),
    (b"ide", Types::Number),
    (b"mod", Types::Number),
    (b"log", Types::Number),
    (b"exp", Types::Number),
    (b"flo", Types::Number),
    // boolean operators
    (b"les", Types::Bool),
    (b"mor", Types::Bool),
    (b"als", Types::Bool),
    (b"oth", Types::Bool),
    (b"par", Types::Bool),
    (b"inv", Types::Bool),
];

const LIST_EXPR_ALIASES: &[ExprTrigger] = &[
    (b"lis", Types::List),
    (b"fin", Types::Any),
    (b"ind", Types::Any),
    (b"cou", Types::Number),
];
const GRAPH_EXPR_ALIASES: &[ExprTrigger] = &[(b"col", Types::Color)];
const FRAME_EXPR_ALIASES: &[ExprTrigger] = &[(b"fra", Types::Number)];
const TRIG_EXPR_ALIASES: &[ExprTrigger] = &[
    (b"sin", Types::Number),
    (b"cos", Types::Number),
    (b"tan", Types::Number),
];
const RAND_EXPR_ALIASES: &[ExprTrigger] = &[(b"ran", Types::Number)];

const BASE_STAT_ALIASES: &[StatTrigger] = &[b"was", b"pri", b"whe", b"whi", b"els", b"not"];

const LIST_STAT_ALIASES: &[StatTrigger] = &[b"fre", b"del", b"app", b"rep"];
const FUNC_STAT_ALIASES: &[StatTrigger] = &[b"fun", b"ret"];
const GRAPH_STAT_ALIASES: &[StatTrigger] = &[
    b"arc", b"rec", //shapes
    b"mov", b"tur", b"lin", b"bez", //turtle
    b"sto", b"fil", b"pen", // shape modifiers
];

const STAMP_STAT_ALIASES: &[StatTrigger] = &[b"sta", b"pol", b"tri", b"hea", b"roc", b"kir"];

///match alias to expr
fn get_expr_state(alias: &'static [u8]) -> Box<dyn ParseState> {
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
        b"flo" => get_state!(floor::FloorState::new()),

        b"lit" => get_state!(multi_lit_num::MultiLitNumState::new()),
        b"int" => get_state!(word_num::WordNumState::new()),
        b"col" => get_state!(color::ColorState::new()),
        b"lis" => get_state!(list::ListState::new()),
        b"fin" => get_state!(find::FindState::new()),
        b"ind" => get_state!(index::IndexState::new()),
        b"cou" => get_state!(len::LengthState::new()),
        b"fra" => get_state!(frame::FrameState::new()),

        b"sin" => get_state!(trig::TrigState::new_sin()),
        b"cos" => get_state!(trig::TrigState::new_cos()),
        b"tan" => get_state!(trig::TrigState::new_tan()),

        b"ran" => get_state!(rand::RandState::new()),

        _ => unreachable!("Got unknown alias {}", std::str::from_utf8(alias).unwrap()),
    }
}

/// match alias to stat
fn get_stat_state(alias: &'static [u8]) -> Box<dyn ParseState> {
    match alias {
        b"sta" => get_state!(stamps::StarState::new()),
        b"pol" => get_state!(stamps::PolygonState::new()),
        b"tri" => get_state!(stamps::TriangleState::new()),
        b"hea" => get_state!(stamps::HeartState::new()),
        b"roc" => get_state!(stamps::RoundRecState::new()),
        b"kir" => get_state!(stamps::KirbyState::new()),
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
    }
}
///function to get alias strings from AliasData
type AliasSelector = fn(&AliasData) -> &[StatTrigger];
///fn to get the continueWith state with the corresponding string
type AliasToState = fn(alias: &'static [u8]) -> Box<dyn ParseState>;
///static alias
#[derive(Debug)]
pub struct StaticAliasData {
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
    pub expr: Vec<ExprTrigger>,
    pub stat: Vec<StatTrigger>,
}

///static alias data
impl AliasData {
    pub const EXPR: StaticAliasData = StaticAliasData {
        func: get_expr_state,
        is_expr: true,
        default_continue: false,
        state_name: "NoneExpr",
    };
    pub const EXPR_CONT: StaticAliasData = StaticAliasData {
        func: get_expr_state,
        is_expr: true,
        default_continue: true,
        state_name: "NoneExprCont",
    };

    pub const STAT: StaticAliasData = StaticAliasData {
        func: get_stat_state,
        is_expr: false,
        default_continue: false,
        state_name: "NoneStat",
    };
    pub const STAT_CONT: StaticAliasData = StaticAliasData {
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

    fn get_aliases(import: Import) -> (&'static [ExprTrigger], &'static [StatTrigger]) {
        match import {
            Import::List => (LIST_EXPR_ALIASES, LIST_STAT_ALIASES),
            Import::Func => (&[], FUNC_STAT_ALIASES),
            Import::Graph => (GRAPH_EXPR_ALIASES, GRAPH_STAT_ALIASES),
            Import::Frame => (FRAME_EXPR_ALIASES, &[]),
            Import::Trig => (TRIG_EXPR_ALIASES, &[]),
            Import::Rand => (RAND_EXPR_ALIASES, &[]),
            Import::Stamp => (&[], STAMP_STAT_ALIASES),
        }
    }
}
