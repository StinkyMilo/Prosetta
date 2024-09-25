use super::*;
use num_literal::get_number;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum VarOrInt {
    Var(Vec<u8>),
    Int(i64),
}

impl fmt::Display for VarOrInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VarOrInt::Var(name) => write!(f, "{}", String::from_utf8_lossy(name)),
            VarOrInt::Int(int_val) => write!(f, "{}", int_val),
        }
    }
}

#[derive(Debug)]
pub struct MultiLitNumState {
    first: bool,
    any_vars: bool,
}

impl ParseState for MultiLitNumState {
    fn step(&mut self, env: &mut Environment, word: &Slice, _rest: &Slice) -> MatchResult {
        if self.first {
            let locs = env.locs.take().unwrap_or_default();
            self.first = false;
            if is_mandatory_close(word) {
                *env.expr = Expr::MultiLitNum {
                    str_start: word.pos + env.global_index,
                    locs,
                    end: End::from_slice(&word, env.global_index),
                    single_value: Some(0),
                    values: Vec::new(),
                };
                return MatchResult::Matched(word.pos, true);
            } else {
                *env.expr = Expr::MultiLitNum {
                    str_start: word.pos + env.global_index,
                    locs,
                    end: End::none(),
                    single_value: None,
                    values: Vec::new(),
                };
            }
        }
        if let Expr::MultiLitNum {
            values,
            end,
            single_value,
            ..
        } = env.expr
        {
            if is_mandatory_close(word) {
                *end = End::from_slice(&word, env.global_index);
                if !self.any_vars {
                    *single_value = Self::get_final_value(values);
                }

                MatchResult::Matched(word.pos, true)
            } else {
                //let lower = word.str.to_ascii_lowercase();
                if let Some((_, var)) = env.vars.try_get_var(&word.str) {
                    self.any_vars = true;
                    values.push(VarOrInt::Var(var));
                } else if let Some(num_value) = get_number(word.str) {
                    values.push(VarOrInt::Int(num_value % 10));
                } else {
                    values.push(VarOrInt::Int((word.len() as i64) % 10));
                }
                MatchResult::Continue
            }
        } else {
            unreachable!()
        }
    }

    fn step_match(
        &mut self,
        _env: &mut Environment,
        _child_index: Option<usize>,
        _word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        unreachable!()
    }

    fn get_name(&self) -> &'static str {
        "MultiLitNum"
    }

    fn get_type(&self) -> StateType {
        StateType::Expr
    }
}

impl MultiLitNumState {
    pub fn new() -> Self {
        Self {
            first: true,
            any_vars: false,
        }
    }
    pub fn get_final_value(values: &Vec<VarOrInt>) -> Option<i64> {
        let mut val = Some(0i64);
        // let mut final_val_multiplier = Some(1i64);
        for i in values.into_iter() {
            if let VarOrInt::Int(i_val) = *i {
                if let Some(var) = val {
                    // let test: i64 = var;
                    val = var
                        .checked_mul(10_i64)
                        .and_then(|val| val.checked_add(i_val))
                } else {
                    //overflowed
                    return None;
                }
            } else {
                unreachable!()
            }
        }

        val
    }
}
