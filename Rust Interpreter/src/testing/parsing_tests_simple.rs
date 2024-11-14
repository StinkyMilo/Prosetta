#![cfg(test)]
use ntest::timeout;

// use crate::parser::*;
use crate::testing::*;


#[test]
#[timeout(1000)]
fn test_line_4() {
    let data = run_parser!(b"lin one two three four!");
    check_lisp!(
        data,
        "(line@0,1,2$22 (litnum 1@4$$3) (litnum 2@8$$3) (litnum 3@12$$5) (litnum 4@18$$4))"
    );
}

#[test]
#[timeout(1000)]
fn test_double_newline() {
    let data = run_parser!(b"was twelve\ntwo.\nwas twelve\n\ntwo.");
    check_lisp!(data, "(assign@0,1,2$14 \"twelve\"@4 (litnum 2@11$$3))");
}


#[test]
#[timeout(1000)]
fn test_triple_newline() {
    let data = run_parser!(b"was twelve\ntwo.\n\n\nwas twelve\ntwo.");
    check_lisp!(data, "(assign@0,1,2$14 \"twelve\"@4 (litnum 2@11$$3))\n(assign@18,19,20$32 mut \"twelve\"@22 (litnum 2@29$$3))");
}
