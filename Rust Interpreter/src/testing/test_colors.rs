#![cfg(test)]

use crate::testing::*;
use bstr::ByteSlice;
use itertools::Itertools;
use ntest::timeout;

const COLOR_STR: &[u8] = include_bytes!("colors.txt");

#[test]
#[timeout(1000)]
fn test_correct_colors_separate() {
    for line in COLOR_STR.lines() {
        let mut str = b"was mario ".to_vec();
        str.extend_from_slice(line);
        str.push(b'.');
        let no_spaces = line.replace(b" ", b"");

        let length = line.len();
        let data = run_parser!(&str);
        check_lisp!(
            data,
            format!(
                "(assign@0,1,2${} \"mario\"@4 (litcol {}@10$${}))",
                length + 10,
                std::str::from_utf8(&no_spaces).unwrap(),
                length
            ),
            format!("testing: {}", std::str::from_utf8(line).unwrap())
        );
    }
}

#[test]
#[timeout(1000)]
fn test_correct_colors_together() {
    for line in COLOR_STR.lines() {
        let spaces = line.find_iter(b" ").powerset();
        for curr_spaces in spaces {
            let mut str = b"was mario ".to_vec();
            let mut curr_color = line.to_vec();
            for space in curr_spaces.into_iter().rev() {
                curr_color.remove(space);
            }
            str.extend_from_slice(&curr_color);
            str.push(b'.');
            let no_spaces = line.replace(b" ", b"");

            let length = curr_color.len();
            let data = run_parser!(&str);
            check_lisp!(
                data,
                format!(
                    "(assign@0,1,2${} \"mario\"@4 (litcol {}@10$${}))",
                    length + 10,
                    std::str::from_utf8(&no_spaces).unwrap(),
                    length
                ),
                format!("testing: {}", std::str::from_utf8(&curr_color).unwrap())
            );
        }
    }
}

//     fn test_incorrect_colors(num: u8) {
//         let mut correct = HashSet::new();
//         let mut words = HashSet::new();
//         for line in COLOR_STR.lines() {
//             correct.insert(line.replace(b" ", b""));
//             for word in line.split_str(b" ") {
//                 words.insert(word);
//             }
//         }
//         println!(
//             "words: {:?}",
//             correct
//                 .iter()
//                 .map(|str| std::str::from_utf8(str).unwrap())
//                 .collect::<Vec<_>>()
//         );
//         let perms = words.iter().permutations(num.into());
//         let perms_vec = perms.clone().count();
//         println!("length: {}", perms_vec);
//         for (index, color) in perms.enumerate() {
//             let no_spaces = color
//                 .iter()
//                 .flat_map(|str| str.into_iter().cloned())
//                 .collect::<Vec<_>>();
//             if !correct.contains(&no_spaces) {
//                 println!("{}:{}", index, std::str::from_utf8(&no_spaces).unwrap())
//             }
//         }
//     }

//     #[test]
//     #[timeout(1000)]
//     fn test_incorrect_colors_1() {
//         test_incorrect_colors(1);
//     }
// }
