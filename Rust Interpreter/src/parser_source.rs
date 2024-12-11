use std::{
    fmt::Debug,
    io::{stdin, BufRead, StdinLock},
    iter::{self, Flatten},
    mem,
};

use bstr::{ByteSlice, ByteVec};

pub type ParserSourceIter<'a> = Flatten<std::vec::IntoIter<Box<dyn Iterator<Item = &'a u8> + 'a>>>;

macro_rules! make_iter {
    ($expr:expr) => {
        Box::new($expr) as Box<dyn iter::Iterator<Item = &u8>>
    };
}

#[derive(Debug)]
enum Source<'a> {
    Stdin {
        source: Option<StdinLock<'a>>,
        start: usize,
        buf: Vec<u8>,
    },
    File,
    String {
        str: Vec<u8>,
        first: bool,
    },
}

#[derive(Debug)]
pub struct ParserSource<'a> {
    sources: Vec<Source<'a>>,
    index: usize,
}

impl<'a> ParserSource<'a> {
    pub fn from_stdin() -> Self {
        Self {
            sources: Vec::new(),
            index: 0,
        }
        .add_stdin()
    }
    pub fn from_string(str: Vec<u8>) -> Self {
        Self {
            sources: Vec::new(),
            index: 0,
        }
        .add_string(str)
    }
}

impl<'a> ParserSource<'a> {
    pub fn add_stdin(mut self) -> Self {
        self.sources.push(Source::Stdin {
            source: Some(stdin().lock()),
            start: 0,
            buf: Vec::new(),
        });
        self
    }

    pub fn add_string(mut self, str: Vec<u8>) -> Self {
        // if last is not newline - add it
        // if does_str_need_newline(&str) {
        //     str.push(b'\n');
        // }
        let mut paragraph = Vec::new();
        let mut empty_line = false;
        let mut first_text = false;
        for slice in str.split_str("\n") {
            // if is empty line -- change empty line
            if slice.trim().len() == 0 {
                empty_line = true;
            } else {
                // if empty line found -- make new buffer
                if empty_line && first_text {
                    // println!("{:?}", paragraph);
                    self.sources.push(Source::String {
                        str: mem::take(&mut paragraph),
                        first: true,
                    });
                }
                empty_line = false;
                first_text = true;
            }
            paragraph.push_str(slice);
            paragraph.push(b'\n');
        }
        // println!("{:?}", paragraph);
        self.sources.push(Source::String {
            str: paragraph,
            first: true,
        });

        self
    }
}

impl<'a> ParserSource<'a> {
    pub fn get_line<'b>(&'b self) -> &'b [u8] {
        match &self.sources[self.index] {
            Source::Stdin { start, buf, .. } => &buf[*start..],
            Source::File => todo!(),
            Source::String { str, first } => {
                // if getting before new line is set - return nothing
                if *first {
                    &[]
                } else {
                    &str
                }
            }
        }
    }
    pub fn new_line<'b>(&'b mut self) -> Option<&'b [u8]> {
        loop {
            if self.index >= self.sources.len() {
                return None;
            }
            let has_failed = match &mut self.sources[self.index] {
                Source::Stdin { source, start, buf } => {
                    if let Some(stdin) = source {
                        Self::get_from_stdin(stdin, start, buf)
                    } else {
                        true
                    }
                }
                Source::File => todo!(),
                Source::String { first, .. } => {
                    let ret = *first;
                    *first = false;
                    !ret
                }
            };
            if has_failed {
                self.index += 1;
            } else {
                return Some(self.get_line());
            }
        }
    }

    pub fn drop_input(&mut self) {
        for s in &mut self.sources {
            if let Source::Stdin { source, .. } = s {
                *source = None;
            }
        }
    }
    pub fn get_iter(&self) -> ParserSourceIter {
        let mut ret = Vec::new();
        let mut add_newline = false;
        for s in &self.sources {
            if add_newline {
                ret.push(make_iter!(iter::once(&b'\n')));
            }
            let iter;
            (iter, add_newline) = match s {
                Source::Stdin { buf, .. } => (make_iter!(buf.iter()), false),
                Source::File => todo!(),
                Source::String { str, .. } => (make_iter!(str.iter()), does_str_need_newline(str)),
            };
            ret.push(iter);
        }
        ret.into_iter().flatten()
    }
}

impl<'a> ParserSource<'a> {
    /// get input from stdin stoping on 0 len input
    /// returns has_failed
    fn get_from_stdin(stdin: &mut StdinLock<'a>, start: &mut usize, buf: &mut Vec<u8>) -> bool {
        println!("Input text to be parsed:");
        let mut has_input = false;
        // let mut has_first_empty = false;
        *start = buf.len();
        loop {
            let mut new_input = Vec::new();
            let has_failed = stdin.read_until(b'\n', &mut new_input).is_err();

            // remove newlines if it exists
            while !new_input.is_empty() {
                let last = *new_input.last().unwrap();
                if last == b'\n' || last == b'\r' {
                    new_input.pop();
                } else {
                    break;
                }
            }

            if has_failed || new_input.len() == 0 {
                // if has_first_empty {
                buf.push(b'\n');
                return !has_input;
            }

            has_input = true;
            if buf.len() == 0 {
                *buf = new_input;
            } else {
                buf.append(&mut new_input);
            }
            buf.push(b'\n');
        }
    }
}

fn does_str_need_newline(str: &Vec<u8>) -> bool {
    !str.last().is_some_and(|f| *f == b'\n')
}
