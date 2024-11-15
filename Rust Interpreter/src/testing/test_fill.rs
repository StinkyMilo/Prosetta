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

#[test]
#[timeout(1000)]
fn test_stroke_color() {
    let data = run_parser!(b"sto dark green 1 2.");
    check_lisp!(
        data,
        "(stroke@0,1,2$18 (litcol darkgreen@4$$10))"
    );
}

#[test]
#[timeout(1000)]
fn test_stroke_3_numbers() {
    let data = run_parser!(b"sto 0 dark green 1 2.");
    check_lisp!(
        data,
        "(stroke@0,1,2$20 (litnum 0@4$$1) (litnum 1@17$$1) (litnum 2@19$$1))"
    );
}

#[test]
#[timeout(1000)]
fn test_stroke_transparent() {
    let data = run_parser!(b"sto 0. 1 2.");
    check_lisp!(
        data,
        "(stroke@0,1,2$5 (litnum 0@4$$1))"
    );
}

#[test]
#[timeout(1000)]
fn test_fill_color_alias() {
    let data = run_parser!(b"fil col 0. 1 2..");
    check_lisp!(
        data,
        "(fill@0,1,2$15 (color@4,5,6$14 (litnum 0@8$$1) (litnum 1@11$$1) (litnum 2@13$$1)))"
    );
}

#[test]
#[timeout(1000)]
fn test_stroke_width() {
    let data = run_parser!(b"pen 15.");
    check_lisp!(
        data,
        "(linewidth@0,1,2$6 (litnum 15@4$$2))"
    );
}


