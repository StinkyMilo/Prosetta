#![cfg(test)]
use crate::testing::*;
use ntest::timeout;
#[test]
#[timeout(1000)]
fn test_fill_color() {
    let data = run_parser!(b"fill green 1 2.");
    check_lisp!(
        data,
        "(fill@0,1,2$14 (litcol green@5$$6))"
    );
}

#[test]
#[timeout(1000)]
fn test_fill_3_numbers() {
    let data = run_parser!(b"fill 0 green 1 2.");
    check_lisp!(
        data,
        "(fill@0,1,2$16 (litnum 0@5$$1) (litnum 1@13$$1) (litnum 2@15$$1))"
    );
}

#[test]
#[timeout(1000)]
fn test_fill_transparent() {
    let data = run_parser!(b"fill 0. 1 2.");
    check_lisp!(
        data,
        "(fill@0,1,2$6 (litnum 0@5$$1))"
    );
}

