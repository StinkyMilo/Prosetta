#[cfg(test)]
mod tests_simple {
    use ntest::timeout;

    use crate::parser::*;
    use crate::testing::*;
    use crate::writers::lisp_like_writer;
    //use crate::lisp_like_writer::*;
    //use std::hint;
    #[test]
    #[timeout(1000)]
    fn set_var_to_seven() {
        let text = b"I was going to be seventy.".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            lisp_like_writer::write_first(&parser.data.exprs),
            "(assign@2,3,4$25 \"going\"@6 (litnum 70@18$$7))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn set_var_to_seven_with_ellipsis() {
        let text = b"I was always seventy-seven....".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            lisp_like_writer::write_first(&parser.data.exprs),
            "(assign@2,3,4$26$$3 \"always\"@6 (litnum 77@13$$13))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn make_complicated_litnum() {
        let text = b"I was always one-hundred-and-twenty-three-thousand-three-hundred-and-two...."
            .to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            lisp_like_writer::write_first(&parser.data.exprs),
            "(assign@2,3,4$72$$3 \"always\"@6 (litnum 123302@13$$59))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn make_twenty_one_litnum() {
        let text = b"I was always twenty-one....".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            lisp_like_writer::write_first(&parser.data.exprs),
            "(assign@2,3,4$23$$3 \"always\"@6 (litnum 21@13$$10))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn make_zero() {
        let text = b"I was always zero....".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            lisp_like_writer::write_first(&parser.data.exprs),
            "(assign@2,3,4$17$$3 \"always\"@6 (litnum 0@13$$4))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn make_gettysburg() {
        let text = b"I was always four-score-and-seven....".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            lisp_like_writer::write_first(&parser.data.exprs),
            "(assign@2,3,4$33$$3 \"always\"@6 (litnum 87@13$$20))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn do_not_make_gas_station() {
        let text = b"I was always seven-eleven....".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::FailedLine
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_wizards_with_double_close() {
        let text = b"The wizards were literally nine at most!".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            lisp_like_writer::write_first(&parser.data.exprs),
            "(assign@4,7,10$39 \"were\"@12 (litnum@17,18,19$39 924))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_wizards_with_double_close_ellipsis() {
        let text = b"The wizards were literally nine at most...".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            lisp_like_writer::write_first(&parser.data.exprs),
            "(assign@4,7,10$39$$3 \"were\"@12 (litnum@17,18,19$39$$3 924))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_lit_zero() {
        let text = b"The wizards were literally...".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            lisp_like_writer::write_first(&parser.data.exprs),
            "(assign@4,7,10$26$$3 \"were\"@12 (litnum@17,18,19$26$$3 0))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_nice_69() {
        let text = b"It was nice. lit six nine. Yeah.".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            lisp_like_writer::write_first(&parser.data.exprs),
            "(assign@3,4,5$31 \"nice\"@7 (litnum@13,14,15$25 69))"
        );
    }
    #[test]
    #[timeout(1000)]
    fn test_ellipsis_6_close() {
        let text = b"It was sub sub sub sub sub sub one...".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            lisp_like_writer::write_first(&parser.data.exprs),
            "(assign@3,4,5$34$$3 \"sub\"@7 (-@11,12,13$34$$3 (-@15,16,17$34$$3 \
            (-@19,20,21$34$$3 (-@23,24,25$34$$3 (-@27,28,29$34$$3 (litnum 1@31$$3)))))))"
        );
    }
    #[test]
    #[timeout(1000)]
    fn test_2_peirod() {
        let text = b"It was sub sub one..".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            lisp_like_writer::write_first(&parser.data.exprs),
            "(assign@3,4,5$19 \"sub\"@7 (-@11,12,13$18 (litnum 1@15$$3)))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_ellipsis_overload_12() {
        let text = b"It was sub sub sub sub sub sub sub sub sub sub sub sub one......".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            lisp_like_writer::write_first(&parser.data.exprs),
            "(assign@3,4,5$61$$3 \"sub\"@7 (-@11,12,13$61$$3 (-@15,16,17$58$$3 (-@19,20,21$58$$3 (-@23,24,25$58$$3 \
            (-@27,28,29$58$$3 (-@31,32,33$58$$3 (-@35,36,37$58$$3 (-@39,40,41$58$$3 (-@43,44,45$58$$3 \
            (-@47,48,49$58$$3 (-@51,52,53$58$$3 (litnum 1@55$$3)))))))))))))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_if_else_pri() {
        let text = b"whe one pri yes! else pri no:(:(".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            lisp_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(if@0,1,2$15 (litnum 1@4$$3) then:\n  (print@8,9,10$15 \"yes\"@12)\n)\n(else@17,18,19$30\
            \n  (print@22,23,24$28 \"no\"@26)\n)"
        );
    }
    #[test]
    #[timeout(1000)]
    fn test_if_space_else_pri() {
        let text = b"whe one pri yes. Or. pri maybe. Else pri no:( sadge :(".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            lisp_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(if@0,1,2$19 (litnum 1@4$$3) then:\n  (print@8,9,10$15 \"yes\"@12)\n)\n(print@21,22,23$30 \"maybe\"@25)\n(print@37,38,39$43 \"no\"@41)"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_if_space_else_rect() {
        let text = b"whe one pri yes! Recording one two. Else pri no:( double sadge :(".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            lisp_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(if@0,1,2$15 (litnum 1@4$$3) then:\n  \
            (print@8,9,10$15 \"yes\"@12)\n)\n\
            (rect@17,18,19$34 (litnum 1@27$$3) (litnum 2@31$$3))\n\
            (print@41,42,43$47 \"no\"@45)"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_in_word_hyphen() {
        let text = b"I was about to learn in-depth mathematics -- It was crazy!".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            lisp_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(assign@2,3,4$42$$2 \"about\"@6 (wordnum@21,22,27$42$$2 @30$$11))"
        );
    }
    #[test]
    #[timeout(1000)]
    fn test_print_no_vars() {
        let text = b"pri hi. pri hello world. pri \"hello world\".".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            lisp_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(print@0,1,2$6 \"hi\"@4)\n(print@8,9,10$23 )\n(print@25,26,27$42 \"hello world\"@29)"
        );
    }
    #[test]
    #[timeout(1000)]
    fn test_print_vars() {
        let text = b"was test 4. pri one test two test. pri test. pri four.".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            lisp_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(assign@0,1,2$10 \"test\"@4 (litnum 4@9$$1))\n(print@12,13,14$33 (litnum 1@16$$3) (var \"test\"@20) (litnum 2@25$$3) (var \"test\"@29))\n(print@35,36,37$43 (var \"test\"@39))\n(print@45,46,47$53 (litnum 4@49$$4))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_else_past() {
        let text = b"whels one whi one pri hi..".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            lisp_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(while@10,11,12$25 (litnum 1@14$$3) then:\n  (print@18,19,20$24 \"hi\"@22)\n)"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_pri_mult_str() {
        let text = b"pri \"mario\" \"luigi\"!".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            lisp_like_writer::write_first(&parser.data.exprs),
            "(print@0,1,2$19 \"mario\"@4 \"luigi\"@12)"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_pri_varible_casing() {
        let text: Vec<u8> = b"was hi1 one. was HI2 two. pri hi1 Hi1 hi2 Hi2.".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            lisp_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(assign@0,1,2$11 \"hi1\"@4 (litnum 1@8$$3))\n(assign@13,14,15$24 \"hi2\"@17 (litnum 2@21$$3))\n\
            (print@26,27,28$45 (var \"hi1\"@30) (var \"hi1\"@34) (var \"hi2\"@38) (var \"hi2\"@42))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_varible_substrings() {
        let text: Vec<u8> =
            b"was cat one. was car cat. was car car. was cart cater. was cater handcarts.".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            lisp_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(assign@0,1,2$11 \"cat\"@4 (litnum 1@8$$3))\n\
            (assign@13,14,15$24 \"car\"@17 (var \"cat\"@21))\n\
            (assign@26,27,28$37 mut \"car\"@30 (var \"car\"@34))\n\
            (assign@39,40,41$53 \"cart\"@43 (var \"cat\"@48))\n\
            (assign@55,56,57$74 \"cater\"@59 (var \"cart\"@69))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_multilitnum_nan() {
        let text: Vec<u8> =
            b"was having little or no money in my purse, and nothing particular to interest me on shore, \
            I thought I would sail about a little and see the watery part of the world!".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            lisp_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(assign@0,1,2$165 \"having\"@4 \
            (multilitnum@11,12,13$165 2 2 5 2 2 5 1 3 7 0 2 8 2 2 5 1 1 7 1 5 4 5 1 6 3 3 3 6 4 2 3 5))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_var_apostrophes() {
        let text: Vec<u8> = b"wasn't 'cause one. was only b'e'ca'use'.".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            lisp_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(assign@0,1,2$17 \"cause\"@7|0 (litnum 1@14$$3))\n\
            (assign@19,20,21$39 \"only\"@23 (var \"cause\"@32|2))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_for_each() {
        let text: Vec<u8> = b"fre value lis 1 2 3. pri value..".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            lisp_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(foreach@0,1,2$31 value (list@10,11,12$19 (litnum 1@14$$1) (litnum 2@16$$1) (litnum 3@18$$1)) then:\
            \n  (print@21,22,23$30 (var \"value\"@25))\n)"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_double_else() {
        let text: Vec<u8> = b"whe one pri good.. els els pri bad...".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            lisp_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(if@0,1,2$17 (litnum 1@4$$3) then:\n  (print@8,9,10$16 \"good\"@12)\n)\n(else@19,20,21$34$$3\n  (print@27,28,29$34$$3 \"bad\"@31)\n)"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_ret_out_of_function() {
        let text: Vec<u8> = b"fun func. ret one! ret. pri hello. ret. func.".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            lisp_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(function@0,1,2$17 \"func\"@4 (args) (return@10,11,12$17 (litnum 1@14$$3)))\n(print@24,25,26$33 \"hello\"@28)\n(\"func\"@40 )"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_0_arg_function() {
        let text: Vec<u8> = b"fun F'unc'. pri hi... fun'c.".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            lisp_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(function@0,1,2$18$$3 \"func\"@4|1,5 (args) (print@12,13,14$18$$3 \"hi\"@16))\n(\"func\"@22|3 )"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_multi_arg_function() {
        let text: Vec<u8> = b"fun 'cause can't 'cause 'w'ow. p'ri hi! 'cause one two three. \
        c'a'us'e one two. cause one. cause."
            .to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            lisp_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(function@0,1,2$38 \"cause\"@4|0 (args \"cant\"@11|3 \"wow\"@24|0,2) \
            (print@31,33,34$38 \"hi\"@36))\n(\"cause\"@41 (litnum 1@47$$3) (litnum 2@51$$3))\n\
            (\"cause\"@62|1,3 (litnum 1@71$$3) (litnum 2@75$$3))"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_infinite_loop_function() {
        let text: Vec<u8> =
            b"fun in'finite. infi'n'ite... 'infinite'? pri \"this will never print\".".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            lisp_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(function@0,1,2$25$$3 \"infinite\"@4|2 (args) (\"infinite\"@15|4,6 ))\n\
            (\"infinite\"@30|8 )\n(print@41,42,43$68 \"this will never print\"@45)"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_function_fail() {
        let text: Vec<u8> =
            b"fun func. pri hi.".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            lisp_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(function@4,5,6$16 \"pri\"@10 (args) )"
        );
    }

    #[test]
    #[timeout(1000)]
    fn test_line_4() {
        let text = b"lin one two three four!".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            lisp_like_writer::write_first(&parser.data.exprs),
            "(line@0,1,2$22 (litnum 1@4$$3) (litnum 2@8$$3) (litnum 3@12$$5) (litnum 4@18$$4))"
        );
    }
    #[test]
    #[timeout(1000)]
    fn test_was_one_letter_var() {
        let text = b"was a mario two.".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            lisp_like_writer::write_first(&parser.data.exprs),
            "(assign@0,1,2$15 \"mario\"@6 (litnum 2@12$$3))"
        );
    }
}
