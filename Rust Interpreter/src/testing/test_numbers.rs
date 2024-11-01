#[cfg(test)]
mod tests_assign_var {
    use crate::testing::*;
    use ntest::timeout;
    
    #[test]
    #[timeout(1000)]
    fn set_var_to_seven() {
        let data = run_parser!(b"I was going to be seventy.");
        check_lisp!(data, "(assign@2,3,4$25 \"going\"@6 (litnum 70@18$$7))");
    }

    #[test]
    #[timeout(1000)]
    fn set_var_to_seven_with_ellipsis() {
        let data = run_parser!(b"I was always seventy-seven....");
        check_lisp!(data, "(assign@2,3,4$26$$3 \"always\"@6 (litnum 77@13$$13))");
    }

    #[test]
    #[timeout(1000)]
    fn make_complicated_litnum() {
        let data = run_parser!(
            b"I was always one-hundred-and-twenty-three-thousand-three-hundred-and-two...."
        );
        check_lisp!(
            data,
            "(assign@2,3,4$72$$3 \"always\"@6 (litnum 123302@13$$59))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn make_twenty_one_litnum() {
        let data = run_parser!(b"I was always twenty-one....");
        check_lisp!(data, "(assign@2,3,4$23$$3 \"always\"@6 (litnum 21@13$$10))");
    }

    #[test]
    #[timeout(1000)]
    fn make_zero() {
        let data = run_parser!(b"I was always zero....");
        check_lisp!(data, "(assign@2,3,4$17$$3 \"always\"@6 (litnum 0@13$$4))");
    }

    #[test]
    #[timeout(1000)]
    fn make_gettysburg() {
        let data = run_parser!(b"I was always four-score-and-seven....");
        check_lisp!(data, "(assign@2,3,4$33$$3 \"always\"@6 (litnum 87@13$$20))");
    }

    #[test]
    #[timeout(1000)]
    fn do_not_make_gas_station() {
        let data = run_parser!(b"I was always seven-eleven two.");
        check_lisp!(data, "(assign@2,3,4$29 \"always\"@6 (litnum 2@26$$3))");
    }
}
