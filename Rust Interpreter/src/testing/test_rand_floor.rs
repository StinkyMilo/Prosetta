#![cfg(test)]
use crate::testing::*;
use ntest::timeout;

#[test]
#[timeout(1000)]
fn test_rand_0() {
    let data = run_parser!(b"was random ran..");
    check_lisp!(
        data,
        "(assign@0,1,2$15 \"random\"@4 (rand@11,12,13$14))"
    );
}

#[test]
#[timeout(1000)]
fn test_rand_1_2() {
    let data = run_parser!(b"was random ran ran 2 3 4...");
    check_lisp!(
        data,
        "(assign@0,1,2$24$$3 \"random\"@4 (rand@11,12,13$24$$3 (rand@15,16,17$24$$3 (litnum 2@19$$1) (litnum 3@21$$1))))"
    );
}
