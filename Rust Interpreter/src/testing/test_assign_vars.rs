#![cfg(test)]
use crate::testing::*;
use ntest::timeout;

#[test]
#[timeout(1000)]
fn test_was_one_letter_var() {
    let data = run_parser!(b"was a mario two.");
    check_lisp!(data, "(assign@0,1,2$15 \"mario\"@6 (litnum 2@12$$3))");
}

#[test]
#[timeout(1000)]
fn test_var_apostrophes() {
    let data = run_parser!(b"wasn't 'cause one. was only b'e'ca'use'.");
    check_lisp!(
        data,
        "(assign@0,1,2$17 \"cause\"@7|0 (litnum 1@14$$3))\n\
            (assign@19,20,21$39 \"only\"@23 (var \"cause\"@32|2))"
    );
}

#[test]
#[timeout(1000)]
fn test_varible_substrings() {
    let data =
        run_parser!(b"was cat one. was car cat. was car car. was cart cater. was cater handcarts.");
    check_lisp!(
        data,
        "(assign@0,1,2$11 \"cat\"@4 (litnum 1@8$$3))\n\
            (assign@13,14,15$24 \"car\"@17 (var \"cat\"@21))\n\
            (assign@26,27,28$37 mut \"car\"@30 (var \"car\"@34))\n\
            (assign@39,40,41$53 \"cart\"@43 (var \"cat\"@48))\n\
            (assign@55,56,57$74 \"cater\"@59 (var \"cart\"@69))"
    );
}

#[test]
#[timeout(1000)]
fn test_assign_no_end() {
    let data = run_parser!(b"was cat one. was car two");
    check_lisp!(data, "(assign@0,1,2$11 \"cat\"@4 (litnum 1@8$$3))");
}
