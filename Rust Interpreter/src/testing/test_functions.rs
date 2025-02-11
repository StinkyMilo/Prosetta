#![cfg(test)]
use crate::testing::*;
use ntest::timeout;

#[test]
#[timeout(1000)]
fn test_ret_out_of_function() {
    let data = run_parser!(b"fun func. ret one! ret. pri hello. ret. func.");
    check_lisp!(data,
            "(function@0,1,2$17 \"func\"@4 (args) (return@10,11,12$17 (litnum 1@14$$3)))\n(print@24,25,26$33 \"hello\"@28)\n(\"func\"@40 )"
        );
}

#[test]
#[timeout(1000)]
fn test_0_arg_function() {
    let data = run_parser!(b"fun F'unc'. pri hi... fun'c.");
    check_lisp!(data,
            "(function@0,1,2$18$$3 \"func\"@4|1,5 (args) (print@12,13,14$18$$3 \"hi\"@16))\n(\"func\"@22|3 )"
        );
}

#[test]
#[timeout(1000)]
fn test_multi_arg_function() {
    let data = run_parser!(
        b"fun 'cause can't 'cause 'w'ow. p'ri hi! 'cause one two three. \
        c'a'us'e one two. cause one. cause."
    );
    check_lisp!(
        data,
        "(function@0,1,2$38 \"cause\"@4|0 (args \"cant\"@11|3 \"wow\"@24|0,2) \
            (print@31,33,34$38 \"hi\"@36))\n(\"cause\"@41 (litnum 1@47$$3) (litnum 2@51$$3))\n\
            (\"cause\"@62|1,3 (litnum 1@71$$3) (litnum 2@75$$3))"
    );
}

#[test]
#[timeout(1000)]
fn test_infinite_loop_function() {
    let data =
        run_parser!(b"fun in'finite. infi'n'ite... 'infinite'? pri \"this will never print\"...");
    check_lisp!(
        data,
        "(function@0,1,2$68$$3 \"infinite\"@4|2 (args) (\"infinite\"@15|4,6 ) \
        (\"infinite\"@30|8 ) (print@41,42,43$68$$3 \"this will never print\"@45))"
    );
}

#[test]
#[timeout(1000)]
fn test_function_fail() {
    let data = run_parser!(b"fun func. pri hi..");
    check_lisp!(
        data,
        "(function@0,1,2$17 \"func\"@4 (args) (print@10,11,12$16 \"hi\"@14))"
    );
}
