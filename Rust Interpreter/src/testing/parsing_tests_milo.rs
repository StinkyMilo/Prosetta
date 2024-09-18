// /*
// * regex
// * ^(.+?)::(.+?)\("(.+?)"\)\);( \/\/(.+?))?$
// * assert_step!\(parser, \2,"\3","\5"\);
// */
// #[cfg(test)]
// mod tests {
//     use crate::parser::*;
//     use crate::testing::*;

//     #[test]
//     fn test_parse_line_1() {
//         let text = "Equals inch innumerably. Rabbitfish hide in Hell.".to_string();
//         let mut binding = text.as_bytes();
//         let mut parser = Parser::new(&mut binding);
//         assert_step!(parser, Start, "NoneStat", "Equals");
//         assert_step!(parser, ContinueWith, "Equals", "inch");
//         {
//             assert_step!(parser, ContinueWith, "NoneExprCont", "innumerably");
//             assert_step!(parser, ContinueWith, "Num", "Rabbitfish");
//             assert_step!(parser, Matched, "Num", "Rabbitfish");
//             assert_step!(parser, Matched, "NoneExprCont", "in");
//         }
//         assert_step!(parser, Matched, "Equals", "in");
//         assert_step!(parser, MatchedLine, "NoneStat", "");
//         assert_eq!(
//             linq_like_writer::write_first(&parser.exprs),
//             "(eq@0,1 \"inch\"@7 (num@13,15,16 \"rabbitfish\"@25))"
//         );
//         assert!(parser.vars.contains(&"inch".as_bytes().to_vec()));
//     }

//     #[test]
//     fn test_parse_line_2() {
//         let text = "Equations miles across amuse you as you inch, inch again, heating, heaving."
//             .to_string();
//         let mut binding = text.as_bytes();
//         let mut parser = Parser::new(&mut binding);
//         add_vars!(parser, "inch");
//         assert_step!(parser, Start, "NoneStat", "Equations");
//         assert_step!(parser, ContinueWith, "Equals", "miles");
//         assert_step!(parser, ContinueWith, "NoneExprCont", "across");
//         {
//             assert_step!(parser, Continue, "NoneExprCont", "amuse");
//             assert_step!(parser, ContinueWith, "Mult", "you");

//             assert_step!(parser, ContinueWith, "NoneExpr", "you");
//             assert_step!(parser, Failed, "NoneExpr", "you");
//             assert_step!(parser, Continue, "Mult", "as");

//             assert_step!(parser, ContinueWith, "NoneExpr", "as");
//             assert_step!(parser, Failed, "NoneExpr", "as");
//             assert_step!(parser, Continue, "Mult", "you");

//             assert_step!(parser, ContinueWith, "NoneExpr", "you");
//             assert_step!(parser, Failed, "NoneExpr", "you");
//             assert_step!(parser, Continue, "Mult", "inch");

//             {
//                 assert_step!(parser, ContinueWith, "NoneExpr", "inch");
//                 assert_step!(parser, ContinueWith, "Var", "inch");
//                 assert_step!(parser, Matched, "Var", "inch"); // (2)
//                 assert_step!(parser, Matched, "NoneExpr", "inch"); // (2)
//             }
//             {
//                 assert_step!(parser, ContinueWith, "NoneExpr", "inch"); // (2)
//                 assert_step!(parser, ContinueWith, "Var", "inch"); // (2)
//                 assert_step!(parser, Matched, "Var", "inch"); // (2)
//                 assert_step!(parser, Matched, "NoneExpr", "again");
//             }

//             assert_step!(parser, ContinueWith, "NoneExpr", "again");
//             assert_step!(parser, Failed, "NoneExpr", "again");
//             assert_step!(parser, Continue, "Mult", "heating");

//             assert_step!(parser, ContinueWith, "NoneExpr", "heating");
//             assert_step!(parser, Failed, "NoneExpr", "heating");

//             assert_step!(parser, Matched, "Mult", "heating");
//             assert_step!(parser, Matched, "NoneExprCont", "heaving");
//         }
//         assert_step!(parser, Matched, "Equals", "heaving");
//         assert_step!(parser, MatchedLine, "NoneStat", "");

