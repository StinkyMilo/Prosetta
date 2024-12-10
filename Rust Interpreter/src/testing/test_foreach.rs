#![cfg(test)]
use crate::testing::*;
use ntest::timeout;

#[test]
#[timeout(1000)]
fn test_foreach_variable_scope() {
    let data = run_parser!(b"fre ist list 1 2 3.. pri ist..");
    check_lisp!(
            data,
            "(foreach@0,1,2$29 ist (list@8,9,10$18 (litnum 1@13$$1) (litnum 2@15$$1) (litnum 3@17$$1)) then:\n  (print@21,22,23$28 (var \"ist\"@25))\n)"
        );
}

#[test]
#[timeout(1000)]
fn test_for_each() {
    let data = run_parser!(b"fre value lis 1 2 3. pri value..");
    check_lisp!(data,
            "(foreach@0,1,2$31 value (list@10,11,12$19 (litnum 1@14$$1) (litnum 2@16$$1) (litnum 3@18$$1)) then:\
            \n  (print@21,22,23$30 (var \"value\"@25))\n)"
        );
}

#[test]
#[timeout(1000)]
fn test_for_each_num() {
    let data = run_parser!(b"fre value 3. pri value..");
    check_lisp!(data,
            "(foreach@0,1,2$23 value (litnum 3@10$$1) then:\n  (print@13,14,15$22 (var \"value\"@17))\n)"
        );
}
