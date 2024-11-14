#![cfg(test)]
use crate::testing::*;
use ntest::timeout;
#[test]
#[timeout(1000)]
fn test_wizards_with_double_close() {
    let data = run_parser!(b"The wizards were literally nine at most!");
    check_lisp!(
        data,
        "(assign@4,7,10$39 \"were\"@12 (litnum@17,18,19$39 924))"
    );
}

#[test]
#[timeout(1000)]
fn test_wizards_with_double_close_ellipsis() {
    let data = run_parser!(b"The wizards were literally nine at most...");
    check_lisp!(
        data,
        "(assign@4,7,10$39$$3 \"were\"@12 (litnum@17,18,19$39$$3 924))"
    );
}

#[test]
#[timeout(1000)]
fn test_lit_zero() {
    let data = run_parser!(b"The wizards were literally...");
    check_lisp!(
        data,
        "(assign@4,7,10$26$$3 \"were\"@12 (litnum@17,18,19$26$$3 0))"
    );
}

#[test]
#[timeout(1000)]
fn test_lit_close_early() {
    let data = run_parser!(b"It was nice int. nice two..");
    check_lisp!(
        data,
        "(assign@3,4,5$26 \"nice\"@7 (wordnum@12,13,14$25 4@17))"
    );
}

#[test]
#[timeout(1000)]
fn test_multilitnum_overflow_fail() {
    let data = run_parser!(
            b"was having little or no money in my purse, and nothing particular to interest me on shore, \
            I thought I would sail about a little and see the watery part of the world!");
    check_lisp!(data,
            "(assign@0,1,2$165 \"having\"@4 \
            (multilitnum@11,12,13$165 2 2 5 2 2 5 1 3 7 0 2 8 2 2 5 1 1 7 1 5 4 5 1 6 3 3 3 6 4 2 3 5))"
        );
}

#[test]
#[timeout(1000)]
fn test_in_word_hyphen() {
    let data = run_parser!(b"I was about to learn in-depth mathematics -- It was crazy!");
    check_lisp!(
        data,
        "(assign@2,3,4$42$$2 \"about\"@6 (wordnum@21,22,27$42$$2 11@30))"
    );
}

#[test]
#[timeout(1000)]
fn test_word_no_close() {
    let data = run_parser!(b"I was two two. was into a two");
    check_lisp!(data, "(assign@2,3,4$13 \"two\"@6 (litnum 2@10$$3))");
}