//         assert_eq!(
//             linq_like_writer::write_first(&parser.exprs),
//             "(eq@0,1 \"miles\"@10 (mult@24,25 (var \"inch\"@40) (var \"inch\"@46)))"
//         );
//     }

//     #[test]
//     fn test_parse_line_2_no_var() {
//         let text = "Equations miles across amuse you as you inch, inch again, heating, heaving."
//             .to_string();
//         let mut binding = text.as_bytes();
//         let mut parser = Parser::new(&mut binding);
//         assert_step!(parser, Start, "NoneStat", "Equations");
//         assert_step!(parser, ContinueWith, "Equals", "miles");
//         assert_step!(parser, ContinueWith, "NoneExprCont", "across");
//         {
//             assert_step!(parser, Continue, "NoneExprCont", "amuse");
//             assert_step!(parser, ContinueWith, "Mult", "you");
//             assert_step!(parser, ContinueWith, "NoneExpr", "you");
//             assert_step!(parser, Failed, "NoneExpr", "you");
//             assert_step!(parser, Continue, "Mult", "as");

//             assert_step!(parser, ContinueWith, "NoneExpr", "as");
//             assert_step!(parser, Failed, "NoneExpr", "as");
//             assert_step!(parser, Continue, "Mult", "you");

//             assert_step!(parser, ContinueWith, "NoneExpr", "you");
//             assert_step!(parser, Failed, "NoneExpr", "you");
//             assert_step!(parser, Continue, "Mult", "inch");

//             assert_step!(parser, ContinueWith, "NoneExpr", "inch");
//             assert_step!(parser, Failed, "NoneExpr", "inch");
//             assert_step!(parser, Failed, "Mult", "inch");
//         }

//         assert_step!(parser, Continue, "NoneExprCont", "you");
//         assert_step!(parser, Continue, "NoneExprCont", "as");
//         assert_step!(parser, Continue, "NoneExprCont", "you");
//         assert_step!(parser, Continue, "NoneExprCont", "inch");
//         assert_step!(parser, Continue, "NoneExprCont", "inch");
//         assert_step!(parser, Continue, "NoneExprCont", "again");
//         assert_step!(parser, Continue, "NoneExprCont", "heating");
//         assert_step!(parser, Continue, "NoneExprCont", "heaving");
//         assert_step!(parser, Continue, "NoneExprCont", "");
//         assert_step!(parser, Failed, "NoneExprCont", "");
//         assert_step!(parser, Failed, "Equals", "miles");

//         assert_step!(parser, Continue, "NoneStat", "miles");
//         assert_step!(parser, Continue, "NoneStat", "across");
//         assert_step!(parser, Continue, "NoneStat", "amuse");
//         assert_step!(parser, Continue, "NoneStat", "you");
//         assert_step!(parser, Continue, "NoneStat", "as");
//         assert_step!(parser, Continue, "NoneStat", "you");
//         assert_step!(parser, Continue, "NoneStat", "inch");
//         assert_step!(parser, Continue, "NoneStat", "inch");
//         assert_step!(parser, Continue, "NoneStat", "again");
//         assert_step!(parser, Continue, "NoneStat", "heating");
//         assert_step!(parser, Continue, "NoneStat", "heaving");
//         assert_step!(parser, Continue, "NoneStat", "");
//         assert_step!(parser, FailedLine, "NoneStat", "");

//         assert_eq!(parser.exprs.vec.len(), 0);
//     }

