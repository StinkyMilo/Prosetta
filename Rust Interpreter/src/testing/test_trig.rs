#![cfg(test)]
use crate::testing::*;
use ntest::timeout;

#[test]
#[timeout(1000)]
fn test_sin() {
    let data = run_parser!(b"was sino sine. one! was sino sin one");
    check_lisp!(
        data,
        "(assign@0,1,2$18 \"sino\"@4 (sin@9,10,11$18 (litnum 1@15$$3)))"
    );
}

#[test]
#[timeout(1000)]
fn test_cos() {
    let data = run_parser!(b"was cost cosine. one! was cos cos ide 1..");
    check_lisp!(
        data,
        "(assign@0,1,2$20 \"cost\"@4 (cos@9,10,11$20 (litnum 1@17$$3)))\n\
        (assign@22,23,24$40 \"cos\"@26 (cos@30,31,32$39 (litnum 1@38$$1)))"
    );
}

#[test]
#[timeout(1000)]
fn test_tan() {
    let data = run_parser!(b"was tano tangent. one two! was tano tana sub..");
    check_lisp!(
        data,
        "(assign@0,1,2$25 \"tano\"@4 (tan@9,10,11$25 (litnum 1@18$$3)))"
    );
}
