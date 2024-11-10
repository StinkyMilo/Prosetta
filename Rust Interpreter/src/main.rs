#![cfg(not(feature = "wasm"))]

use std::{
    io::{self, Read},
    mem,
};

use crate::parser::{Parser, ParserFlags};

#[path = "testing/testing.rs"]
mod testing;
//mod playground;

mod parser_runner;

mod commands;
mod docs_lib;
mod parser;
mod writers;

use docs_lib::gen_output;
use parser::ParserSource;
use parser_runner::{run_parser, RunnerFlags};

fn main() {
    if cfg!(feature = "gen-doc-output") {
        use std::fs;
        println!("Generating JS output...");
        let paths = fs::read_dir("../Frontend/docs").unwrap();

        for p in paths {
            match p {
                Ok(v) => {
                    let path = v.path();
                    // We're only using ASCII so I think this is fine
                    let path_str = path.display().to_string();

                    if path.is_file() && path_str.ends_with(".md") {
                        gen_output(&path_str);
                    }
                }
                Err(e) => println!("{}", e),
            }
        }
        return;
    }
    println!("size of parser: {}", mem::size_of::<Parser>());

    let mut args: Vec<String> = std::env::args().skip(1).collect();

    for e in &mut args {
        e.make_ascii_lowercase();
    }

    args.sort();

    let parser_flags = ParserFlags {
        title: !cfg!(feature = "no-title"), //args.binary_search(&"not".to_string()).is_ok(),
    };

    let vis_flags: RunnerFlags = RunnerFlags {
        assert_steps: true,
        input: true,
        whole_program: true,
        linted: true,
        line_rendered: true,
        word_trigger: true,
    };

    run_parser(
        parser_flags,
        vis_flags,
        ParserSource::from_stdin(), //ParserSource::from_string(MILO_POEM_2[0].to_vec())
    );

    let _ = io::stdin().read(&mut [0u8]).unwrap();
}

#[allow(dead_code)]
static MILO_POEM: [&[u8]; 2] = [
    b"
The wizards utter 'paint iambically.'
The peasants hadn't choice but to obey.
\
The wizards' cruel entertainments chant
and utter utter nonsense, void of weight.
\
The wizards' cursed victims utter trash;
the mages stand offended that despite
intent most fair, the peasants: they dissent!
\
The wizards thought it boon to speak in verse
but overestimate the peasant's skill;
'there is no point to it' the peasants thought.
'What cruel poetry they thrust on us.'
And so the peasants organized revolt.
\
They searched for mages speaking just in verse;
they thought and thought and thought 'where could they be?'",
    b"
But long had passed; magicians marched away
from cruel bitter thought and cursed man.
\
And so in lack of overlords but yet
still wrought by curse the agriculturists
admitted thought that life is not so bad.
Despite new vocal eccentricities,
their burdens lifted free of mages cruel.
They thought that they were cursed, but in fact
\
the wizards had abandonded cruel thought,
and left the peasants free of emperor.
\
And so their revolution had achieved
a world where peasants had to speak in verse
but answered not to any cruel lord
for they had long since gone, with nothing left
but a society that slowly learned
restriction fosters creativity.",
];

#[allow(dead_code)]
static MILO_POEM_2: [&[u8]; 1] = [b"
    was name les int marioooo. int luigi.!
    was name2 mor int marioooo. int luigi.!
    "];