//     #[test]
//     fn test_parse_line_3() {
//         let text = "Equate furlongs to ambiguity; disencumber your heels. Inch farther, farther."
//             .to_string();
//         let mut binding = text.as_bytes();
//         let mut parser = Parser::new(&mut binding);
//         add_vars!(parser, "inch", "miles");
//         assert_step!(parser, Start, "NoneStat", "Equate");
//         assert_step!(parser, ContinueWith, "Equals", "furlongs");
//         assert_step!(parser, ContinueWith, "NoneExprCont", "to");
//         {
//             assert_step!(parser, Continue, "NoneExprCont", "ambiguity");
//             assert_step!(parser, ContinueWith, "Mult", "disencumber");
//             {
//                 assert_step!(parser, ContinueWith, "NoneExpr", "disencumber");
//                 assert_step!(parser, ContinueWith, "Num", "your");
//                 assert_step!(parser, Matched, "Num", "your");
//                 assert_step!(parser, Matched, "NoneExpr", "Inch");
//             }
//             assert_step!(parser, ContinueWith, "NoneExpr", "Inch");
//             assert_step!(parser, ContinueWith, "Var", "Inch");
//             assert_step!(parser, Matched, "Var", "Inch");
//             assert_step!(parser, Matched, "NoneExpr", "farther");

//             assert_step!(parser, ContinueWith, "NoneExpr", "farther");
//             assert_step!(parser, Failed, "NoneExpr", "farther"); //(2)

//             assert_step!(parser, Matched, "Mult", "farther"); //(2)
//             assert_step!(parser, Matched, "NoneExprCont", "farther"); //(2)
//         }
//         assert_step!(parser, Matched, "Equals", "farther"); //(2)
//         assert_step!(parser, MatchedLine, "NoneStat", "");
//         assert_eq!(
//             linq_like_writer::write_first(&parser.exprs),
//             "(eq@0,1 \"furlongs\"@7 (mult@20,24 (num@34,36,37 \"your\"@42) (var \"inch\"@54)))"
//         )
//     }

//     #[test]
//     fn test_parse_line_4() {
//         let text =
//             "Equip longer armour. For miles, you swing your pendulum as you head home, harrowed."
//                 .to_string();
//         let mut binding = text.as_bytes();
//         let mut parser = Parser::new(&mut binding);
//         add_vars!(parser, "inch", "miles", "furlongs");
//         assert_step!(parser, Start, "NoneStat", "Equip");
//         assert_step!(parser, ContinueWith, "Equals", "longer");
//         {
//             assert_step!(parser, ContinueWith, "NoneExprCont", "armour");
//             assert_step!(parser, ContinueWith, "Mult", "For");
//             assert_step!(parser, ContinueWith, "NoneExpr", "For");
//             assert_step!(parser, Failed, "NoneExpr", "For");
//             assert_step!(parser, Continue, "Mult", "miles");

//             {
//                 assert_step!(parser, ContinueWith, "NoneExpr", "miles");
//                 assert_step!(parser, ContinueWith, "Var", "miles");
//                 assert_step!(parser, Matched, "Var", "miles");
//                 assert_step!(parser, Matched, "NoneExpr", "you");
//             }

//             assert_step!(parser, ContinueWith, "NoneExpr", "you");
//             assert_step!(parser, Failed, "NoneExpr", "you");
//             assert_step!(parser, Continue, "Mult", "swing");

//             assert_step!(parser, ContinueWith, "NoneExpr", "swing");
//             assert_step!(parser, Failed, "NoneExpr", "swing");
//             assert_step!(parser, Continue, "Mult", "your");

//             assert_step!(parser, ContinueWith, "NoneExpr", "your");
//             assert_step!(parser, Failed, "NoneExpr", "your");
//             assert_step!(parser, Continue, "Mult", "pendulum");
//             {
//                 assert_step!(parser, ContinueWith, "NoneExpr", "pendulum");
//                 assert_step!(parser, ContinueWith, "Num", "as");
//                 assert_step!(parser, Matched, "Num", "as");
//                 assert_step!(parser, Matched, "NoneExpr", "home");
//             }
//             assert_step!(parser, ContinueWith, "NoneExpr", "home");
//             assert_step!(parser, Failed, "NoneExpr", "home");
//             assert_step!(parser, Matched, "Mult", "home");
//             assert_step!(parser, Matched, "NoneExprCont", "harrowed");
//         }
//         assert_step!(parser, Matched, "Equals", "harrowed");
//         assert_step!(parser, MatchedLine, "NoneStat", "");

//         assert_eq!(
//             linq_like_writer::write_first(&parser.exprs),
//             "(eq@0,1 \"longer\"@6 (mult@15,17 (var \"miles\"@25) (num@49,51,54 \"as\"@56)))"
//         )
//     }

