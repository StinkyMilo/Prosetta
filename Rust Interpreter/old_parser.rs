#[path = "parsing_tests.rs"]
mod tests;

use crate::commands::*;
use std::{
    array,
    collections::HashSet,
    fmt,
    io::BufRead, iter,
};

type MatchResult<T> = Option<(usize, T)>;
type VarSet = HashSet<Vec<u8>>;
type ParseFn<T> = fn(&VarSet, &Slice<'_>, Vec<usize>) -> MatchResult<T>;
type BuildInParseFn<T> = fn(num: u16, &VarSet, &Slice<'_>, locs: Vec<usize>) -> MatchResult<T>;

#[derive(PartialEq)]
struct Slice<'a> {
    str: &'a [u8],
    pos: usize,
}

impl<'a> fmt::Debug for Slice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Slice")
            .field("str", &String::from_utf8_lossy(&self.str))
            .field("pos", &self.pos)
            .finish()
    }
}

impl<'a> Slice<'a> {
    fn len(&self) -> usize {
        self.str.len()
    }
    fn end(&self) -> usize {
        self.pos + self.str.len()
    }
    fn offset(&self,offset:usize)->Slice{
        Slice{
            str:&self.str[offset..],
            pos:self.pos+offset
        }
    }
}

// struct Env {}
// impl Index<usize> for Slice<'_>{
//     fn index(&self, nucleotide: Nucleotide) -> &Self::Output {
//         match nucleotide {
//             Nucleotide::A => &self.a,
//             Nucleotide::C => &self.c,
//             Nucleotide::G => &self.g,
//             Nucleotide::T => &self.t,
//         }
//     }
// }
/*
    usize: last location
    T:type
*/
//type MatchResult<T> = Option<(usize, T)>;
//type Matched<T> = Some<(usize, T)>;

// pub enum MatchResult<T> {
//     Matched(usize, T),
//     Failed,
// }

// impl<T> MatchResult<T> {
//     pub const fn is_ok(&self) -> bool {
//         matches!(*self, MatchResult::Matched(_, _))
//     }
// }

const EXPR_COMS: [&[u8]; 3] = ["num".as_bytes(), "mu".as_bytes(), "and".as_bytes()];
const EXPR_FN: [ParseFn<Expr>; 3] = [match_num, match_mult, match_add];
fn match_expr_fn(num: u16, vars: &VarSet, slice: &Slice, locs: Vec<usize>) -> MatchResult<Expr> {
    EXPR_FN[num as usize](vars, slice, locs)
}
const STAT_COMS: [&[u8]; 3] = ["eq".as_bytes(), "pi".as_bytes(), "li".as_bytes()];
const STAT_FN: [ParseFn<Stat>; 3] = [match_eq, match_circle, match_line];
fn match_stat_fn(num: u16, vars: &VarSet, slice: &Slice, locs: Vec<usize>) -> MatchResult<Stat> {
    STAT_FN[num as usize](vars, slice, locs)
}

// returns (next word, rest of slice)
fn get_next_word<'a>(slice: &'a Slice<'a>, mut start: usize) -> Option<(Slice<'a>, Slice<'a>)> {
    // find start of word
    while start < slice.len() && !slice.str[start].is_ascii_alphanumeric() {
        start += 1;
    }

    // find end of word
    let mut end = start;
    while end < slice.len() && slice.str[end].is_ascii_alphanumeric() {
        end += 1;
    }

    (start < slice.len()).then(|| {
        (
            Slice {
                str: &slice.str[start..end],
                pos: slice.pos + start,
            },
            Slice {
                str: &slice.str[end..],
                pos: slice.pos + end,
            },
        )
    })
}

// returns the rest after the end of the word
fn find_word_end<'a>(slice: &'a Slice<'a>, start: usize) -> Slice<'a> {
    // find end of word
    let mut end = start;
    while end < slice.len() && slice.str[end].is_ascii_alphanumeric() {
        end += 1;
    }
    //let test = end < slice.len();
    Slice {
        str: &slice.str[end..],
        pos: slice.pos + end,
    }
}

// returns the rest after finding the end of an h word 
fn find_h_close<'a>(slice: &'a Slice<'a>, start: usize) -> Option<Slice<'_>> {
    // find h
    let mut end = start;
    while end < slice.len() && slice.str[end] != b'h' {
        end += 1;
    }
    let test = end < slice.len();
    // find end of h word
    test.then(||find_word_end(slice, end))
}

fn match_expr(vars: &VarSet, slice: &Slice) -> MatchResult<Expr> {
    let var = match_var(vars, slice);
    if var.is_some() {
        return var;
    }
    match_built_in(&EXPR_COMS, match_expr_fn, vars, slice)
}

