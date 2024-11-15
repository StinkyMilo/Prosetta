#![cfg(test)]
use crate::parser::*;
use crate::testing::test_lib::*;

// find_word_end
#[test]
fn test_find_word_end_after() {
    assert_eq!(
        find_word_end(&new_slice(" a b c", 0), 0),
        new_slice(" a b c", 0)
    );
    assert_eq!(
        find_word_end(&new_slice(" a b c", 0), 1),
        new_slice(" b c", 2)
    );
    assert_eq!(
        find_word_end(&new_slice(" a b c", 0), 2),
        new_slice(" b c", 2)
    );
    assert_eq!(
        find_word_end(&new_slice(" a b c", 0), 3),
        new_slice(" c", 4)
    );
}

#[test]
fn test_find_word_end_fails() {
    assert_eq!(find_word_end(&new_slice("", 0), 0), new_slice("", 0));
    assert_eq!(find_word_end(&new_slice(" ", 0), 0), new_slice(" ", 0));
    assert_eq!(find_word_end(&new_slice("a", 0), 0), new_slice("", 1));
    assert_eq!(find_word_end(&new_slice("a ", 0), 0), new_slice(" ", 1));
    assert_eq!(find_word_end(&new_slice(" a", 0), 0), new_slice(" a", 0));
}

#[test]
fn test_find_word_end_out() {
    assert_eq!(find_word_end(&new_slice("a ", 0), 2), new_slice("", 2));
    assert_eq!(find_word_end(&new_slice("a  ", 0), 3), new_slice("", 3));
    assert_eq!(find_word_end(&new_slice("a  ", 0), 4), new_slice("", 3));
}

// find_end_close
#[test]
fn test_find_close_slice_only() {
    assert_eq!(
        find_close_slice(&new_slice(".", 0), 0),
        Some((new_slice(".", 0), new_slice("", 1)))
    );
    assert_eq!(
        find_close_slice(&new_slice("!", 0), 0),
        Some((new_slice("!", 0), new_slice("", 1)))
    );
    assert_eq!(
        find_close_slice(&new_slice("...", 0), 0),
        Some((new_slice("...", 0), new_slice("", 3)))
    );
}
#[test]
fn test_find_close_slice_after() {
    assert_eq!(
        find_close_slice(&new_slice("a . b ", 0), 1),
        Some((new_slice(".", 2), new_slice(" b ", 3)))
    );
    assert_eq!(
        find_close_slice(&new_slice("a ! b ", 0), 0),
        Some((new_slice("!", 2), new_slice(" b ", 3)))
    );
    assert_eq!(
        find_close_slice(&new_slice("a ... b ", 0), 0),
        Some((new_slice("...", 2), new_slice(" b ", 5)))
    );
}
#[test]
fn test_find_close_slice_touching_after() {
    assert_eq!(
        find_close_slice(&new_slice(" a .b c", 0), 0),
        Some((new_slice(".", 3), new_slice("b c", 4)))
    );
    assert_eq!(
        find_close_slice(&new_slice(" a !b c", 0), 0),
        Some((new_slice("!", 3), new_slice("b c", 4)))
    );
    assert_eq!(
        find_close_slice(&new_slice(" a ...b c", 0), 0),
        Some((new_slice("...", 3), new_slice("b c", 6)))
    );
}
#[test]
fn test_find_close_slice_touching_before() {
    assert_eq!(
        find_close_slice(&new_slice(" a. b c", 0), 0),
        Some((new_slice(".", 2), new_slice(" b c", 3)))
    );
    assert_eq!(
        find_close_slice(&new_slice(" a! b c", 0), 0),
        Some((new_slice("!", 2), new_slice(" b c", 3)))
    );
    assert_eq!(
        find_close_slice(&new_slice(" a... b c", 0), 0),
        Some((new_slice("...", 2), new_slice(" b c", 5)))
    );
}
#[test]
fn test_find_close_slice_touching_both() {
    assert_eq!(
        find_close_slice(&new_slice("a b.c d", 0), 0),
        Some((new_slice(".", 3), new_slice("c d", 4)))
    );
    assert_eq!(
        find_close_slice(&new_slice("a b!c d", 0), 0),
        Some((new_slice("!", 3), new_slice("c d", 4)))
    );
    assert_eq!(
        find_close_slice(&new_slice("a b...c d", 0), 0),
        Some((new_slice("...", 3), new_slice("c d", 6)))
    );
}
#[test]
fn test_find_close_slice_newline() {
    assert_eq!(
        find_close_slice(&new_slice("\n . a", 0), 0),
        Some((new_slice(".", 2), new_slice(" a", 3)))
    );
    assert_eq!(
        find_close_slice(&new_slice("\n a .", 0), 0),
        Some((new_slice(".", 4), new_slice("", 5)))
    );
    assert_eq!(
        find_close_slice(&new_slice("a b \n c . d", 0), 0),
        Some((new_slice(".", 8), new_slice(" d", 9)))
    );
}

#[test]
fn test_find_end_close_fails() {
    assert_eq!(find_close_slice(&new_slice("", 0), 0), None);
    assert_eq!(find_close_slice(&new_slice("a b c d", 0), 0), None);
}