//     #[test]
//     fn test_parse_line_5() {
//         let text =
//             "Pick your pace. Go longer,  longer,  yet longer. Take the road below you.".to_string();
//         let mut binding = text.as_bytes();
//         let mut parser = Parser::new(&mut binding);
//         add_vars!(parser, "inch", "miles", "furlongs", "longer");

//         assert_step!(parser, Start, "NoneStat", "Pick");
//         assert_step!(parser, ContinueWith, "Circle", "your");
//         assert_step!(parser, ContinueWith, "NoneExprCont", "your");
//         assert_step!(parser, Continue, "NoneExprCont", "pace");
//         assert_step!(parser, Continue, "NoneExprCont", "Go");
//         {
//             assert_step!(parser, Continue, "NoneExprCont", "longer");
//             assert_step!(parser, ContinueWith, "Var", "longer");
//             assert_step!(parser, Matched, "Var", "longer");
//             assert_step!(parser, Matched, "NoneExprCont", "longer");
//         }
//         {
//             assert_step!(parser, ContinueWith, "NoneExprCont", "longer");
//             assert_step!(parser, ContinueWith, "Var", "longer");
//             assert_step!(parser, Matched, "Var", "longer");
//             assert_step!(parser, Matched, "NoneExprCont", "yet");
//         }
//         assert_step!(parser, ContinueWith, "NoneExprCont", "yet");
//         {
//             assert_step!(parser, Continue, "NoneExprCont", "longer");
//             assert_step!(parser, ContinueWith, "Var", "longer");
//             assert_step!(parser, Matched, "Var", "longer");
//             assert_step!(parser, Matched, "NoneExprCont", "Take");
//         }
//         assert_step!(parser, Matched, "Circle", "Take");
//         assert_step!(parser, MatchedLine, "NoneStat", "road");

//         assert_eq!(
//             linq_like_writer::write_first(&parser.exprs),
//             "(circle@0,1 (var \"longer\"@19) (var \"longer\"@28) (var \"longer\"@41))"
//         )
//     }

//     #[test]
//     fn test_parse_line_6() {
//         let text = "Point East. but few miles longer, to furlongs more ahead.".to_string();
//         let mut binding = text.as_bytes();
//         let mut parser = Parser::new(&mut binding);

//         add_vars!(parser, "inch", "miles", "furlongs", "longer");
//         assert_step!(parser, Start, "NoneStat", "Point");
//         assert_step!(parser, ContinueWith, "Circle", "East");
//         assert_step!(parser, ContinueWith, "NoneExprCont", "East");
//         assert_step!(parser, Continue, "NoneExprCont", "but");
//         assert_step!(parser, Continue, "NoneExprCont", "few");
//         {
//             assert_step!(parser, Continue, "NoneExprCont", "miles");
//             assert_step!(parser, ContinueWith, "Var", "miles");
//             assert_step!(parser, Matched, "Var", "miles");
//             assert_step!(parser, Matched, "NoneExprCont", "longer");
//         }
//         {
//             assert_step!(parser, ContinueWith, "NoneExprCont", "longer");
//             assert_step!(parser, ContinueWith, "Var", "longer");
//             assert_step!(parser, Matched, "Var", "longer");
//             assert_step!(parser, Matched, "NoneExprCont", "to");
//         }
//         assert_step!(parser, ContinueWith, "NoneExprCont", "to");
//         {
//             assert_step!(parser, Continue, "NoneExprCont", "furlongs");
//             assert_step!(parser, ContinueWith, "Var", "furlongs");
//             assert_step!(parser, Matched, "Var", "furlongs");
//             assert_step!(parser, Matched, "NoneExprCont", "more");
//         }

//         assert_step!(parser, Matched, "Circle", "more");
//         assert_step!(parser, MatchedLine, "NoneStat", "");

//         assert_eq!(
//             linq_like_writer::write_first(&parser.exprs),
//             "(circle@0,2 (var \"miles\"@20) (var \"longer\"@26) (var \"furlongs\"@37))"
//         )
//     }

