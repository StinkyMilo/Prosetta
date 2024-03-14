// use crate::commands::*;

// struct Bounds {
//     x1: usize,
//     y1: usize,
//     x2: usize,
//     y2: usize,
// }

// struct BoundsWriter {
//     bounds: Bounds,
//     first_write: bool,
// }

// impl Default for Bounds {
//     fn default() -> Self {
//         Bounds {
//             x1: 0,
//             y1: 0,
//             x2: 0,
//             y2: 0,
//         }
//     }
// }

// impl BoundsWriter {
//     pub fn new() -> Self {
//         BoundsWriter {
//             bounds: Default::default(),
//             first_write: false,
//         }
//     }
//     pub fn get(&self) -> &Bounds {
//         &self.bounds
//     }
// }

// impl BoundsWriter {
//     #[allow(dead_code)]
//     pub fn write(&mut self,exprs: &ExprArena, line_starts: &Vec<usize>) {

//         for statement in line_starts {
//             self.write_expr(exprs, *statement);
//         }
//     }
//     #[allow(dead_code)]
//     pub fn write_one(&mut self,exprs: &ExprArena) {
//         self.write_expr(exprs, 0);
//     }

//     fn write_expr(&mut self, exprs: &ExprArena, index: usize) -> String {
//         match &exprs[index] {
//             Expr::Line {
//                 x_index,
//                 y_index,
//                 x2_index,
//                 y2_index,
//                 ..
//             } => {}
//             Expr::Circle {
//                 locs,
//                 x_index,
//                 y_index,
//                 r_index,
//             } => format!(
//                 "(circle@{} {} {} {})",
//                 join_locs(&locs),
//                 write_expr(exprs, *x_index),
//                 write_expr(exprs, *y_index),
//                 write_expr(exprs, *r_index)
//             ),
//             // does not change boudns
//             _=>{}
//             //expr => panic!("found {expr:?} which has no branch"),
//         }
//     }
// }
// // ensure points
// impl BoundsWriter{
//     pub add_point(&mut self,point:(usize,usize)){

//     }
// }
