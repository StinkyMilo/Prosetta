// use std::usize;

use crate::parser::alias::WordTrigger;

#[allow(dead_code)]
pub fn write(trigger_word_data: &Vec<WordTrigger>) -> String {
    let mut str = "".to_string();
    for statement in trigger_word_data {
        str += &format!("{{\"start\":{}, \"end\":{}, \"alias\":\"{}\"}},",statement.word_start, statement.word_end, String::from_utf8_lossy(&statement.alias_trigger.to_vec()));
    }
    str.pop();
    format!("[{}]",str)
}
