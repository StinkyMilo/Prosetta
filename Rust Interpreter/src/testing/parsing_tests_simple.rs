#[cfg(test)]
mod tests_simple {
    use crate::parser::*;
    use crate::testing::*;
    //use crate::linq_like_writer::*;
    //use std::hint;
    #[test]
    fn set_var_to_seven() {
        let text = b"I was going to be seventy.".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(assign@2,3,4$25 \"going\"@6 (litnum 70@18$$7))"
        );
    }

    #[test]
    fn set_var_to_seven_with_ellipsis() {
        let text = b"I was always seventy-seven....".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(assign@2,3,4$26$$3 \"always\"@6 (litnum 77@13$$13))"
        );
    }

    #[test]
    fn make_complicated_litnum() {
        let text = b"I was always one-hundred-and-twenty-three-thousand-three-hundred-and-two....".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(assign@2,3,4$72$$3 \"always\"@6 (litnum 123302@13$$59))"
        );
    }

    #[test]
    fn make_twenty_one_litnum() {
        let text = b"I was always twenty-one....".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(assign@2,3,4$23$$3 \"always\"@6 (litnum 21@13$$10))"
        );
    }

    #[test]
    fn make_zero() {
        let text = b"I was always zero....".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(assign@2,3,4$17$$3 \"always\"@6 (litnum 0@13$$4))"
        );
    }

    #[test]
    fn make_gettysburg() {
        let text = b"I was always four-score-and-seven....".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(assign@2,3,4$33$$3 \"always\"@6 (litnum 87@13$$20))"
        );
    }

    #[test]
    fn do_not_make_gas_station() {
        let text = b"I was always seven-eleven....".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::FailedLine
        );
    }

    #[test]
    fn test_wizards_with_double_close() {
        let text = b"The wizards were literally nine at most!".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(assign@4,7,10$39 \"were\"@12 (litnum@17,18,19$39 924))"
        );
    }

    #[test]
    fn test_wizards_with_double_close_ellipsis() {
        let text = b"The wizards were literally nine at most...".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(assign@4,7,10$39$$3 \"were\"@12 (litnum@17,18,19$39$$3 924))"
        );
    }

    #[test]
    fn lit_zero() {
        let text = b"The wizards were literally...".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(assign@4,7,10$26$$3 \"were\"@12 (litnum@17,18,19$26$$3 0))"
        );
    }

    #[test]
    fn test_nice_69() {
        let text = b"It was nice. lit six nine. Yeah.".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(assign@3,4,5$31 \"nice\"@7 (litnum@13,14,15$25 69))"
        );
    }
    #[test]
    fn test_ellipsis_6_close() {
        let text = b"It was sub sub sub sub sub sub one...".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(assign@3,4,5$34$$3 \"sub\"@7 (-@11,12,13$34$$3 (-@15,16,17$34$$3 \
            (-@19,20,21$34$$3 (-@23,24,25$34$$3 (-@27,28,29$34$$3 (litnum 1@31$$3)))))))"
        );
    }
    #[test]
    fn test_2_peirod() {
        let text = b"It was sub sub one..".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(assign@3,4,5$19 \"sub\"@7 (-@11,12,13$18 (litnum 1@15$$3)))"
        );
    }

    #[test]
    fn test_ellipsis_overload_12() {
        let text = b"It was sub sub sub sub sub sub sub sub sub sub sub sub one......".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(assign@3,4,5$61$$3 \"sub\"@7 (-@11,12,13$61$$3 (-@15,16,17$58$$3 (-@19,20,21$58$$3 (-@23,24,25$58$$3 \
            (-@27,28,29$58$$3 (-@31,32,33$58$$3 (-@35,36,37$58$$3 (-@39,40,41$58$$3 (-@43,44,45$58$$3 \
            (-@47,48,49$58$$3 (-@51,52,53$58$$3 (litnum 1@55$$3)))))))))))))"
        );
    }

    #[test]
    fn test_if_else_pri() {
        let text = b"whe one pri yes! else pri no:(:(".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        test_lib::run_to_completion(&mut parser);
        assert_eq!(
            linq_like_writer::write(&parser.data.exprs, &parser.data.stat_starts),
            "(if@0,1,2$15 (litnum 1@4$$3) then:\n  (print@8,9,10$15 \"yes\"@12)\n)\n(else@17,18,19$30\
            \n  (print@22,23,24$28 \"no\"@26)\n)"
        );
    }

    // #[test]
    // fn test_liechtenstein() {
    //     let text = b"The wars in Liechtenstein ravaged the country..".to_vec();
    //     let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
    //     assert_eq!(
    //         test_lib::assert_result(&mut parser),
    //         ParserResult::MatchedLine
    //     );
    //     assert_eq!(
    //         linq_like_writer::write_first(&parser.data.exprs),
    //         "(assign@4,5,7$46 \"in\"@9 (wordnum@13,19,21$45 @26$$7))"
    //     );
    // }

    // #[test]
    // fn test_nottingham() {
    //     let text = b"I was in Nottingham and it literally snowed the entire time I was there! All eight days!".to_vec();
    //     let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
    //     assert_eq!(
    //         test_lib::assert_result(&mut parser),
    //         ParserResult::MatchedLine
    //     );
    //     assert_eq!(
    //         linq_like_writer::write_first(&parser.data.exprs),
    //         "(assign@2,3,4$87 \"in\"@6 (skip@9,10,11 @20$71 (litnum 8@77$5)))"
    //     );
    // }

    // #[test]
    // fn test_easy_as_123() {
    //     let text = b"It was as nice andd easy as one two three..".to_vec();
    //     let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
    //     assert_eq!(
    //         test_lib::assert_result(&mut parser),
    //         ParserResult::MatchedLine
    //     );
    //     assert_eq!(
    //         linq_like_writer::write_first(&parser.data.exprs),
    //         "(assign@3,4,5$42 \"as\"@7 (add@15,17,18$41 (litnum 1@28$3) (litnum 2@32$3) (litnum 3@36$5)))"
    //     );
    // }

    // // #[test]
    // // fn test_it_was_not_as_easy() {
    // //     let text = b"It was as bad andd not as easy as one two three...".to_vec();
    // //     let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
    // //     assert_eq!(
    // //         test_lib::assert_result(&mut parser),
    // //         ParserResult::FailedLine
    // //     );
    // // }

    // // #[test]
    // // fn test_it_was_easy_as_one() {
    // //     let text = b"It was as bad add one but not two three...".to_vec();
    // //     let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
    // //     assert_eq!(
    // //         test_lib::assert_result(&mut parser),
    // //         ParserResult::FailedLine
    // //     );
    // // }

    // #[test]
    // fn test_submarine() {
    //     let text = b"It was SS Submarine seven..".to_vec();
    //     let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
    //     assert_eq!(
    //         test_lib::assert_result(&mut parser),
    //         ParserResult::MatchedLine
    //     );
    //     assert_eq!(
    //         linq_like_writer::write_first(&parser.data.exprs),
    //         "(assign@3,4,5$26 \"SS\"@7 (sub@10,11,12$25 (litnum 7@20$5)))"
    //     );
    // }

    #[test]
    fn test_line_4() {
        let text = b"lin one two three four!".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(line@0,1,2$22 (litnum 1@4$$3) (litnum 2@8$$3) (litnum 3@12$$5) (litnum 4@18$$4))"
        );
    }
}
