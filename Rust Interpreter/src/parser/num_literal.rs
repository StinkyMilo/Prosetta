use super::*;

#[derive(Debug)]

pub struct LiteralNumState {
    value: Option<i64>,
}

impl ParseState for LiteralNumState {
    fn step(&mut self, env: &mut Enviroment, word: &Slice, rest: &Slice) -> MatchResult {

        // if not checked - try
        if self.value.is_none() {
            self.value = get_number(word.str)
        }

        // check
        if let Some(value) = self.value {
            *env.expr = Expr::LitNum {
                locs:Vec::new(),
                str_start: word.pos + env.global_index,
                str_length: word.len(),
                value,
            };
            MatchResult::Matched(rest.pos)
        } else {
            // future words could be number names
            MatchResult::Continue
        }
    }

    fn step_match(
        &mut self,
        _env: &mut Enviroment,
        _did_child_match: bool,
        _word: &Slice,
        _rest: &Slice,
    ) -> MatchResult {
        // has no child to match - fn should never be called
        unimplemented!()
    }

    fn get_name(&self) -> &'static str {
        "NumLiteral"
    }

    fn do_replace(&self) -> bool {
        false
    }
}

impl LiteralNumState {
    pub fn new() -> Self {
        LiteralNumState { value: None }
    }
    pub fn check(&mut self, _env: &mut Enviroment, word: &Slice) -> bool {
        self.value = get_number(word.str);
        self.value.is_some()
    }
}

pub fn get_number(word: &[u8]) -> Option<i64> {
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
