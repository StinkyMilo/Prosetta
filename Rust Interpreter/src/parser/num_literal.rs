use core::str;

use super::*;
#[derive(Debug)]

pub struct LiteralNumState {}

impl ParseState for LiteralNumState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        // try parse number
        let value = get_number(word.str);

        // if value exists - match
        if let Some(value) = value {
            *env.expr = Expr::LitNum {
                str_start: word.pos + env.global_index,
                str_length: word.len(),
                value,
            };
            MatchResult::Matched(rest.pos, ReturnType::Number, false)
        } else {
            // word is not a number
            MatchResult::Failed
        }
    }

    fn step_match(
        &mut self,
        _env: &mut Environment,
        _child_index: Option<(usize, ReturnType)>,
        _word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        // has no child to match - fn should never be called
        unreachable!()
    }

    fn get_name(&self) -> &'static str {
        "NumLit"
    }

    fn get_type(&self) -> StateType {
        StateType::Expr
    }
}

impl LiteralNumState {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn get_number(word: &[u8]) -> Option<i64> {
    get_number_word(word).or_else(|| get_number_literal(word))
}

fn get_number_literal(mut word: &[u8]) -> Option<i64> {
    let mut neg = 1i64;
    if word.starts_with(b"-") {
        neg = -1;
        word = &word[1..]
    }

    let number = || String::from_utf8(word[2..].to_vec()).ok();
    let num = {
        if word.starts_with(b"0x") {
            i64::from_str_radix(&number()?, 16)
        } else if word.starts_with(b"0o") {
            i64::from_str_radix(&number()?, 8)
        } else if word.starts_with(b"0b") {
            i64::from_str_radix(&number()?, 2)
        } else {
            let number = String::from_utf8(word.to_vec()).ok()?;
            i64::from_str_radix(&number, 10)
        }
    };
    num.ok().and_then(|num| Some(num * neg))
}

fn get_number_word(word: &[u8]) -> Option<i64> {
    let first_binding = word.to_ascii_lowercase();
    let temp_binding = String::from_utf8_lossy(&*&first_binding);
    let split_match: Vec<&str> = temp_binding.split("-").collect();
    let mut rev_values = Vec::new();
    //TODO: Enforce that each number must be less than the last, and you can't do like eleven-one
    for item in split_match {
        let next_value: i64 = match item {
            "and" => continue,
            "zero" => 0,
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            "ten" => 10,
            "eleven" => 11,
            "dozen" => 12,
            "twelve" => 12,
            "thirteen" => 13,
            "fourteen" => 14,
            "fifteen" => 15,
            "sixteen" => 16,
            "seventeen" => 17,
            "eighteen" => 18,
            "nineteen" => 19,
            "twenty" => 20,
            "score" => 20,
            "thirty" => 30,
            "forty" => 40,
            "fifty" => 50,
            "sixty" => 60,
            "seventy" => 70,
            "eighty" => 80,
            "ninety" => 90,
            "hundred" => 100,
            "thousand" => 1000,
            "million" => 1000000,
            "billion" => 1000000000,
            _ => return None,
        };
        rev_values.push(next_value);
    }
    if rev_values.len() == 0 {
        return None;
    }
    get_number_word_from_list(&rev_values, 0, rev_values.len())
}

fn get_number_word_from_list(values: &Vec<i64>, min_index: usize, max_index: usize) -> Option<i64> {
    let mut max_val = min_index;
    for i in min_index + 1..max_index {
        if values[i] > values[max_val] {
            max_val = i;
        }
    }
    if max_index - min_index > 1 {
        match values[max_val] {
            20 | 30 | 40 | 50 | 60 | 70 | 80 | 90 | 100 | 1000 | 10000 | 100000 | 1000000
            | 1000000000 => (),
            _ => return None,
        }
    }
    let left_side_val = if max_val == min_index {
        1
    } else {
        match get_number_word_from_list(values, min_index, max_val) {
            Some(value) => value,
            _ => return None,
        }
    };
    let right_side_val = if max_val == max_index - 1 {
        0
    } else {
        match get_number_word_from_list(values, max_val + 1, max_index) {
            Some(value) => value,
            _ => return None,
        }
    };
    Some(left_side_val * values[max_val] + right_side_val)
}
