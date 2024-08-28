use std::{
    fmt::Debug,
    io::{stdin, BufRead, StdinLock},
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

    pub fn add_string(mut self, mut str: Vec<u8>) -> Self {
        // if last is not newline - add it
        if does_str_need_newline(&str) {
            str.push(b'\n');
        }
        self.sources.push(Source::String { str, first: true });
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
                        let mut new_input = Vec::new();
                        let has_failed = stdin.read_until(b'\n', &mut new_input).is_err();

                        // remove \r if it exists
                        if new_input.last() == Some(&b'\r') {
                            new_input.pop();
                        }

                        if has_failed || new_input.len() == 0 {
                            true
                        } else {
                            if buf.len() == 0 {
                                *buf = new_input;
                            } else {
                                //buf.push(b'\n');
                                *start = buf.len();
                                buf.append(&mut new_input);
                            }
                            false
                        }
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
    pub fn get_iter<'b>(&'b self) -> ParserSourceIter {
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

fn does_str_need_newline(str: &Vec<u8>) -> bool {
    !str.last().is_some_and(|f| *f == b'\n')
}