#[test]
fn test_find_end_close_out() {
    assert_eq!(find_close_slice(&new_slice("a ", 0), 2), None);
    assert_eq!(find_close_slice(&new_slice("a ", 0), 3), None);
    assert_eq!(find_close_slice(&new_slice("a ", 0), 4), None);
}

// get_next_slice
#[test]
fn test_get_next_slice_space_end() {
    assert_eq!(
        get_next_slice(&new_slice("asdf   ", 2), 0),
        (new_slice("asdf", 2), new_slice("   ", 6))
    );
    assert_eq!(
        get_next_slice(&new_slice(" asdf  ", 1), 0),
        (new_slice("asdf", 2), new_slice("  ", 6))
    );
    assert_eq!(
        get_next_slice(&new_slice("  asdf ", 0), 0),
        (new_slice("asdf", 2), new_slice(" ", 6))
    );
}

#[test]
fn test_get_next_slice_no_space_end() {
    assert_eq!(
        get_next_slice(&new_slice("asdf", 2), 0),
        (new_slice("asdf", 2), new_slice("", 6))
    );

    assert_eq!(
        get_next_slice(&new_slice(" asdf", 2), 0),
        (new_slice("asdf", 3), new_slice("", 7))
    );

    assert_eq!(
        get_next_slice(&new_slice("  asdf", 2), 0),
        (new_slice("asdf", 4), new_slice("", 8))
    );
}
#[test]
fn test_get_next_slice_fails() {
    assert_eq!(
        get_next_slice(&new_slice("", 2), 0),
        (new_slice("", 2), new_slice("", 2))
    );
    assert_eq!(
        get_next_slice(&new_slice(" ", 4), 0),
        (new_slice("", 5), new_slice("", 5))
    );
    assert_eq!(
        get_next_slice(&new_slice(" ", 8), 0),
        (new_slice("", 9), new_slice("", 9))
    );
}

#[test]
fn test_get_next_slice_out() {
    assert_eq!(
        get_next_slice(&new_slice("a ", 0), 2),
        (new_slice("", 2), new_slice("", 2))
    );
    assert_eq!(
        get_next_slice(&new_slice("a ", 0), 3),
        (new_slice("", 2), new_slice("", 2))
    );
    assert_eq!(
        get_next_slice(&new_slice("a ", 0), 4),
        (new_slice("", 2), new_slice("", 2))
    );
}

#[test]
fn test_get_next_slice_close() {
    assert_eq!(
        get_next_slice(&new_slice(".a", 2), 0),
        (new_slice(".", 2), new_slice("a", 3))
    );
    assert_eq!(
        get_next_slice(&new_slice(" .a ", 1), 0),
        (new_slice(".", 2), new_slice("a ", 3))
    );
    assert_eq!(
        get_next_slice(&new_slice("  .a  ", 0), 0),
        (new_slice(".", 2), new_slice("a  ", 3))
    );
}

#[test]
fn test_get_next_slice_double_close() {
    assert_eq!(
        get_next_slice(&new_slice("..a", 2), 0),
        (new_slice(".", 2), new_slice(".a", 3))
    );
    assert_eq!(
        get_next_slice(&new_slice(" ..a ", 1), 0),
        (new_slice(".", 2), new_slice(".a ", 3))
    );
    assert_eq!(
        get_next_slice(&new_slice("  ..a  ", 0), 0),
        (new_slice(".", 2), new_slice(".a  ", 3))
    );
}
#[test]
fn test_get_next_slice_triple_close() {
    assert_eq!(
        get_next_slice(&new_slice("...a", 0), 0),
        (new_slice("...", 0), new_slice("a", 3))
    );
    assert_eq!(
        get_next_slice(&new_slice(" ...a ", 0), 0),
        (new_slice("...", 1), new_slice("a ", 4))
    );
    assert_eq!(
        get_next_slice(&new_slice("  ...a  ", 0), 0),
        (new_slice("...", 2), new_slice("a  ", 5))
    );
}
#[test]
fn test_get_next_slice_close_only() {
    assert_eq!(
        get_next_slice(&new_slice(".", 0), 0),
        (new_slice(".", 0), new_slice("", 1))
    );
    assert_eq!(
        get_next_slice(&new_slice("!", 0), 0),
        (new_slice("!", 0), new_slice("", 1))
    );
    assert_eq!(
        get_next_slice(&new_slice("...", 0), 0),
        (new_slice("...", 0), new_slice("", 3))
    );
}
#[test]
fn test_get_next_slice_newline() {
    assert_eq!(
        get_next_slice(&new_slice("a\na", 0), 0),
        (new_slice("a", 0), new_slice("\na", 1))
    );
    assert_eq!(
        get_next_slice(&new_slice("\naa", 0), 0),
        (new_slice("aa", 1), new_slice("", 3))
    );
    assert_eq!(
        get_next_slice(&new_slice("\n a a", 0), 0),
        (new_slice("a", 2), new_slice(" a", 3))
    );
}
