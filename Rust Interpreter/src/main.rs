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
mod parser;
mod writers;

use parser::ParserSource;
use parser_runner::{run_parser, RunnerFlags};

fn main() {
    // test_incorrect_colors();
    // let t = Test;
    // let ta: Box<dyn Any> = Box::new(Test);
    // let ta2: Box<dyn Test2> = Box::new(Test);
    // println!("{:?}", t.type_id());
    // println!("{:?}", ta.type_id());
    // println!("{:?}", (*ta).type_id());
    // println!("{:?}", ta2.type_id());
    // println!("{:?}", (*(&*ta2 as &dyn Any)).type_id());
    //playground::print_test();
    //return;

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

// while !matches!(
//     result,
//     ParserResult::NoInput | ParserResult::MatchedLine(_) | ParserResult::FailedLine(_)
// ) {
//     result = parser.step();
// }
// //print!("{:?},", result);
// print!(
//     "{}",
//     linq_like_writer::write(&parser.exprs, &parser.stat_starts)
// );
// std::io::stdout().flush().unwrap();

// println!(
//     "name of eq: {}",
//     format!("{:#?}",(&commands::Expr::Eq {
//         locs: vec![0, 1],
//         name_start: 7,
//         name: "inch".as_bytes().to_vec(),
//         value_index: 1
//     }))
// );
//let mut buf= Vec::new();
//testing::test_ast1();
// let mut input =io::stdin().lock();
//let stats = parser_state::parse(&mut input);
//println!("{}",processing_writer::write(&stats))
//let s:String = Default::default();

//parser.vars.insert("inch".as_bytes().to_vec());
//crate::testing::add_vars!(parser, "inch", "miles", "furlongs", "longer");
//let mut result = parser.step();

// loop {
//     match parser.step() {
//         ParserResult::MatchedLine => print(&parser),
//         ParserResult::FailedLine => println!("   parse failed"),
//         ParserResult::NoInput => break,
//         _ => {}
//     }
// }

// loop {
//     match parser.step() {
//         ParserResult::NoInput => break,
//         state => println!(
//             "assert_step!(parser, {:?}, \"{}\", \"{}\");",
//             state,
//             parser.get_state(),
//             std::str::from_utf8(parser.get_word()).unwrap()
//         ),
//     }
// }

// let data = parser.into_data();
// let iter = data.source.get_iter();
// println!(
//     "    text input:\n\"{}\"",
//     std::str::from_utf8(iter.cloned().collect::<Vec<_>>().as_slice()).unwrap()
// );
// let iter = data.source.get_iter();
// //input.seek(SeekFrom::Start);
// //println!();
// //println!("== {:?}", parser.exprs.vec);
// println!(
//     "   whole program:\n{}",
//     linq_like_writer::write(&data.exprs, &data.stat_starts)
// );
// let mut lint = writers::syntax_lint::SyntaxLinter::<
//     writers::syntax_renderers::wind_renderer::WindowsRenderer,
// >::new();
// lint.write(&data.exprs, &data.stat_starts, iter);
// println!(
//     "   linted:\n{}",
//     std::str::from_utf8(&lint.into_string()).unwrap()
// );
// println!(
//     "   java program:\n{}",
//     processing_writer::write(&parser.exprs, &parser.stat_starts)
// );
