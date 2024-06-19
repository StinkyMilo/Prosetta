use std::{
    fmt::Debug,
    io::{self, BufRead, StdinLock},
};

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
        start: usize,
        end: usize,
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
            sources: vec![Source::String {
                str,
                start: 0,
                end: 0,
            }],
            index: 0,
        }
    }
}

impl<'a> ParserSource<'a> {
    pub fn get_line<'b>(&'b self) -> &'b [u8] {
        match &self.sources[self.index] {
            Source::Stdin { start, buf, .. } => &buf[*start..],
            Source::File => todo!(),
            Source::String { str, start, end } => &str[*start..*end],
        }
    }
    pub fn new_line<'b>(&'b mut self) -> Option<&'b [u8]> {
        let mut ret = false;
        while !ret {
            ret = match &mut self.sources[self.index] {
                Source::Stdin { source, start, buf } => {
                    let mut new_line = Vec::new();
                    let ret = source
                        .as_mut()
                        .is_some_and(|s| s.read_until(b'\n', &mut new_line).is_ok());
                    if ret {
                        *start += new_line.len();
                        buf.append(&mut new_line);
                    }
                    ret
                }
                Source::File => todo!(),
                Source::String { str, start, end } => {
                    let loc = str.iter().skip(*end).position(|&r| r == b'\n');
                    let index = loc.unwrap_or(str.len());
                    *start = *end;
                    *end = index;
                    *start != *end
                }
            };
        }
        Some(self.get_line())
    }

    pub fn drop_input(&mut self) {
        for s in &mut self.sources {
            if let Source::Stdin { source, .. } = s {
                *source = None;
            }
        }
    }
    pub fn reset(&mut self) {
        for s in &mut self.sources {
            if let Source::Stdin { source, .. } = s {
                *source = None;
            }
        }
    }
    pub fn get_n(&mut self) {
        for s in &mut self.sources {
            if let Source::Stdin { source, .. } = s {
                *source = None;
            }
        }
    }
}
