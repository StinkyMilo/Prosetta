#![cfg(test)]
use crate::testing::*;
use ntest::timeout;

#[test]
#[timeout(1000)]
fn test_if_else_pri() {
    let data = run_parser!(b"whe one pri yes! else pri no:(:(");
    check_lisp!(
        data,
        "(if@0,1,2$15 (litnum 1@4$$3) then:\n  (print@8,9,10$15 \"yes\"@12)\n)\n(else@17,18,19$30\
            \n  (print@22,23,24$28 \"no\"@26)\n)"
    );
}
#[test]
#[timeout(1000)]
fn test_if_space_else_pri() {
    let data = run_parser!(b"whe one pri yes. Or. pri maybe. Else pri no:( sadge :(");
    check_lisp!(data,
            "(if@0,1,2$19 (litnum 1@4$$3) then:\n  (print@8,9,10$15 \"yes\"@12)\n)\n(print@21,22,23$30 \"maybe\"@25)\n(print@37,38,39$43 \"no\"@41)"
        );
}

#[test]
#[timeout(1000)]
fn test_if_space_else_rect() {
    let data = run_parser!(b"whe one pri yes! Recording one two. Else pri no:( double sadge :(");
    check_lisp!(
        data,
        "(if@0,1,2$15 (litnum 1@4$$3) then:\n  \
            (print@8,9,10$15 \"yes\"@12)\n)\n\
            (rect@17,18,19$34 (litnum 1@27$$3) (litnum 2@31$$3))\n\
            (print@41,42,43$47 \"no\"@45)"
    );
}

#[test]
#[timeout(1000)]
fn test_else_past() {
    let data = run_parser!(b"whels one whi one pri hi..");
    check_lisp!(
        data,
        "(while@10,11,12$25 (litnum 1@14$$3) then:\n  (print@18,19,20$24 \"hi\"@22)\n)"
    );
}

#[test]
#[timeout(1000)]
fn test_double_else() {
    let data = run_parser!(b"whe one pri good.. els els pri bad...");
    check_lisp!(
        data,
        "(if@0,1,2$17 (litnum 1@4$$3) then:\n  \
            (print@8,9,10$16 \"good\"@12)\n)\n\
            (else@19,20,21$34$$3\n  (print@27,28,29$34$$3 \"bad\"@31)\n)"
    );
}

#[test]
#[timeout(1000)]
fn test_double_else_last() {
    let data = run_parser!(b"whe one pri good.. els pri bad. els...");
    check_lisp!(
        data,
        "(if@0,1,2$17 (litnum 1@4$$3) then:\n  \
            (print@8,9,10$16 \"good\"@12)\n)\n\
            (else@19,20,21$35$$3\n  (print@23,24,25$30 \"bad\"@27)\n)"
    );
}

#[test]
#[timeout(1000)]
fn test_if_else_simple() {
    let data = run_parser!(b"whe one pri one. pri two.. els pri three. pri four..");
    check_lisp!(
        data,
        "(if@0,1,2$25 (litnum 1@4$$3) then:\n  \
            (print@8,9,10$15 (litnum 1@12$$3))\n  \
            (print@17,18,19$24 (litnum 2@21$$3))\n)\n\
            (else@27,28,29$51\n  \
            (print@31,32,33$40 (litnum 3@35$$5))\n  \
            (print@42,43,44$50 (litnum 4@46$$4))\n)"
    );
}
#[test]
#[timeout(1000)]
fn test_if_inside_fail() {
    let data = run_parser!(b"whe one pri one. els. whe one els. pri two..");
    check_lisp!(
        data,
        "(if@0,1,2$20 (litnum 1@4$$3) then:\n  \
            (print@8,9,10$15 (litnum 1@12$$3))\n)\n\
            (if@22,23,24$43 (litnum 1@26$$3) then:\n  \
            (print@35,36,37$42 (litnum 2@39$$3))\n)"
    );
}

#[test]
#[timeout(1000)]
fn test_if_not() {
    let data = run_parser!(b"whe one not two! pri one!");
    check_lisp!(
        data,
        "(if@0,1,2$24 (litnum 1@4$$3) then:\n  \
        (not@8,9,10$15 @12$$3 \"two\")\n  \
        (print@17,18,19$24 (litnum 1@21$$3))\n)"
    );
}
