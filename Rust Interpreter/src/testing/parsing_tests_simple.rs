#[cfg(test)]
mod tests_simple {
    use crate::parser::*;
    use crate::testing::*;
    //use crate::linq_like_writer::*;
    //use std::hint;
    #[test]
    fn set_var_to_seven() {
        let text = b"I was seventy seven.".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(assign@2,3,4$19 \"seventy\"@6 (litnum 7@14$5))"
        );
    }

    #[test]
    fn test_wizards() {
        let text = b"The wizards were literally nine at most..".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(assign@4,7,10$40 \"were\"@12 (mutlilitnum@17,18,19$39 (litnum 9@27$4)))"
        );
    }

    #[test]
    fn test_nice_69() {
        let text = b"I was nice. lit six nine. Yeah.".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(assign@2,3,4$30 \"nice\"@6 (mutlilitnum@12,13,14$24 (litnum 6@16$3) (litnum 9@20$4)))"
        );
    }

    #[test]
    fn test_liechtenstein() {
        let text = b"The wars in Liechtenstein ravaged the country..".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), Default::default());
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(assign@4,5,7$46 \"in\"@9 (wordnum@13,19,21$45 \"ravaged\"@26))"
        );
    }

    #[test]
    fn test_nottingham() {
        let text = b"I was in Nottingham and it literally snowed the entire time I was there! All eight days!  ".to_vec();
        let mut parser = Parser::new(ParserSource::from_string(text), ParserFlags { not: true });
        assert_eq!(
            test_lib::assert_result(&mut parser),
            ParserResult::MatchedLine
        );
        assert_eq!(
            linq_like_writer::write_first(&parser.data.exprs),
            "(assign@2,3,4$87 \"in\"@6 (skip@9,10,11 @20$71 (litnum 8@77$5)))"
        );
    }
}
