use super::*;
use num_literal::get_number;

#[derive(Debug, PartialEq)]
pub enum VarOrInt {
    Var(SubStrData),
    Int(i64),
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

            let (end, single_value) = if is_mandatory_close(word) {
                (End::from_slice(&word, env.global_index), Some(0))
            } else {
                (End::none(), None)
            };

            *env.expr = Expr::MultiLitNum {
                str_start: word.pos + env.global_index,
                locs,
                end,
                single_value,
                values: Vec::new(),
            };

            if single_value.is_some() {
                return MatchResult::Matched(word.pos, true);
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
                if let Some(var) = env.symbols.try_get_var(word, env.global_index) {
                    self.any_vars = true;
                    values.push(VarOrInt::Var(var));
                } else if let Some(num_value) = get_number(word.str) {
                    values.push(VarOrInt::Int(num_value % 10));
                } else {
                    values.push(VarOrInt::Int((word.len() as i64) % 10));
                }
                MatchResult::Continue(0)
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
        for i in values.into_iter() {
            if let VarOrInt::Int(i_val) = *i {
                if let Some(var) = val {
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
