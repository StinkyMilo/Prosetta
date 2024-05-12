#[cfg(test)]
mod tests2 {
    use crate::parser::*;
    use crate::testing::*;
    //use crate::linq_like_writer::*;
    //use std::hint;
    #[test]
    fn test_parse_line_sub_literal() {
        let text = "Eq seven sub nine two h h".to_string();
        let mut binding = text.as_bytes();
        let mut parser = Parser::new(&mut binding);
        assert_step!(parser, Start, "NoneStat", "Eq");
        assert_step!(parser, ContinueWith, "Equals", "seven");
        {
            assert_step!(parser, ContinueWith, "NoneExprCont", "sub");
            assert_step!(parser, ContinueWith, "Sub", "nine");
            {
                assert_step!(parser, ContinueWith, "NoneExpr", "nine");
                assert_step!(parser, ContinueWith, "NumLiteral", "nine");
                assert_step!(parser, Matched, "NumLiteral", "nine");
                assert_step!(parser, Matched, "NoneExpr", "two");
            }
            {
                assert_step!(parser, ContinueWith, "NoneExpr", "two");
                assert_step!(parser, ContinueWith, "NumLiteral", "two");
                assert_step!(parser, Matched, "NumLiteral", "two");
                assert_step!(parser, Matched, "NoneExpr", "h");
            }
            assert_step!(parser, ContinueWith, "NoneExpr", "h");
            assert_step!(parser, Failed, "NoneExpr", "h");
            assert_step!(parser, Matched, "Sub", "h");
            assert_step!(parser, Matched, "NoneExprCont", "h");
        }
        assert_step!(parser, Matched, "Equals", "h");
        assert_step!(parser, MatchedLine, "NoneStat", "");
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
        assert_step!(parser, Start, "NoneStat", "Eq");
        assert_step!(parser, ContinueWith, "Equals", "nice");
        {
            assert_step!(parser, ContinueWith, "NoneExprCont", "sub");
            assert_step!(parser, ContinueWith, "Sub", "lit");
            {
                assert_step!(parser, ContinueWith, "NoneExpr", "lit");
                assert_step!(parser, ContinueWith, "NumLit", "one");
                assert_step!(parser, Continue, "NumLit", "zero");
                assert_step!(parser, Continue, "NumLit", "two");
                assert_step!(parser, Continue, "NumLit", "four");
                assert_step!(parser, Continue, "NumLit", "h");
                assert_step!(parser, Matched, "NumLit", "h");
                assert_step!(parser, Matched, "NoneExpr", "lit");
            }
            {
                assert_step!(parser, ContinueWith, "NoneExpr", "lit");
                assert_step!(parser, ContinueWith, "NumLit", "Nine");
                assert_step!(parser, Continue, "NumLit", "Five");
                assert_step!(parser, Continue, "NumLit", "Five");
                assert_step!(parser, Continue, "NumLit", "h");
                assert_step!(parser, Matched, "NumLit", "h");
                assert_step!(parser, Matched, "NoneExpr", "h");
            }
            assert_step!(parser, ContinueWith, "NoneExpr", "h");
            assert_step!(parser, Failed, "NoneExpr", "h");
            assert_step!(parser, Matched, "Sub", "h");
            assert_step!(parser, Matched, "NoneExprCont", "h");
        }
        assert_step!(parser, Matched, "Equals", "h");
        assert_step!(parser, MatchedLine, "NoneStat", "");
        assert_eq!(
            linq_like_writer::write_first(&parser.exprs),
            "(eq@0,1 \"nice\"@3 (sub@8,9,10 (litnum@12,13,14 1024@16$17) (litnum@36,37,38 955@40$14)))"
        );
    }

