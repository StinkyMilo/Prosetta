#![cfg(test)]

use crate::testing::*;
use ntest::timeout;

#[test]
#[timeout(1000)]
fn test_pri_fin() {
    let data = run_parser!(b"pri fin 3 2...");
    check_lisp!(data, "(print@0,1,2$11$$3 (litnum 3@8$$1) (litnum 2@10$$1))");
}

fn test_grace_step() {
    let data = run_parser!(b"the films director is jimothy green.");
    check_lisp!(data, "(print@0,1,2$11$$3 (litnum 3@8$$1) (litnum 2@10$$1))");
}
