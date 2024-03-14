#[cfg(test)]
mod tests {
    use crate::parser::*;

    use crate::linq_like_writer::*;
    use std::collections::HashSet;

    fn new_slice(str: &str, start: usize) -> Slice {
        Slice {
            str: str.as_ref(),
            pos: start,
        }
    }

    fn new_sub_slice(str: &str, start: usize) -> Slice {
        let bytes: &[u8] = str.as_ref();
        Slice {
            str: &bytes[start..],
            pos: start,
        }
    }

    fn new_env<'a>(
        vars: &'a HashSet<Vec<u8>>,
        expr: &'a mut Expr,
        child_index: usize,
        locs: Option<Vec<usize>>
    ) -> Enviroment<'a> {
        Enviroment {
            vars,
            expr,
            child_index,
            locs
        }
    }

    #[test]
    fn test_get_next_word_simple() {
        assert_eq!(
            get_next_word(&new_slice("asdf   ", 2), 0),
            (new_slice("asdf", 2), new_slice("   ", 6))
        );

        assert_eq!(
            get_next_word(&new_slice(" asdf  ", 1), 0),
            (new_slice("asdf", 2), new_slice("  ", 6))
        );

        assert_eq!(
            get_next_word(&new_slice("  asdf ", 0), 0),
            (new_slice("asdf", 2), new_slice(" ", 6))
        );
    }

    #[test]
    fn test_get_next_word_no_space_end() {
        assert_eq!(
            get_next_word(&new_slice("asdf", 2), 0),
            (new_slice("asdf", 2), new_slice("", 6))
        );

        assert_eq!(
            get_next_word(&new_slice(" asdf", 2), 0),
           (new_slice("asdf", 3), new_slice("", 7))
        );

        assert_eq!(
            get_next_word(&new_slice("  asdf", 2), 0),
            (new_slice("asdf", 4), new_slice("", 8))
        );
    }
    #[test]
    fn test_get_next_word_fails() {
        assert_eq!(get_next_word(&new_slice("", 2), 0), (new_slice("", 2), new_slice("", 2)));
        assert_eq!(get_next_word(&new_slice(" ", 4), 0), (new_slice("", 5), new_slice("", 5)));
        assert_eq!(get_next_word(&new_slice(" ", 8), 0), (new_slice("", 9), new_slice("",9)));
    }

    #[test]
    fn test_get_next_word_out() {
        assert_eq!(get_next_word(&new_slice("a ", 0), 2), (new_slice("", 2), new_slice("", 2)));
        assert_eq!(get_next_word(&new_slice("a ", 0), 3), (new_slice("", 2), new_slice("", 2)));
        assert_eq!(get_next_word(&new_slice("a ", 0), 4), (new_slice("", 2), new_slice("", 2)));
    }

    #[test]
    fn test_find_word_end_after() {
        assert_eq!(
            find_word_end(&new_slice(" a b c", 0), 0),
            new_slice(" a b c", 0)
        );
        assert_eq!(
            find_word_end(&new_slice(" a b c", 0), 1),
            new_slice(" b c", 2)
        );
        assert_eq!(
            find_word_end(&new_slice(" a b c", 0), 2),
            new_slice(" b c", 2)
        );
        assert_eq!(
            find_word_end(&new_slice(" a b c", 0), 3),
            new_slice(" c", 4)
        );
    }

    #[test]
    fn test_find_word_end_fails() {
        assert_eq!(find_word_end(&new_slice("", 0), 0), new_slice("", 0));
        assert_eq!(find_word_end(&new_slice(" ", 0), 0), new_slice(" ", 0));
        assert_eq!(find_word_end(&new_slice("a", 0), 0), new_slice("", 1));
        assert_eq!(find_word_end(&new_slice("a ", 0), 0), new_slice(" ", 1));
        assert_eq!(find_word_end(&new_slice(" a", 0), 0), new_slice(" a", 0));
    }

    #[test]
    fn test_find_word_end_out() {
        assert_eq!(find_word_end(&new_slice("a ", 0), 2), new_slice("", 2));
        assert_eq!(find_word_end(&new_slice("a  ", 0), 3), new_slice("", 3));
        assert_eq!(find_word_end(&new_slice("a  ", 0), 4), new_slice("", 3));
    }

    #[test]
    fn test_find_h_close_after() {
        assert_eq!(
            find_h_close(&new_slice(" a h b ", 0), 0),
            Some(new_slice(" b ", 4))
        );
        assert_eq!(
            find_h_close(&new_slice(" a H b ", 0), 0),
            Some(new_slice(" b ", 4))
        );
        assert_eq!(
            find_h_close(&new_slice(" a hb c", 0), 0),
            Some(new_slice(" c", 5))
        );
        assert_eq!(
            find_h_close(&new_slice(" a bhc d", 0), 0),
            Some(new_slice(" d", 6))
        );
    }

    #[test]
    fn test_find_h_close_fails() {
        assert_eq!(find_h_close(&new_slice("h", 0), 0), Some(new_slice("", 1)));
        assert_eq!(find_h_close(&new_slice("ha", 0), 0), Some(new_slice("", 2)));
        assert_eq!(
            find_h_close(&new_slice("haa", 0), 0),
            Some(new_slice("", 3))
        );
        assert_eq!(
            find_h_close(&new_slice("haa ", 0), 0),
            Some(new_slice(" ", 3))
        );
        assert_eq!(find_h_close(&new_slice("a b c d", 0), 0), None);
    }

    #[test]
    fn test_find_h_close_out() {
        assert_eq!(find_h_close(&new_slice("a ", 0), 2), None);
        assert_eq!(find_h_close(&new_slice("a ", 0), 3), None);
        assert_eq!(find_h_close(&new_slice("a ", 0), 4), None);
    }

    // #[test]
    // fn test_step_num_simple() {
    //     let vars = HashSet::new();
    //     let mut expr = Expr::Num {
    //         locs: vec![0, 1],
    //         str_start: 0,
    //         str: Vec::new(),
    //     };
    //     let mut state: StateContext = StateContext::None;
    //     let mut env = new_env(&vars, &mut expr, &mut state, 0);
    //     let str = &new_sub_slice("num Rabbitfish hiding".as_ref(), 3);
    //     let (word, rest) = get_next_word(&str, 0);
    //     assert_eq!(
    //         step_num(&mut env, MatchChildResult::None, &word, &rest),
    //         MatchResult::Matched(21)
    //     );
    //     assert_eq!(
    //         expr,
    //         Expr::Num {
    //             locs: vec![0, 1],
    //             str_start: 4,
    //             str: b"rabbitfish".to_vec()
    //         }
    //     );
    //     assert_eq!(
    //         write_one(&ExprArena { vec: vec![expr] }),
    //         "(num@0,1 \"rabbitfish\"@4)"
    //     );
    //     assert_eq!(state, StateContext::None);
    // }

    // #[test]
    // fn test_step_num_fail() {
    //     let vars = HashSet::new();
    //     let mut expr = Expr::Num {
    //         locs: vec![0, 1],
    //         str_start: 0,
    //         str: Vec::new(),
    //     };
    //     let mut state: StateContext = StateContext::None;
    //     let mut env = new_env(&vars, &mut expr, &mut state, 0);
    //     let str = &new_sub_slice("num Rabbitfish".as_ref(), 3);
    //     let (word, rest) = get_next_word(&str, 0);
    //     assert_eq!(
    //         step_num(&mut env, MatchChildResult::None, &word, &rest),
    //         MatchResult::Failed
    //     );
    // }

    #[test]
    fn test_parse_line_1() {
        let text = "Equals inch innumerably. Rabbitfish hide in Hell.".to_string();
        let mut binding = text.as_bytes();
        let mut parser = Parser::new(&mut binding);
        assert_eq!(parser.step(), ParserResult::Continue("Equals"));
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::Continue("Num"));
        assert_eq!(parser.step(), ParserResult::Matched("Num"));
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::Matched("Equals"));
        assert_eq!(parser.step(), ParserResult::MatchedLine("NoneStat"));
        assert_eq!(linq_like_writer::write_one(&parser.exprs),"(eq@0,1 \"inch\"@7 (num@13,15,16 \"rabbitfish\"@25))")
    }


    #[test]
    fn test_parse_line_2() {
        let text = "Equations miles across amuse you as you inch, inch again, heating, heaving.".to_string();
        let mut binding = text.as_bytes();
        let mut parser = Parser::new(&mut binding);
        parser.vars.insert("inch".as_bytes().to_vec());
        assert_eq!(parser.step(), ParserResult::Continue("Equals")); //Equations
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //across
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //across
        assert_eq!(parser.step(), ParserResult::Continue("Mult")); //amuse
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //as
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::Continue("Var")); //inch
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //inch
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //inch (2)
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //inch (2)
        assert_eq!(parser.step(), ParserResult::Continue("Var")); //inch (2)
        assert_eq!(parser.step(), ParserResult::Matched("Var"));  //inch (2)
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //inch (2)
        assert_eq!(parser.step(), ParserResult::Matched("Mult")); //heating
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //heating
        assert_eq!(parser.step(), ParserResult::Matched("Equals")); //heaving
        assert_eq!(parser.step(), ParserResult::MatchedLine("NoneStat")); //heaving
        assert_eq!(linq_like_writer::write_one(&parser.exprs),"(eq@0,1 \"miles\"@10 (mult@24,25 (var \"inch\"@40) (var \"inch\"@46)))")
    }
    #[test]
    fn test_parse_line_2_no_var() {
        let text = "Equations miles across amuse you as you inch, inch again, heating, heaving.".to_string();
        let mut binding = text.as_bytes();
        let mut parser = Parser::new(&mut binding);
        assert_eq!(parser.step(), ParserResult::Continue("Equals")); // Equations
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); // Equations
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); // across
        assert_eq!(parser.step(), ParserResult::Continue("Mult")); //amuse
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //amuse
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //as
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //inch
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //inch
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //again
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //heating
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //heaving
        assert_eq!(parser.step(), ParserResult::Failed("NoneExpr")); //eof
        assert_eq!(parser.step(), ParserResult::Failed("Mult")); //eof
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //amuse
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //as
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //inch
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //inch
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //again
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //heating
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr")); //heaving
        assert_eq!(parser.step(), ParserResult::Failed("NoneExpr")); //eof
        assert_eq!(parser.step(), ParserResult::Failed("Equals")); //eof
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneStat")); //Equations
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneStat")); //miles
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneStat")); //across
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneStat")); //amuse
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneStat")); //you
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneStat")); //as
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneStat")); //you
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneStat")); //inch
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneStat")); //inch
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneStat")); //again
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneStat")); //heating
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneStat")); //heaving
        assert_eq!(parser.step(), ParserResult::FailedLine("NoneStat")); //eof

        assert_eq!(parser.exprs.vec.len(),0);
    }

    #[test]
    fn test_parse_line_3() {
        let text = "Equate furlongs to ambiguity; disencumber your heels. Inch farther, farther.".to_string();
        let mut binding = text.as_bytes();
        let mut parser = Parser::new(&mut binding);
        parser.vars.insert("inch".as_bytes().to_vec());
        parser.vars.insert("miles".as_bytes().to_vec());
        assert_eq!(parser.step(), ParserResult::Continue("Equals"));
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::ContinueFail("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::Continue("Mult"));
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::Continue("Num"));
        assert_eq!(parser.step(), ParserResult::Matched("Num"));
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::Continue("Var"));
        assert_eq!(parser.step(), ParserResult::Matched("Var"));
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::Continue("Mult"));
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::Matched("Equals"));
        assert_eq!(parser.step(), ParserResult::MatchedLine("NoneStat"));
        assert_eq!(linq_like_writer::write_one(&parser.exprs),"(eq@0,1 \"miles\"@10 (mult@24,25 (var \"inch\"@40) (var \"inch\"@46)))")
    }

    //     #[test]
    //     fn test_1() {
    //         assert_eq!(
    //             parse_line(
    //                 &HashSet::new(),
    //                 &new_slice(
    //                     "Equals inch innumerably. Rabbitfish hide close at hand. ",
    //                     0
    //                 )
    //             ),
    //             (
    //                 54,
    //                 vec![Expr::Eq {
    //                     locs: vec![0, 1],
    //                     name_start: 7,
    //                     name: "inch".as_bytes().to_vec(),
    //                     value_index: 1
    //                 },
    //                 Expr::Num {
    //                     locs: vec![13, 15, 16],
    //                     str_start: 25,
    //                     str: "Rabbitfish".as_bytes().to_vec()
    //                 }],
    //                 vec![0]
    //             )
    //         );
    //     }

    //     #[test]
    //     fn test_var() {
    //         assert_eq!(
    //             parse_line(
    //                 &HashSet::from(["inch".as_bytes().to_vec()]),
    //                 &new_slice("Dont equal foot. inch your way further.", 0)
    //             ),
    //             (
    //                 38,
    //                 vec![Expr::Eq {
    //                     locs: vec![5, 6],
    //                     name_start: 11,
    //                     name: "foot".as_bytes().to_vec(),
    //                     value_index:1
    //                 },
    //                 Expr::Var {
    //                     name_start: 17,
    //                     name: "inch".as_bytes().to_vec()
    //                 }],
    //                 vec![0]
    //             )
    //         );
    //         assert_eq!(
    //             parse_line(
    //                 &HashSet::new(),
    //                 &new_slice("Dont equal inch. Rabbitfish hardly exist", 0)
    //             ),
    //             (40,Expr::None)
    //         );
    //     }

    //     #[test]
    //     fn test_2() {
    //         assert_eq!(
    //             parse_line(
    //                 &HashSet::from(["inch".as_bytes().to_vec()]),
    //                 &new_slice(
    //                     "Equations miles across amuse you as you inch, inch again, heating, heaving.",
    //                     0
    //                 )
    //             ),
    //             (
    //                 38,
    //                 vec![Expr::Eq {
    //                     locs: vec![0, 1],
    //                     name_start: 10,
    //                     name: "miles".as_bytes().to_vec(),
    //                     value_index:1
    //                 },
    //                 Expr::Mult {
    //                         locs: vec![24, 25],
    //                         a_index:2,
    //                         b_index:3
    //                     },
    //                 Expr::Var {
    //                     name_start: 40,
    //                     name: "inch".as_bytes().to_vec()
    //                 },
    //                 Expr::Var {
    //                     name_start: 46,
    //                     name: "inch".as_bytes().to_vec()
    //                 }],
    //                 vec![0]
    //             )
    //         );
    //     }
}
