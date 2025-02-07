// use std::usize;

use crate::parser::alias::{WordTrigger, WordTriggerType};

#[allow(dead_code)]
pub fn write(trigger_word_data: &Vec<WordTrigger>) -> String {
    let mut str = "".to_string();
    for statement in trigger_word_data {
        let internal = match &statement.trigger_data {
            WordTriggerType::Alias(alias_name, has_matched) => {
                format!(
                    "\"type\":\"alias\", \"value\":\"{}\", \"has_matched\":{has_matched}",
                    String::from_utf8_lossy(alias_name)
                )
            }
            WordTriggerType::Length(length, mod10) => {
                format!(
                    "\"type\":\"length\", \"len\":{}, \"mod10\":{}",
                    length, mod10
                )
            }
            WordTriggerType::Variable(var_name) => {
                format!(
                    "\"type\":\"variable\", \"name\":\"{}\"",
                    String::from_utf8_lossy(var_name)
                )
            }
            WordTriggerType::Import(import_name) => {
                format!(
                    "\"type\":\"import\",\"name\":\"{}\"",
                    String::from_utf8_lossy(import_name)
                )
            }
        };
        str += &format!(
            "{{\"start\":{}, \"end\":{}, {}}},",
            statement.word_start, statement.word_end, internal
        );
    }
    str.pop();
    format!("[{}]", str)
}
