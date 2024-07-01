use std::{
    fmt::Debug,
    io::{self, BufRead, StdinLock},
    iter::{self, Flatten},
};

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
            sources: vec![Source::Stdin {
                source: Some(io::stdin().lock()),
                start: 0,
                buf: Vec::new(),
            }],
            index: 0,
        }
    }
    pub fn from_string(str: Vec<u8>) -> Self {
        Self {
            sources: vec![Source::String { str, first: true }],
            index: 0,
        }
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
                    //let mut new_input = Vec::new();
                    let mut ret = false;
                    // let
                    // while ret {
                    //     ret = source
                    //         .as_mut()
                    //         .is_some_and(|s| s.read_until(b'\n', &mut new_line).is_ok());
                    //     if ret {
                    //         *start = buf.len();
                    //         buf.append(&mut new_line);
                    //     }
                    // }
                    ret
                }
                Source::File => todo!(),
                Source::String { str, first } => {
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
    pub fn get_iter<'b>(&'b self) -> ParserSourceIter {
        let mut ret = Vec::new();
        let mut first = true;
        for s in &self.sources {
            if !first {
                ret.push(make_iter!(iter::once(&b'\n')));
            }
            first = false;
            ret.push(match s {
                Source::Stdin { buf, .. } => make_iter!(buf.iter()),
                Source::File => todo!(),
                Source::String { str, .. } => make_iter!(str.iter()),
            });
        }
        ret.into_iter().flatten()
    }
}
