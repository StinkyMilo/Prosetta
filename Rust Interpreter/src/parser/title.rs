use super::*;
/// state for equals
#[derive(Debug)]
pub struct TitleState;
impl ParseState for TitleState {
    fn step(&mut self, _env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        // check by
        if word.str != b"by" {
            let slice = find_newline(&word).or_else(|| find_newline(&rest));
            if let Some(slice) = slice {
                let offset = slice.pos - word.pos;
                MatchResult::Continue(offset)
            } else {
                // did not find new line -- will never match
                MatchResult::Failed
            }
        } else {
            // let slice = find_newline(&word).or_else(|| find_newline(&rest));
            // MatchResult::Matched(close.pos, false);
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
        "Title"
    }

    fn get_type(&self) -> StateType {
        StateType::None
    }
}

impl TitleState {
    pub fn new() -> Self {
        Self
    }
}
