#[cfg(test)]
mod tests_assign_var {
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
}
