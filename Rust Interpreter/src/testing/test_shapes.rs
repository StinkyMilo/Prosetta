#![cfg(test)]
use ntest::timeout;

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
fn test_line_1_2_3() {
    let data = run_parser!(b"lin one. lin one two. lin one two three.");
    check_lisp!(
        data,
        "(line@0,1,2$7 (litnum 1@4$$3))\n\
        (line@9,10,11$20 (litnum 1@13$$3) (litnum 2@17$$3))\n\
        (line@22,23,24$39 (litnum 1@26$$3) (litnum 2@30$$3) (litnum 3@34$$5))"
    );
}
#[test]
#[timeout(1000)]
fn test_line_5() {
    let data = run_parser!(b"lin five four three two one zero...");
    check_lisp!(
        data,
        "(line@0,1,2$32$$3 (litnum 5@4$$4) (litnum 4@9$$4) (litnum 3@14$$5) (litnum 2@20$$3))"
    );
}
#[test]
#[timeout(1000)]
fn test_circle_3() {
    let data = run_parser!(b"arc one two three!");
    check_lisp!(
        data,
        "(arc@0,1,2$17 (litnum 1@4$$3) (litnum 2@8$$3) (litnum 3@12$$5))"
    );
}
#[test]
#[timeout(1000)]
fn test_circle_1_2() {
    let data = run_parser!(b"arc one. arc one two. arc one two three.");
    check_lisp!(
        data,
        "(arc@0,1,2$7 (litnum 1@4$$3))\n\
        (arc@9,10,11$20 (litnum 1@13$$3) (litnum 2@17$$3))\n\
        (arc@22,23,24$39 (litnum 1@26$$3) (litnum 2@30$$3) (litnum 3@34$$5))"
    );
}
