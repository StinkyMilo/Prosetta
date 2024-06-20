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
            "(set@2,3,4$19 \"seventy\"@6 (litnum 7@14$5))"
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
            "(set@4,7,10$40 \"were\"@12 (litnum@17,18,19$39 9@27$4))"
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
    }
}