//     #[test]
//     fn test_parse_line_7() {
//         let text =
//             "Point West. And no longer do miles hang before you. But longer furlongs await ahead."
//                 .to_string();
//         let mut binding = text.as_bytes();
//         let mut parser = Parser::new(&mut binding);

//         add_vars!(parser, "inch", "miles", "furlongs", "longer");
//         assert_step!(parser, Start, "NoneStat", "Point");
//         assert_step!(parser, ContinueWith, "Circle", "West");
//         {
//             assert_step!(parser, ContinueWith, "NoneExprCont", "West");
//             assert_step!(parser, Continue, "NoneExprCont", "And");
//             assert_step!(parser, ContinueWith, "Add", "no");
//             assert_step!(parser, ContinueWith, "NoneExpr", "no");
//             assert_step!(parser, Failed, "NoneExpr", "no");
//             assert_step!(parser, Continue, "Add", "longer");
//             {
//                 assert_step!(parser, ContinueWith, "NoneExpr", "longer");
//                 assert_step!(parser, ContinueWith, "Var", "longer");
//                 assert_step!(parser, Matched, "Var", "longer");
//                 assert_step!(parser, Matched, "NoneExpr", "do");
//             }
//             assert_step!(parser, ContinueWith, "NoneExpr", "do");
//             assert_step!(parser, Failed, "NoneExpr", "do");
//             assert_step!(parser, Continue, "Add", "miles");
//             {
//                 assert_step!(parser, ContinueWith, "NoneExpr", "miles");
//                 assert_step!(parser, ContinueWith, "Var", "miles");
//                 assert_step!(parser, Matched, "Var", "miles");
//                 assert_step!(parser, Matched, "NoneExpr", "hang");
//             }
//             assert_step!(parser, ContinueWith, "NoneExpr", "hang");
//             assert_step!(parser, Failed, "NoneExpr", "hang");
//             assert_step!(parser, Matched, "Add", "hang");
//             assert_step!(parser, Matched, "NoneExprCont", "before");
//         }
//         assert_step!(parser, ContinueWith, "NoneExprCont", "before");
//         assert_step!(parser, Continue, "NoneExprCont", "you");
//         assert_step!(parser, Continue, "NoneExprCont", "But");
//         {
//             assert_step!(parser, Continue, "NoneExprCont", "longer");
//             assert_step!(parser, ContinueWith, "Var", "longer");
//             assert_step!(parser, Matched, "Var", "longer");
//             assert_step!(parser, Matched, "NoneExprCont", "furlongs");
//         }
//         {
//             assert_step!(parser, ContinueWith, "NoneExprCont", "furlongs");
//             assert_step!(parser, ContinueWith, "Var", "furlongs");
//             assert_step!(parser, Matched, "Var", "furlongs");
//             assert_step!(parser, Matched, "NoneExprCont", "await");
//         }
//         assert_step!(parser, Matched, "Circle", "await");
//         assert_step!(parser, MatchedLine, "NoneStat", "");

//         assert_eq!(
//             linq_like_writer::write_first(&parser.exprs),
//             "(circle@0,2 (add@12,13,14 (var \"longer\"@19) (var \"miles\"@29)) (var \"longer\"@56) (var \"furlongs\"@63))"
//         )
//     }

