#[cfg(test)]
mod tests {
    use crate::parser::*;

    //use crate::linq_like_writer::*;
    use std::collections::HashSet;
    //use std::hint;

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
        locs: Option<Vec<usize>>,
    ) -> Enviroment<'a> {
        Enviroment {
            vars,
            expr,
            child_index,
            locs,
            global_index: 0,
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
        assert_eq!(
            get_next_word(&new_slice("", 2), 0),
            (new_slice("", 2), new_slice("", 2))
        );
        assert_eq!(
            get_next_word(&new_slice(" ", 4), 0),
            (new_slice("", 5), new_slice("", 5))
        );
        assert_eq!(
            get_next_word(&new_slice(" ", 8), 0),
            (new_slice("", 9), new_slice("", 9))
        );
    }

    #[test]
    fn test_get_next_word_out() {
        assert_eq!(
            get_next_word(&new_slice("a ", 0), 2),
            (new_slice("", 2), new_slice("", 2))
        );
        assert_eq!(
            get_next_word(&new_slice("a ", 0), 3),
            (new_slice("", 2), new_slice("", 2))
        );
        assert_eq!(
            get_next_word(&new_slice("a ", 0), 4),
            (new_slice("", 2), new_slice("", 2))
        );
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
        assert_eq!(parser.step(), ParserResult::ContinueWith("Equals"));
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::ContinueWith("Num"));
        assert_eq!(parser.step(), ParserResult::Matched("Num"));
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::Matched("Equals"));
        assert_eq!(parser.step(), ParserResult::MatchedLine("NoneStat"));
        assert_eq!(
            linq_like_writer::write_first(&parser.exprs),
            "(eq@0,1 \"inch\"@7 (num@13,15,16 \"rabbitfish\"@25))"
        );
        assert!(parser.vars.contains(&"inch".as_bytes().to_vec()));
    }

    #[test]
    fn test_parse_line_2() {
        let text = "Equations miles across amuse you as you inch, inch again, heating, heaving."
            .to_string();
        let mut binding = text.as_bytes();
        let mut parser = Parser::new(&mut binding);
        parser.vars.insert("inch".as_bytes().to_vec());
        assert_eq!(parser.step(), ParserResult::ContinueWith("Equals")); //Equations
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //across
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //across
        assert_eq!(parser.step(), ParserResult::ContinueWith("Mult")); //amuse
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //as
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //inch
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //inch
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //inch (2)
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //inch (2)
        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //inch (2)
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //inch (2)
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //inch (2)
        assert_eq!(parser.step(), ParserResult::Matched("Mult")); //heating
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //heating
        assert_eq!(parser.step(), ParserResult::Matched("Equals")); //heaving
        assert_eq!(parser.step(), ParserResult::MatchedLine("NoneStat")); //heaving
        assert_eq!(
            linq_like_writer::write_first(&parser.exprs),
            "(eq@0,1 \"miles\"@10 (mult@24,25 (var \"inch\"@40) (var \"inch\"@46)))"
        );
    }

    #[test]
    fn test_parse_line_2_no_var() {
        let text = "Equations miles across amuse you as you inch, inch again, heating, heaving."
            .to_string();
        let mut binding = text.as_bytes();
        let mut parser = Parser::new(&mut binding);
        assert_eq!(parser.step(), ParserResult::ContinueWith("Equals")); // Equations
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); // Equations
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); // across
        assert_eq!(parser.step(), ParserResult::ContinueWith("Mult")); //amuse
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //amuse
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //as
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //inch
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //inch
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //again
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //heating
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //heaving
        assert_eq!(parser.step(), ParserResult::Failed("NoneExpr")); //eof
        assert_eq!(parser.step(), ParserResult::Failed("Mult")); //eof
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //amuse
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //as
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //inch
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //inch
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //again
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //heating
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //heaving
        assert_eq!(parser.step(), ParserResult::Failed("NoneExpr")); //eof
        assert_eq!(parser.step(), ParserResult::Failed("Equals")); //eof
        assert_eq!(parser.step(), ParserResult::Continue("NoneStat")); //Equations
        assert_eq!(parser.step(), ParserResult::Continue("NoneStat")); //miles
        assert_eq!(parser.step(), ParserResult::Continue("NoneStat")); //across
        assert_eq!(parser.step(), ParserResult::Continue("NoneStat")); //amuse
        assert_eq!(parser.step(), ParserResult::Continue("NoneStat")); //you
        assert_eq!(parser.step(), ParserResult::Continue("NoneStat")); //as
        assert_eq!(parser.step(), ParserResult::Continue("NoneStat")); //you
        assert_eq!(parser.step(), ParserResult::Continue("NoneStat")); //inch
        assert_eq!(parser.step(), ParserResult::Continue("NoneStat")); //inch
        assert_eq!(parser.step(), ParserResult::Continue("NoneStat")); //again
        assert_eq!(parser.step(), ParserResult::Continue("NoneStat")); //heating
        assert_eq!(parser.step(), ParserResult::Continue("NoneStat")); //heaving
        assert_eq!(parser.step(), ParserResult::FailedLine("NoneStat")); //eof

        assert_eq!(parser.exprs.vec.len(), 0);
    }

    #[test]
    fn test_parse_line_3() {
        let text = "Equate furlongs to ambiguity; disencumber your heels. Inch farther, farther."
            .to_string();
        let mut binding = text.as_bytes();
        let mut parser = Parser::new(&mut binding);
        parser.vars.insert("inch".as_bytes().to_vec());
        parser.vars.insert("miles".as_bytes().to_vec());
        assert_eq!(parser.step(), ParserResult::ContinueWith("Equals")); //Equate furlongs
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //to
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //to
        assert_eq!(parser.step(), ParserResult::ContinueWith("Mult")); //ambiguity
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //disencumber
        assert_eq!(parser.step(), ParserResult::ContinueWith("Num")); //disencumber
        assert_eq!(parser.step(), ParserResult::Matched("Num")); //disencumber your heels
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //disencumber your heels
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //Inch
        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //Inch
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //Inch
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //Inch
        assert_eq!(parser.step(), ParserResult::Matched("Mult")); //farther
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //farther
        assert_eq!(parser.step(), ParserResult::Matched("Equals")); //farther (2)
        assert_eq!(parser.step(), ParserResult::MatchedLine("NoneStat")); //farther (2)
        assert_eq!(
            linq_like_writer::write_first(&parser.exprs),
            "(eq@0,1 \"furlongs\"@7 (mult@20,24 (num@34,36,37 \"your\"@42) (var \"inch\"@54)))"
        )
    }

    #[test]
    fn test_parse_line_4() {
        let text =
            "Equip longer armour. For miles, you swing your pendulum as you head home, harrowed."
                .to_string();
        let mut binding = text.as_bytes();
        let mut parser = Parser::new(&mut binding);
        parser.vars.insert("inch".as_bytes().to_vec());
        parser.vars.insert("miles".as_bytes().to_vec());
        parser.vars.insert("furlongs".as_bytes().to_vec());
        assert_eq!(parser.step(), ParserResult::ContinueWith("Equals")); //Equip longer
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //armour
        assert_eq!(parser.step(), ParserResult::ContinueWith("Mult")); //armour
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //armour
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //For
        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //miles
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //miles
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //miles
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //swing
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //your
        assert_eq!(parser.step(), ParserResult::ContinueWith("Num")); //pendulum as you head
        assert_eq!(parser.step(), ParserResult::Matched("Num")); //pendulum as you head
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //pendulum as you head
        assert_eq!(parser.step(), ParserResult::Matched("Mult")); //home
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //home
        assert_eq!(parser.step(), ParserResult::Matched("Equals")); //harrowed
        assert_eq!(parser.step(), ParserResult::MatchedLine("NoneStat")); //harrowed

        assert_eq!(
            linq_like_writer::write_first(&parser.exprs),
            "(eq@0,1 \"longer\"@6 (mult@15,17 (var \"miles\"@25) (num@49,51,54 \"as\"@56)))"
        )
    }

    #[test]
    fn test_parse_line_5() {
        let text =
            "Pick your pace. Go longer,  longer,  yet longer. Take the road below you.".to_string();
        let mut binding = text.as_bytes();
        let mut parser = Parser::new(&mut binding);
        parser.vars.insert("inch".as_bytes().to_vec());
        parser.vars.insert("miles".as_bytes().to_vec());
        parser.vars.insert("furlongs".as_bytes().to_vec());
        parser.vars.insert("longer".as_bytes().to_vec());
        assert_eq!(parser.step(), ParserResult::ContinueWith("Circle")); //Pick
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //your
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //your
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //pace
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //Go

        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //longer
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //longer
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //longer

        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //longer (2)
        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //longer (2)
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //longer (2)
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //longer (2)

        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //yet
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //yet
        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //longer (3)
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //longer (3)
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //longer (3)

        assert_eq!(parser.step(), ParserResult::Matched("Circle")); //Take the

        assert_eq!(parser.step(), ParserResult::MatchedLine("NoneStat")); //

        assert_eq!(
            linq_like_writer::write_first(&parser.exprs),
            "(circle@0,1 (var \"longer\"@19) (var \"longer\"@28) (var \"longer\"@41))"
        )
    }

    #[test]
    fn test_parse_line_6() {
        let text = "Point East. but few miles longer, to furlongs more ahead.".to_string();
        let mut binding = text.as_bytes();
        let mut parser = Parser::new(&mut binding);
        parser.vars.insert("inch".as_bytes().to_vec());
        parser.vars.insert("miles".as_bytes().to_vec());
        parser.vars.insert("furlongs".as_bytes().to_vec());
        parser.vars.insert("longer".as_bytes().to_vec());
        assert_eq!(parser.step(), ParserResult::ContinueWith("Circle")); //Point
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //East
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //East
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //but
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //few

        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //miles
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //miles
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //miles

        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //longer
        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //longer
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //longer
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //longer

        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //to
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //to
        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //furlongs
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //furlongs
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //furlongs

        assert_eq!(parser.step(), ParserResult::Matched("Circle")); //more ahead

        assert_eq!(parser.step(), ParserResult::MatchedLine("NoneStat")); //

        assert_eq!(
            linq_like_writer::write_first(&parser.exprs),
            "(circle@0,2 (var \"miles\"@20) (var \"longer\"@26) (var \"furlongs\"@37))"
        )
    }

    #[test]
    fn test_parse_line_7() {
        let text =
            "Point West. And no longer do miles hang before you. But longer furlongs await ahead."
                .to_string();
        let mut binding = text.as_bytes();
        let mut parser = Parser::new(&mut binding);
        parser.vars.insert("inch".as_bytes().to_vec());
        parser.vars.insert("miles".as_bytes().to_vec());
        parser.vars.insert("furlongs".as_bytes().to_vec());
        parser.vars.insert("longer".as_bytes().to_vec());
        assert_eq!(parser.step(), ParserResult::ContinueWith("Circle")); //Point
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //West
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //West
        assert_eq!(parser.step(), ParserResult::ContinueWith("Add")); //And
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //no
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //no

        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //longer
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //longer
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //longer

        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //no
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //no
        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //miles
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //miles
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //miles

        assert_eq!(parser.step(), ParserResult::Matched("Add")); //hang
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //hang

        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //before
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //before
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //But

        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //longer
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //longer
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //longer

        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //furlongs
        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //furlongs
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //furlongs
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //furlongs

        assert_eq!(parser.step(), ParserResult::Matched("Circle")); // await ahead
        assert_eq!(parser.step(), ParserResult::MatchedLine("NoneStat")); //

        assert_eq!(
            linq_like_writer::write_first(&parser.exprs),
            "(circle@0,2 (add@12,13,14 (var \"longer\"@19) (var \"miles\"@29)) (var \"longer\"@56) (var \"furlongs\"@63))"
        )
    }

    #[test]
    fn test_parse_line_8() {
        let text =
            "Like miles before and longer miles after, home remains away from view. And longer miles await ahead. And even longer miles stretch behind."
                .to_string();
        let mut binding = text.as_bytes();
        let mut parser = Parser::new(&mut binding);
        parser.vars.insert("inch".as_bytes().to_vec());
        parser.vars.insert("miles".as_bytes().to_vec());
        parser.vars.insert("furlongs".as_bytes().to_vec());
        parser.vars.insert("longer".as_bytes().to_vec());
        assert_eq!(parser.step(), ParserResult::ContinueWith("Line")); //Like
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //miles
        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //miles
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //miles
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //miles

        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //before
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //before

        assert_eq!(parser.step(), ParserResult::ContinueWith("Add")); //And
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //longer
        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //longer
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //longer
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //longer

        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //miles
        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //miles
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //miles
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //miles
        assert_eq!(parser.step(), ParserResult::Matched("Add")); //after, home
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //

        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //remains
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //remains
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //away
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //from
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //view

        assert_eq!(parser.step(), ParserResult::ContinueWith("Add")); //And
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //longer
        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //longer
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //longer
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //longer

        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //miles
        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //miles
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //miles
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //miles
        assert_eq!(parser.step(), ParserResult::Matched("Add")); //await ahead
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //

        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //And
        assert_eq!(parser.step(), ParserResult::ContinueWith("Add")); //And

        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //even
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //even

        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //longer
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //longer
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //longer

        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //miles
        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //miles
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //miles
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //miles
        assert_eq!(parser.step(), ParserResult::Matched("Add")); //stretch
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); // behind

        assert_eq!(parser.step(), ParserResult::Matched("Line")); // behind
        assert_eq!(parser.step(), ParserResult::MatchedLine("NoneStat")); //

        assert_eq!(
            linq_like_writer::write_first(&parser.exprs),
            "(line@0,1 (var \"miles\"@5) (add@18,19,20 (var \"longer\"@22) (var \"miles\"@29)) (add@71,72,73 (var \"longer\"@75) \
            (var \"miles\"@82)) (add@101,102,103 (var \"longer\"@110) (var \"miles\"@117)))"
        )
    }
    #[test]
    fn test_parse_line_1_and_2() {
        let text = "Equals inch innumerably. Rabbitfish hide in Hell.\n\
                Equations miles across amuse you as you inch, inch again, heating, heaving."
            .to_string();
        let mut binding = text.as_bytes();
        let mut parser = Parser::new(&mut binding);
        assert_eq!(parser.step(), ParserResult::ContinueWith("Equals"));
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::ContinueWith("Num"));
        assert_eq!(parser.step(), ParserResult::Matched("Num"));
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::Matched("Equals"));
        assert_eq!(parser.step(), ParserResult::MatchedLine("NoneStat"));

        assert_eq!(parser.step(), ParserResult::ContinueWith("Equals")); //Equations
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //across
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //across
        assert_eq!(parser.step(), ParserResult::ContinueWith("Mult")); //amuse
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //as
        assert_eq!(parser.step(), ParserResult::Continue("NoneExpr")); //you
        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //inch
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //inch
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //inch (2)
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr")); //inch (2)
        assert_eq!(parser.step(), ParserResult::ContinueWith("Var")); //inch (2)
        assert_eq!(parser.step(), ParserResult::Matched("Var")); //inch (2)
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //inch (2)
        assert_eq!(parser.step(), ParserResult::Matched("Mult")); //heating
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr")); //heating
        assert_eq!(parser.step(), ParserResult::Matched("Equals")); //heaving
        assert_eq!(parser.step(), ParserResult::MatchedLine("NoneStat")); //heaving
        assert_eq!(
            linq_like_writer::write(&parser.exprs, &parser.stat_starts),
            "(eq@0,1 \"inch\"@7 (num@13,15,16 \"rabbitfish\"@25))\n\
            (eq@50,51 \"miles\"@60 (mult@74,75 (var \"inch\"@90) (var \"inch\"@96)))\n"
        )
    }
}
