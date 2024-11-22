#![cfg(test)]
use ntest::timeout;

use crate::testing::*;

#[test]
#[timeout(1000)]
fn test_double_newline() {
    let data = run_parser!(b"was twelve\ntwo.\nwas twelve\n\ntwo.");
    check_lisp!(data, "(assign@0,1,2$14 \"twelve\"@4 (litnum 2@11$$3))");
}

#[test]
#[timeout(1000)]
fn test_four_newline() {
    let data = run_parser!(b"was twelve\ntwo.\n\n\n\nwas twelve\ntwo.");
    check_lisp!(
        data,
        "(assign@0,1,2$14 \"twelve\"@4 (litnum 2@11$$3))\n\
        (assign@19,20,21$33 mut \"twelve\"@23 (litnum 2@30$$3))"
    );
}

#[test]
#[timeout(1000)]
fn test_not() {
    let data = run_parser!(b"not two. was two two twelve two twenty-two twenty-three...");
    check_lisp!(
        data,
        "(not@0,1,2$7 @4$$3 \"two\")\n(assign@9,10,11$55$$3 \"twelve\"@21 (litnum 23@43$$12))"
    );
}
