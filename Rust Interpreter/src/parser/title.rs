use bstr::ByteSlice;

use super::*;
/// state for equals
#[derive(Debug)]
pub struct TitleState {
    parsing_names: bool,
    is_author_closed: bool,
    data: Title,
}
impl ParseState for TitleState {
    fn step(&mut self, env: &mut Environment, word: &Slice, rest: &Slice) -> MatchResult {
        if self.parsing_names {
            // names cannot be more than 255
            if word.len() > 255 {
                return MatchResult::Continue(0);
            }
            let separator = Self::is_separator(word.str);
            // is name
            if separator.close_count == 0 {
                if self.is_author_closed {
                    // first author already done
                    if self.data.authors.len() >= 1 {
                        self.add_imports(word.str, word.pos);
                    }
                    self.data
                        .authors
                        .push((word.str.to_vec(), word.pos, word.str.len()));
                } else {
                    // will always exist
                    let author = self.data.authors.last_mut().unwrap();
                    author.0.push(b' ');
                    author.0.extend_from_slice(&word.str);
                    author.2 = word.end() - author.1;
                    // second author started
                    if self.data.authors.len() >= 2 {
                        self.add_imports(word.str, word.pos);
                    }
                }
                self.is_author_closed = false;
                MatchResult::Continue(0)
                // is name close
            } else if separator.only_forced {
                self.is_author_closed = true;
                self.data.delim.push((word.pos, separator.close_length));

                MatchResult::Continue(0)
                // is total close
            } else {
                self.data.delim.push((word.pos, separator.close_length));
                let title = mem::replace(&mut self.data, Title::new());
                *env.expr = Expr::Title { data: title };
                MatchResult::Matched(word.pos, true)
            }
        } else if word.len() >= 2 && word.str.to_ascii_lowercase() == b"by" {
            self.data
                .title
                .extend_from_slice(&env.full_text[..word.pos].trim());
            self.data.by_start = word.pos;
            self.parsing_names = true;

            MatchResult::Continue(0)
        } else {
            let slice = find_newline(&word, 0).or_else(|| find_newline(&rest, 0));
            if let Some(newline) = slice {
                let offset = newline.pos - word.pos - 1;
                MatchResult::Continue(offset)
            // did not find another new line -- poem has ended -- will never match
            } else {
                MatchResult::Failed
            }
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
        Self {
            parsing_names: false,
            is_author_closed: true,
            data: Title::new(),
        }
    }

    ///returns and optinal length of the close
    fn is_separator(str: &[u8]) -> CloseData {
        if str.len() >= 3 && str == b"and" {
            CloseData {
                close_count: 1,
                close_length: 3,
                only_forced: true,
            }
        } else if str.len() >= 1 && str == b"&" {
            CloseData {
                close_count: 1,
                close_length: 1,
                only_forced: true,
            }
        } else {
            get_close_data(str)
        }
    }
    fn add_imports(&mut self, name: &[u8], index: usize) {
        let imports = Import::get_all();
        if let Some((offset, _, imp_index)) =
            parser_structs::try_get_best_val(name, &mut imports.iter().map(|e| e.1), &|_| true)
        {
            self.data.imports.push((
                imports[imp_index].0,
                offset as usize + index,
                imports[imp_index].1.len() as u8,
            ))
        }
    }
}
