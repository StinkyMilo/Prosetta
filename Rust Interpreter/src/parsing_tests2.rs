#[cfg(test)]
mod tests2 {
    //use crate::parser::parsing_tests1;
    use crate::parser::*;
    //use crate::linq_like_writer::*;
    //use std::hint;
    #[test]
    fn test_parse_line_sub_literal() {
        let text = "Eq seven sub nine two h h".to_string();
        let mut binding = text.as_bytes();
        let mut parser = Parser::new(&mut binding);
        assert_eq!(parser.step(), ParserResult::ContinueWith("Equals"));
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::ContinueWith("Sub"));
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::ContinueWith("NumLiteral"));
        assert_eq!(parser.step(), ParserResult::Matched("NumLiteral"));
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::ContinueWith("NumLiteral"));
        assert_eq!(parser.step(), ParserResult::Matched("NumLiteral"));
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::Matched("Sub"));
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::Matched("Equals"));
        assert_eq!(parser.step(), ParserResult::MatchedLine("NoneStat"));
        assert_eq!(
            linq_like_writer::write_first(&parser.exprs),
            "(eq@0,1 \"seven\"@3 (sub@9,10,11 (litnum 9@13$4) (litnum 2@18$3)))"
        );
    }

    #[test]
    fn test_parse_line_sub_literal_expr() {
        let text = "Eq nice sub lit one zero two four h lit Nine Five Five h h h".to_string();
        let mut binding = text.as_bytes();
        let mut parser = Parser::new(&mut binding);
        assert_eq!(parser.step(), ParserResult::ContinueWith("Equals"));
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::ContinueWith("Sub"));
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::ContinueWith("NumLit"));
        assert_eq!(parser.step(), ParserResult::Continue("NumLit"));
        assert_eq!(parser.step(), ParserResult::Continue("NumLit"));
        assert_eq!(parser.step(), ParserResult::Continue("NumLit"));
        assert_eq!(parser.step(), ParserResult::Continue("NumLit"));
        assert_eq!(parser.step(), ParserResult::Matched("NumLit"));
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::ContinueWith("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::ContinueWith("NumLit"));
        assert_eq!(parser.step(), ParserResult::Continue("NumLit"));
        assert_eq!(parser.step(), ParserResult::Continue("NumLit"));
        assert_eq!(parser.step(), ParserResult::Continue("NumLit"));
        assert_eq!(parser.step(), ParserResult::Matched("NumLit"));
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::Matched("Sub"));
        assert_eq!(parser.step(), ParserResult::Matched("NoneExpr"));
        assert_eq!(parser.step(), ParserResult::Matched("Equals"));
        assert_eq!(parser.step(), ParserResult::MatchedLine("NoneStat"));
        assert_eq!(
            linq_like_writer::write_first(&parser.exprs),
            "(eq@0,1 \"nice\"@3 (sub@8,9,10 (litnum@12,13,14 1024@16$17) (litnum@36,37,38 955@40$14)))"
        );
    }
}
