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
fn test_triple_newline() {
    let data = run_parser!(b"was twelve\ntwo.\n\n\nwas twelve\ntwo.");
    check_lisp!(data, "(assign@0,1,2$14 \"twelve\"@4 (litnum 2@11$$3))\n(assign@18,19,20$32 mut \"twelve\"@22 (litnum 2@29$$3))");
}

#[test]
#[timeout(1000)]
fn test_not() {
    let data = run_parser!(b"not two. was two two twelve two twenty-two twenty-three...");
    check_lisp!(data, "(assign@0,1,2$14 \"twelve\"@4 (litnum 2@11$$3))\n(assign@18,19,20$32 mut \"twelve\"@22 (litnum 2@29$$3))");
}
