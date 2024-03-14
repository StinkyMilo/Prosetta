// use crate::commands::*;
// use crate::processing_writer;

// fn get_ast1() -> Vec<Stat> {
//     vec![
//         Stat::Eq {
//             name: b"inch".to_vec(),
//             value: 1
//             locs: vec![],
//             name_start: 0,
//         },
//         Expr::Num {
//             str: b"Rabbitfish".to_vec(),
//             locs: vec![],
//             str_start: 0,
//         },
//         Stat::Eq {
//             name: b"miles".to_vec(),
//             value: Box::new(Expr::Mult {
//                 a: Box::new(Expr::Var {
//                     name: b"inch".to_vec(),
//                     name_start: 0,
//                 }),
//                 b: Box::new(Expr::Var {
//                     name: b"inch".to_vec(),
//                     name_start: 0,
//                 }),
//                 locs: vec![],
//             }),
//             locs: vec![],
//             name_start: 0,
//         },
//         Stat::Eq {
//             name: b"furlongs".to_vec(),
//             value: Box::new(Expr::Mult {
//                 a: Box::new(Expr::Num {
//                     str: b"your".to_vec(),
//                     locs: vec![],
//                     str_start: 0,
//                 }),
//                 b: Box::new(Expr::Var {
//                     name: b"inch".to_vec(),
//                     name_start: 0,
//                 }),
//                 locs: vec![],
//             }),
//             locs: vec![],
//             name_start: 0,
//         },
//         Stat::Eq {
//             name: b"longer".to_vec(),
//             value: Box::new(Expr::Mult {
//                 a: Box::new(Expr::Var {
//                     name: b"miles".to_vec(),
//                     name_start: 0,
//                 }),
//                 b: Box::new(Expr::Num {
//                     str: b"as".to_vec(),
//                     locs: vec![],
//                     str_start: 0,
//                 }),
//                 locs: vec![],
//             }),
//             locs: vec![],
//             name_start: 0,
//         },
//         Stat::Circle {
//             x: Box::new(Expr::Var {
//                 name: b"longer".to_vec(),
//                 name_start: 0,
//             }),
//             y: Box::new(Expr::Var {
//                 name: b"longer".to_vec(),
//                 name_start: 0,
//             }),
//             r: Box::new(Expr::Var {
//                 name: b"longer".to_vec(),
//                 name_start: 0,
//             }),
//             locs: vec![],
//         },
//         Stat::Circle {
//             x: Box::new(Expr::Var {
//                 name: b"miles".to_vec(),
//                 name_start: 0,
//             }),
//             y: Box::new(Expr::Var {
//                 name: b"longer".to_vec(),
//                 name_start: 0,
//             }),
//             r: Box::new(Expr::Var {
//                 name: b"furlongs".to_vec(),
//                 name_start: 0,
//             }),
//             locs: vec![],
//         },
//         Stat::Circle {
//             x: Box::new(Expr::Add {
//                 a: Box::new(Expr::Var {
//                     name: b"longer".to_vec(),
//                     name_start: 0,
//                 }),
//                 b: Box::new(Expr::Var {
//                     name: b"miles".to_vec(),
//                     name_start: 0,
//                 }),
//                 locs: vec![],
//             }),
//             y: Box::new(Expr::Var {
//                 name: b"longer".to_vec(),
//                 name_start: 0,
//             }),
//             r: Box::new(Expr::Var {
//                 name: b"furlongs".to_vec(),
//                 name_start: 0,
//             }),
//             locs: vec![],
//         },
//         Stat::Line {
//             x: Box::new(Expr::Var {
//                 name: b"miles".to_vec(),
//                 name_start: 0,
//             }),
//             y: Box::new(Expr::Add {
//                 a: Box::new(Expr::Var {
//                     name: b"longer".to_vec(),
//                     name_start: 0,
//                 }),
//                 b: Box::new(Expr::Var {
//                     name: b"miles".to_vec(),
//                     name_start: 0,
//                 }),
//                 locs: vec![],
//             }),
//             x2: Box::new(Expr::Add {
//                 a: Box::new(Expr::Var {
//                     name: b"longer".to_vec(),
//                     name_start: 0,
//                 }),
//                 b: Box::new(Expr::Var {
//                     name: b"miles".to_vec(),
//                     name_start: 0,
//                 }),
//                 locs: vec![],
//             }),
//             y2: Box::new(Expr::Add {
//                 a: Box::new(Expr::Var {
//                     name: b"longer".to_vec(),
//                     name_start: 0,
//                 }),
//                 b: Box::new(Expr::Var {
//                     name: b"miles".to_vec(),
//                     name_start: 0,
//                 }),
//                 locs: vec![],
//             }),
//             locs: vec![],
//         },
//     ]
// }

// pub fn test_ast1(){
//     print!("{}",processing_writer::write(&get_ast1()));
// }