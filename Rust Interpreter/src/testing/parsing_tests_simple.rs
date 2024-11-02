#[cfg(test)]
mod tests_simple {
    use ntest::timeout;

    // use crate::parser::*;
    use crate::testing::*;

    
   

    #[test]
    #[timeout(1000)]
    fn test_if_else_pri() {
        let data = run_parser!( b"whe one pri yes! else pri no:(:(");
         check_lisp!(data,
            "(if@0,1,2$15 (litnum 1@4$$3) then:\n  (print@8,9,10$15 \"yes\"@12)\n)\n(else@17,18,19$30\
            \n  (print@22,23,24$28 \"no\"@26)\n)"
        );
    }
    #[test]
    #[timeout(1000)]
    fn test_if_space_else_pri() {
        let data = run_parser!( b"whe one pri yes. Or. pri maybe. Else pri no:( sadge :(");
         check_lisp!(data,
            "(if@0,1,2$19 (litnum 1@4$$3) then:\n  (print@8,9,10$15 \"yes\"@12)\n)\n(print@21,22,23$30 \"maybe\"@25)\n(print@37,38,39$43 \"no\"@41)"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_if_space_else_rect() {
        let data = run_parser!( b"whe one pri yes! Recording one two. Else pri no:( double sadge :(");
         check_lisp!(data,
            "(if@0,1,2$15 (litnum 1@4$$3) then:\n  \
            (print@8,9,10$15 \"yes\"@12)\n)\n\
            (rect@17,18,19$34 (litnum 1@27$$3) (litnum 2@31$$3))\n\
            (print@41,42,43$47 \"no\"@45)"
        );
    }

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
    fn test_else_past() {
        let data = run_parser!( b"whels one whi one pri hi..");
         check_lisp!(data,
            "(while@10,11,12$25 (litnum 1@14$$3) then:\n  (print@18,19,20$24 \"hi\"@22)\n)"
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
    fn test_double_else() {
        let data = run_parser!(b"whe one pri good.. els els pri bad...");
         check_lisp!(data,
            "(if@0,1,2$17 (litnum 1@4$$3) then:\n  (print@8,9,10$16 \"good\"@12)\n)\n(else@19,20,21$34$$3\n  (print@27,28,29$34$$3 \"bad\"@31)\n)"
        );
    }

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
        let data = run_parser!(b"fun 'cause can't 'cause 'w'ow. p'ri hi! 'cause one two three. \
        c'a'us'e one two. cause one. cause."
            );
         check_lisp!(data,
            "(function@0,1,2$38 \"cause\"@4|0 (args \"cant\"@11|3 \"wow\"@24|0,2) \
            (print@31,33,34$38 \"hi\"@36))\n(\"cause\"@41 (litnum 1@47$$3) (litnum 2@51$$3))\n\
            (\"cause\"@62|1,3 (litnum 1@71$$3) (litnum 2@75$$3))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_infinite_loop_function() {
        let data = run_parser!(
            b"fun in'finite. infi'n'ite... 'infinite'? pri \"this will never print\".");
         check_lisp!(data,
            "(function@0,1,2$25$$3 \"infinite\"@4|2 (args) (\"infinite\"@15|4,6 ))\n\
            (\"infinite\"@30|8 )\n(print@41,42,43$68 \"this will never print\"@45)"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_function_fail() {
        let data = run_parser!(b"fun func. pri hi.");
         check_lisp!(data,
            "(function@4,5,6$16 \"pri\"@10 (args) )"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_line_4() {
        let data = run_parser!( b"lin one two three four!");
         check_lisp!(data,
            "(line@0,1,2$22 (litnum 1@4$$3) (litnum 2@8$$3) (litnum 3@12$$5) (litnum 4@18$$4))"
        );
    }
    
    #[test]
    #[timeout(1000)]
    fn test_double_newline() {
        let data = run_parser!( b"was twelve\ntwo.\nwas twelve\n\ntwo.");
         check_lisp!(data,
            "(assign@0,1,2$14 \"twelve\"@4 (litnum 2@11$$3))"
        );
    }
}
