#[cfg(test)]
mod test_colors {
    use std::collections::HashSet;

    use crate::{parser::foreach, testing::*};
    use bstr::ByteSlice;
    use itertools::Itertools;
    use ntest::timeout;

    const COLOR_STR: &[u8] = include_bytes!("colors.txt");

    // fn check_color(){

    // }

    #[test]
    #[timeout(1000)]
    fn test_correct_colors_separate() {
        for line in COLOR_STR.lines() {
            let mut str = b"was mario ".to_vec();
            str.extend_from_slice(line);
            str.push(b'.');
            let no_spaces = line.replace(b" ", b"");

            let data = run_parser!(&str);
            let length = line.len();
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

                let data = run_parser!(&str);
                let length = curr_color.len();
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
    // #[test]
    // #[timeout(10000)]
    // fn test_incorrect_colors() {
    //     let mut correct = HashSet::new();
    //     let mut words = HashSet::new();
    //     for line in COLOR_STR.lines() {
    //         correct.insert(line.replace(b" ", b""));
    //         for word in COLOR_STR.split_str(b" ") {
    //             words.insert(word);
    //         }
    //     }
    //     let perms = words
    //         .into_iter()
    //         .powerset()
    //         .filter(|set| set.len() <= 5 && set.len() != 0)
    //         .flat_map(|set| {
    //             let len = set.len();
    //             set.into_iter().permutations(len)
    //         });
    //     for color in perms {
    //         let no_spaces = color
    //             .iter()
    //             .flat_map(|str| str.into_iter().cloned())
    //             .collect::<Vec<_>>();
    //         if correct.contains(&no_spaces) {
    //             println!("{}", std::str::from_utf8(&no_spaces).unwrap())
    //         }
    //     }
    // }
}
