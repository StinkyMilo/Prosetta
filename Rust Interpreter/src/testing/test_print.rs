#[cfg(test)]
mod tests_lit_int {
    use crate::testing::*;
    use ntest::timeout;

    #[test]
    #[timeout(1000)]
    fn test_print_no_vars() {
        let data = run_parser!( b"pri hi. pri hello world. pri \"hello world\".");
         check_lisp!(data,
            "(print@0,1,2$6 \"hi\"@4)\n(print@8,9,10$23 )\n(print@25,26,27$42 \"hello world\"@29)"
        );
    }
    #[test]
    #[timeout(1000)]
    fn test_print_vars() {
        let data = run_parser!( b"was test 4. pri one test two test. pri test. pri four.");
         check_lisp!(data,
            "(assign@0,1,2$10 \"test\"@4 (litnum 4@9$$1))\n(print@12,13,14$33 (litnum 1@16$$3) (var \"test\"@20) (litnum 2@25$$3) (var \"test\"@29))\n(print@35,36,37$43 (var \"test\"@39))\n(print@45,46,47$53 (litnum 4@49$$4))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_pri_mult_str() {
        let data = run_parser!( b"pri \"mario\" \"luigi\"!");
         check_lisp!(data,
            "(print@0,1,2$19 \"mario\"@4 \"luigi\"@12)"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_pri_varible_casing() {
        let data = run_parser!(b"was hi1 one. was HI2 two. pri hi1 Hi1 hi2 Hi2.");
         check_lisp!(data,
            "(assign@0,1,2$11 \"hi1\"@4 (litnum 1@8$$3))\n(assign@13,14,15$24 \"hi2\"@17 (litnum 2@21$$3))\n\
            (print@26,27,28$45 (var \"hi1\"@30) (var \"hi1\"@34) (var \"hi2\"@38) (var \"hi2\"@42))"
        );
    }
}