//     #[test]
//     fn test_parse_line_8() {
//         let text =
//             "Like miles before and longer miles after, home remains away from view. And longer miles await ahead. And even longer miles stretch behind."
//                 .to_string();
//         let mut binding = text.as_bytes();
//         let mut parser = Parser::new(&mut binding);
//         add_vars!(parser, "inch", "miles", "furlongs", "longer");
//         assert_step!(parser, Start, "NoneStat", "Like");
//         assert_step!(parser, ContinueWith, "Line", "miles");
//         {
//             assert_step!(parser, ContinueWith, "NoneExprCont", "miles");
//             assert_step!(parser, ContinueWith, "Var", "miles");
//             assert_step!(parser, Matched, "Var", "miles");
//             assert_step!(parser, Matched, "NoneExprCont", "before");
//         }
//         assert_step!(parser, ContinueWith, "NoneExprCont", "before");
//         {
//             assert_step!(parser, Continue, "NoneExprCont", "and");
//             assert_step!(parser, ContinueWith, "Add", "longer");
//             {
//                 assert_step!(parser, ContinueWith, "NoneExpr", "longer");
//                 assert_step!(parser, ContinueWith, "Var", "longer");
//                 assert_step!(parser, Matched, "Var", "longer");
//                 assert_step!(parser, Matched, "NoneExpr", "miles");
//             }
//             {
//                 assert_step!(parser, ContinueWith, "NoneExpr", "miles");
//                 assert_step!(parser, ContinueWith, "Var", "miles");
//                 assert_step!(parser, Matched, "Var", "miles");
//                 assert_step!(parser, Matched, "NoneExpr", "after");
//             }
//             assert_step!(parser, ContinueWith, "NoneExpr", "after");
//             assert_step!(parser, Failed, "NoneExpr", "after");
//             assert_step!(parser, Continue, "Add", "home");
//             assert_step!(parser, ContinueWith, "NoneExpr", "home");
//             assert_step!(parser, Failed, "NoneExpr", "home");
//             assert_step!(parser, Matched, "Add", "home");
//             assert_step!(parser, Matched, "NoneExprCont", "remains");
//         }
//         assert_step!(parser, ContinueWith, "NoneExprCont", "remains");
//         assert_step!(parser, Continue, "NoneExprCont", "away");
//         assert_step!(parser, Continue, "NoneExprCont", "from");
//         assert_step!(parser, Continue, "NoneExprCont", "view");
//         {
//             assert_step!(parser, Continue, "NoneExprCont", "And");
//             assert_step!(parser, ContinueWith, "Add", "longer");
//             {
//                 assert_step!(parser, ContinueWith, "NoneExpr", "longer");
//                 assert_step!(parser, ContinueWith, "Var", "longer");
//                 assert_step!(parser, Matched, "Var", "longer");
//                 assert_step!(parser, Matched, "NoneExpr", "miles");
//             }
//             {
//                 assert_step!(parser, ContinueWith, "NoneExpr", "miles");
//                 assert_step!(parser, ContinueWith, "Var", "miles");
//                 assert_step!(parser, Matched, "Var", "miles");
//                 assert_step!(parser, Matched, "NoneExpr", "await");
//             }
//             assert_step!(parser, ContinueWith, "NoneExpr", "await");
//             assert_step!(parser, Failed, "NoneExpr", "await");
//             assert_step!(parser, Continue, "Add", "ahead");
//             assert_step!(parser, ContinueWith, "NoneExpr", "ahead");
//             assert_step!(parser, Failed, "NoneExpr", "ahead");
//             assert_step!(parser, Matched, "Add", "ahead");
//             assert_step!(parser, Matched, "NoneExprCont", "And");
//         }
//         {
//             assert_step!(parser, ContinueWith, "NoneExprCont", "And");
//             assert_step!(parser, ContinueWith, "Add", "even");
//             assert_step!(parser, ContinueWith, "NoneExpr", "even");
//             assert_step!(parser, Failed, "NoneExpr", "even");
//             assert_step!(parser, Continue, "Add", "longer");
//             {
//                 assert_step!(parser, ContinueWith, "NoneExpr", "longer");
//                 assert_step!(parser, ContinueWith, "Var", "longer");
//                 assert_step!(parser, Matched, "Var", "longer");
//                 assert_step!(parser, Matched, "NoneExpr", "miles");
//             }
//             {
//                 assert_step!(parser, ContinueWith, "NoneExpr", "miles");
//                 assert_step!(parser, ContinueWith, "Var", "miles");
//                 assert_step!(parser, Matched, "Var", "miles");
//                 assert_step!(parser, Matched, "NoneExpr", "stretch");
//             }
//             assert_step!(parser, ContinueWith, "NoneExpr", "stretch");
//             assert_step!(parser, Failed, "NoneExpr", "stretch");
//             assert_step!(parser, Matched, "Add", "stretch");
//             assert_step!(parser, Matched, "NoneExprCont", "behind");
//         }
//         assert_step!(parser, Matched, "Line", "behind");
//         assert_step!(parser, MatchedLine, "NoneStat", "");