fn match_built_in<T>(
    coms: &[&[u8]],
    callback: BuildInParseFn<T>,
    vars: &VarSet,
    slice: &Slice,
) -> MatchResult<T> {
    debug_assert!(coms.len() < u16::MAX as usize);

    let mut progress = vec![0u8; coms.len()];
    let mut com_match = vec![Some(vec![]); coms.len()];
    let mut matched = 0u16;

    for offset in 0..slice.len() {
        // end of word reached
        // reset progresses
        if slice.str[offset] == b' ' {
            progress = vec![0u8; coms.len()];
            com_match = vec![Some(vec![]); coms.len()];
        }

        // does letter match any commands
        for i in 0..coms.len() {
            // does letter match
            if progress[i] < coms[i].len() as u8
                && slice.str[offset].to_ascii_lowercase() == coms[i][progress[i] as usize]
            {
                progress[i] += 1;
                com_match[i].as_mut().unwrap().push(slice.pos+offset);
                if progress[i] == coms[i].len() as u8 {
                    matched +=1;
                }
            }
        }

        // try match
        while matched!=0 {
            matched -= 1;
            let mut min_size = usize::MAX;
            let mut min_locations = usize::MAX;
            let mut min_index = u16::MAX;
            for j in 0..coms.len() {
                // has finished matching
                if progress[j] == coms[j].len() as u8 {
                    let matching_locs = com_match[j].as_ref().unwrap();

                    let size = matching_locs.last().unwrap() - matching_locs[0];
                    let location_sum: usize = matching_locs.iter().sum();

                    // is best match
                    if size < min_size || (size == min_size && location_sum < min_locations) {
                        min_index = j as u16;
                        min_size = size;
                        min_locations = location_sum;
                    }
                }
            }
            let word_end = find_word_end(slice, offset);

            if word_end.len() != 0 {
                //do not try match again
                progress[min_index as usize] += 1;

                // try match rest
                let result = callback(
                    min_index,
                    vars,
                    &word_end,
                    com_match[min_index as usize].take().unwrap(),
                );

                //matched full statement
                if result.is_some() {
                    return result;
                }
            }
        }
    }
    None
}

fn match_var(vars: &VarSet, slice: &Slice) -> MatchResult<Expr> {
    let name = get_next_word(slice, 0)?;

    if vars.contains(name.0.str) {
        Some((
            name.1.pos,
            Expr::Var {
                name_start: name.0.pos,
                name: name.0.str.to_owned(),
            },
        ))
    } else {
        None
    }
}

fn match_num(vars: &VarSet, slice: &Slice, locs: Vec<usize>) -> MatchResult<Expr> {
    let words = get_next_word(slice, 0)?;
    let close = find_h_close(&words.1, 0)?;
    Some((
        close.pos,
        Expr::Num {
            locs: locs,
            str_start: words.0.pos,
            str: words.0.str.to_owned(),
        },
    ))
}

fn match_bifunc(vars: &VarSet, slice: &Slice)->MatchResult<(Expr,Expr)>{
    //let mut curr_slice = slice;
    let a  = match_expr(vars,slice)?;
    let b_start = slice.offset(a.0);
    let b  = match_expr(vars,&b_start)?;

    let close = find_h_close(&b_start, b.0-b_start.pos)?;

    Some((close.pos,(a.1,b.1)))
}

fn match_mult(vars: &VarSet, slice: &Slice, locs: Vec<usize>) -> MatchResult<Expr> {
    let (end,exprs) = match_bifunc(vars,slice)?;
    Some((end,Expr::Mult { locs, a: Box::new(exprs.0), b:Box::new(exprs.1)}))
}

fn match_add(vars: &VarSet, slice: &Slice, locs: Vec<usize>) -> MatchResult<Expr> {
    let (end,exprs) = match_bifunc(vars,slice)?;
    Some((end,Expr::Add { locs, a: Box::new(exprs.0), b:Box::new(exprs.1)}))
}

fn match_eq(vars: &VarSet, slice: &Slice, locs: Vec<usize>) -> MatchResult<Stat> {
    let words = get_next_word(slice, 0)?;
    if words.0.len() > 0 {
        let expr = match_expr(vars, &words.1)?;

        let close = find_h_close(&words.1, expr.0-words.1.pos)?;
        Some((
            close.pos,
            Stat::Eq {
                locs,
                name_start: words.0.pos,
                name: words.0.str.to_owned(),
                value: Box::new(expr.1),
            },
        ))
    } else {
        None
    }
}

fn match_line(vars: &VarSet, slice: &Slice, locs: Vec<usize>) -> MatchResult<Stat> {
    None
}

fn match_circle(vars: &VarSet, slice: &Slice, locs: Vec<usize>) -> MatchResult<Stat> {
    None
}

fn parse_line(vars: &VarSet, slice: &Slice) -> MatchResult<Stat> {
    match_built_in(&STAT_COMS, match_stat_fn, vars, slice)
}

pub fn parse(input: &mut dyn BufRead) -> Vec<Stat> {
    let mut vars: VarSet = HashSet::new();
    let mut statements: Vec<Stat> = Vec::new();

    for line in input.lines() {
        if let Ok(line_str) = line {
            if line_str.len()==0{
               break;
            }
            let str: Vec<u8> = line_str.bytes().collect();
            let slice = Slice { str: &str, pos: 0 };
            let result = parse_line(&vars, &slice);
            if let Some((_usize, stat)) = result {
                if let Stat::Eq {
                    locs: _,
                    name_start: _,
                    name,
                    value: _,
                } = &stat
                {
                    vars.insert(String::from_utf8_lossy(name).bytes().collect());
                }
                statements.push(stat);
            }
        }
    }
    statements
}
