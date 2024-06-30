use crate::{
    commands::{Expr, ExprArena},
    parser::{Parser, ParserSource},
    writers::{syntax_lint::SyntaxLinter, syntax_renderers::wind_renderer::WindowsRenderer},
};

// pub fn print_test() {
//     let source = ParserSource::from_string("was test num . seven ..".into());
//     let exp1 = ExprArena {
//         vec: vec![
//             Expr::Assign {
//                 locs: vec![0, 1, 2],
//                 name_start: 4,
//                 name: "test".into(),
//                 value_index: 1,
//                 end: 22,
//             },
//             Expr::WordNum {
//                 locs: vec![9, 10, 11],
//                 str_start: 13,
//                 str: ".".into(),
//                 end: 21,
//             },
//         ],
//     };
//     let exp2 = ExprArena {
//         vec: vec![
//             Expr::Assign {
//                 locs: vec![0, 1, 2],
//                 name_start: 4,
//                 name: "test".into(),
//                 value_index: 1,
//                 end: 22,
//             },
//             Expr::WordNum {
//                 locs: vec![9, 10, 11],
//                 str_start: 15,
//                 str: "seven".into(),
//                 end: 21,
//             },
//         ],
//     };
//     let exp3 = ExprArena {
//         vec: vec![
//             Expr::Assign {
//                 locs: vec![0, 1, 2],
//                 name_start: 4,
//                 name: "test".into(),
//                 value_index: 1,
//                 end: 21,
//             },
//             Expr::MultiLitNum {
//                 locs: vec![],
//                 str_start: 15,
//                 str_length: 5,
//                 value: 7,
//                 end: usize::MAX,
//             },
//         ],
//     };
//     let exps = vec![exp1, exp2, exp3];
//     for exp in exps {
//         let mut lint = SyntaxLinter::<WindowsRenderer>::new();
//         lint.write(&exp, &[0], source.get_iter());
//         println!("{}", std::str::from_utf8(&lint.into_string()).unwrap());
//     }
// }

pub fn size_test() {

}