//         assert_eq!(
//             linq_like_writer::write_first(&parser.exprs),
//             "(line@0,1 (var \"miles\"@5) (add@18,19,20 (var \"longer\"@22) (var \"miles\"@29)) (add@71,72,73 (var \"longer\"@75) \
//             (var \"miles\"@82)) (add@101,102,103 (var \"longer\"@110) (var \"miles\"@117)))"
//         )
//     }
//     #[test]
//     fn test_parse_line_1_and_2() {
//         let text = "Equals inch innumerably. Rabbitfish hide in Hell.\n\
//                 Equations miles across amuse you as you inch, inch again, heating, heaving."
//             .to_string();
//         let mut binding = text.as_bytes();
//         let mut parser = Parser::new(&mut binding);
//         {
//             assert_step!(parser, Start, "NoneStat", "Equals");
//             assert_step!(parser, ContinueWith, "Equals", "inch");
//             assert_step!(parser, ContinueWith, "NoneExprCont", "innumerably");
//             assert_step!(parser, ContinueWith, "Num", "Rabbitfish");
//             assert_step!(parser, Matched, "Num", "Rabbitfish");
//             assert_step!(parser, Matched, "NoneExprCont", "in");
//             assert_step!(parser, Matched, "Equals", "in");
//             assert_step!(parser, MatchedLine, "NoneStat", "");
//         }
//         {
//             assert_step!(parser, Start, "NoneStat", "ch");
//             assert_step!(parser, ContinueWith, "Equals", "miles");
//             assert_step!(parser, ContinueWith, "NoneExprCont", "across");
//             assert_step!(parser, Continue, "NoneExprCont", "amuse");
//             assert_step!(parser, ContinueWith, "Mult", "you");
//             assert_step!(parser, ContinueWith, "NoneExpr", "you");
//             assert_step!(parser, Failed, "NoneExpr", "you");
//             assert_step!(parser, Continue, "Mult", "as");
//             assert_step!(parser, ContinueWith, "NoneExpr", "as");
//             assert_step!(parser, Failed, "NoneExpr", "as");
//             assert_step!(parser, Continue, "Mult", "you");
//             assert_step!(parser, ContinueWith, "NoneExpr", "you");
//             assert_step!(parser, Failed, "NoneExpr", "you");
//             assert_step!(parser, Continue, "Mult", "inch");
//             assert_step!(parser, ContinueWith, "NoneExpr", "inch");
//             assert_step!(parser, ContinueWith, "Var", "inch");
//             assert_step!(parser, Matched, "Var", "inch");
//             assert_step!(parser, Matched, "NoneExpr", "inch");
//             assert_step!(parser, ContinueWith, "NoneExpr", "inch");
//             assert_step!(parser, ContinueWith, "Var", "inch");
//             assert_step!(parser, Matched, "Var", "inch");
//             assert_step!(parser, Matched, "NoneExpr", "again");
//             assert_step!(parser, ContinueWith, "NoneExpr", "again");
//             assert_step!(parser, Failed, "NoneExpr", "again");
//             assert_step!(parser, Continue, "Mult", "heating");
//             assert_step!(parser, ContinueWith, "NoneExpr", "heating");
//             assert_step!(parser, Failed, "NoneExpr", "heating");
//             assert_step!(parser, Matched, "Mult", "heating");
//             assert_step!(parser, Matched, "NoneExprCont", "heaving");
//             assert_step!(parser, Matched, "Equals", "heaving");
//             assert_step!(parser, MatchedLine, "NoneStat", "");
//         }
//         assert_eq!(
//             linq_like_writer::write(&parser.exprs, &parser.stat_starts),
//             "(eq@0,1 \"inch\"@7 (num@13,15,16 \"rabbitfish\"@25))\n\
//             (eq@50,51 \"miles\"@60 (mult@74,75 (var \"inch\"@90) (var \"inch\"@96)))\n"
//         )
//     }
// }
