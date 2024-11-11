use std::fs;

use crate::{
    parser::{ParsedData, ParserFlags, ParserSource},
    parser_runner::{get_parsed_data, RunnerFlags},
    writers::javascript_writer,
};

pub fn gen_output(path: &str) -> () {
    let parser_flags = ParserFlags { title: true };
    let runner_flags = RunnerFlags {
        assert_steps: false,
        input: false,
        whole_program: false,
        linted: false,
        line_rendered: false,
        word_trigger: false,
    };
    let contents = fs::read_to_string(path).expect("File not found");
    match get_editor_property(&contents, ":code") {
        Some(code) => {
            let data = get_parsed_data(
                parser_flags.clone(),
                runner_flags.clone(),
                ParserSource::from_string(code.as_bytes().to_vec()),
            );
            let output_path = get_output_path(path);
            _ = fs::write(output_path, get_js(&data));
        }
        None => (),
    }
}

pub fn get_output_path(path: &str) -> String {
    format!("{}_exp.js", &path[..path.len() - 3])
}

pub fn get_js(data: &ParsedData) -> String {
    javascript_writer::write(&data.exprs, &data.stat_starts)
}

/// Returns Prosetta code parsed from a Markdown string.
///
/// Returns `None` if no code was found.
pub fn get_editor_property(contents: &str, prop: &str) -> Option<String> {
    // Look for starting tag
    match contents.find("<editor") {
        None => None,
        Some(start) => {
            // Look for ending tag
            let end_str = "</editor>";
            let end;
            match &contents[start..].find(end_str) {
                // If not valid XML, then return None
                None => return None,
                Some(e) => end = start + e + end_str.len(),
            }

            // Only work with the XML section
            let xml = &contents[start..end];

            // Look for ':code' and then a backtick after that because I don't want to be
            // whitespace-dependent
            let code_start;
            let code_end;
            match xml.find(prop) {
                None => return None,
                Some(e) => {
                    let temp_start = e + prop.len();
                    match &xml[temp_start..].find("`") {
                        None => return None,
                        Some(e) => code_start = temp_start + e + 1,
                    }
                }
            }

            // Look for closing backtick
            match &xml[code_start..].find("`") {
                None => return None,
                Some(e) => code_end = code_start + e,
            }

            Some(xml[code_start..code_end].replace("\\n", "\n"))
        }
    }
}
