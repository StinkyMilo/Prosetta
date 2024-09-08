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
            MatchResult::Matched(rest.pos, false)
        } else {
            // word is not a number
            MatchResult::Failed
        }
    }

    fn step_match(
        &mut self,
        _env: &mut Environment,
        _child_index: Option<usize>,
        _word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        // has no child to match - fn should never be called
        unreachable!()
    }

    fn get_name(&self) -> &'static str {
        "NumLit"
    }

    fn do_replace(&self) -> bool {
        false
    }
}

impl LiteralNumState {
    pub fn new() -> Self {
        Self {}
    }
}

fn get_number(word: &[u8]) -> Option<i64> {
    get_number_word(word).or_else(|| get_number_literal(word))
}

pub fn get_number_literal(mut word: &[u8]) -> Option<i64> {
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

pub fn get_number_word(word: &[u8]) -> Option<i64> {
    Some(match &*word.to_ascii_lowercase() {
        b"zero" => 0,
        b"one" => 1,
        b"two" => 2,
        b"three" => 3,
        b"four" => 4,
        b"five" => 5,
        b"six" => 6,
        b"seven" => 7,
        b"eight" => 8,
        b"nine" => 9,
        b"ten" => 10,
        _ => return None,
    })
}