    #[test]
    fn test_parse_line_420_numbers() {
        let text =
            "eq nice mu two 2 and lit one two h lit two one h 0xf -0o15 h 0b11 h h".to_string();
        let mut binding = text.as_bytes();
        let mut parser = Parser::new(&mut binding);
        assert_step!(parser, Start, "NoneStat", "eq");
        assert_step!(parser, ContinueWith, "Equals", "nice");
        {
            assert_step!(parser, ContinueWith, "NoneExprCont", "mu");
            assert_step!(parser, ContinueWith, "Mult", "two");
            {
                assert_step!(parser, ContinueWith, "NoneExpr", "two");
                assert_step!(parser, ContinueWith, "NumLiteral", "two");
                assert_step!(parser, Matched, "NumLiteral", "two");
                assert_step!(parser, Matched, "NoneExpr", "2");
            }
            {
                assert_step!(parser, ContinueWith, "NoneExpr", "2");
                assert_step!(parser, ContinueWith, "NumLiteral", "2");
                assert_step!(parser, Matched, "NumLiteral", "2");
                assert_step!(parser, Matched, "NoneExpr", "and");
            }
            {
                assert_step!(parser, ContinueWith, "NoneExpr", "and");
                assert_step!(parser, ContinueWith, "Add", "lit");
                {
                    assert_step!(parser, ContinueWith, "NoneExpr", "lit");
                    assert_step!(parser, ContinueWith, "NumLit", "one");
                    assert_step!(parser, Continue, "NumLit", "two");
                    assert_step!(parser, Continue, "NumLit", "h");
                    assert_step!(parser, Matched, "NumLit", "h");
                    assert_step!(parser, Matched, "NoneExpr", "lit");
                }
                {
                    assert_step!(parser, ContinueWith, "NoneExpr", "lit");
                    assert_step!(parser, ContinueWith, "NumLit", "two");
                    assert_step!(parser, Continue, "NumLit", "one");
                    assert_step!(parser, Continue, "NumLit", "h");
                    assert_step!(parser, Matched, "NumLit", "h");
                    assert_step!(parser, Matched, "NoneExpr", "0xf");
                }
                {
                    assert_step!(parser, ContinueWith, "NoneExpr", "0xf");
                    assert_step!(parser, ContinueWith, "NumLiteral", "0xf");
                    assert_step!(parser, Matched, "NumLiteral", "0xf");
                    assert_step!(parser, Matched, "NoneExpr", "-0o15");
                }
                {
                    assert_step!(parser, ContinueWith, "NoneExpr", "-0o15");
                    assert_step!(parser, ContinueWith, "NumLiteral", "-0o15");
                    assert_step!(parser, Matched, "NumLiteral", "-0o15");
                    assert_step!(parser, Matched, "NoneExpr", "h");
                }
                assert_step!(parser, ContinueWith, "NoneExpr", "h");
                assert_step!(parser, Failed, "NoneExpr", "h");
                assert_step!(parser, Matched, "Add", "h");
                assert_step!(parser, Matched, "NoneExpr", "0b11");
            }
            {
                assert_step!(parser, ContinueWith, "NoneExpr", "0b11");
                assert_step!(parser, ContinueWith, "NumLiteral", "0b11");
                assert_step!(parser, Matched, "NumLiteral", "0b11");
                assert_step!(parser, Matched, "NoneExpr", "h");
            }
            assert_step!(parser, ContinueWith, "NoneExpr", "h");
            assert_step!(parser, Failed, "NoneExpr", "h");
            assert_step!(parser, Matched, "Mult", "h");
        }
        assert_step!(parser, Matched, "NoneExprCont", "h");
        assert_step!(parser, Matched, "Equals", "h");
        assert_step!(parser, MatchedLine, "NoneStat", "");
        assert_eq!(
            linq_like_writer::write_first(&parser.exprs),
            "(eq@0,1 \"nice\"@3 (mult@8,9 (litnum 2@11$3) (litnum 2@15$1) (add@17,18,19 \
                (litnum@21,22,23 12@25$7) (litnum@35,36,37 21@39$7) (litnum 15@49$3) (litnum -13@53$5)) (litnum 3@61$4)))"
        );
    }
}
