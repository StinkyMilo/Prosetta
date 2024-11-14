#![cfg(all(test, not(feature = "no-doc-tests")))]
use crate::docs_lib::get_editor_property;
use crate::parser::testing::testing::test_lib::get_js;
use crate::parser::{ParserFlags, ParserSource};
use crate::parser_runner::{get_parsed_data, RunnerFlags};
use ntest::timeout;
use std::fs;

#[test]
#[timeout(20000)]
fn test_all_docs() {
    use std::fs;
    println!("Generating JS output...");
    let paths = fs::read_dir("../Frontend/docs").unwrap();

    for p in paths {
        match p {
            Ok(v) => {
                let path = v.path();
                // We're only using ASCII so I think this is fine
                let path_str = path.display().to_string();

                if path.is_file() && path_str.ends_with(".md") {
                    test_file(&path_str);
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}

fn test_file(path: &str) -> () {
    let parser_flags = ParserFlags { title: true };
    let runner_flags = RunnerFlags {
        assert_steps: false,
        input: false,
        whole_program: false,
        linted: false,
        line_rendered: false,
        word_trigger: false,
    };
    let mut js_output = None;
    let contents = fs::read_to_string(path).expect("File not found");
    match get_editor_property(&contents, ":code") {
        Some(code) => {
            let output_path = format!("{}_exp.js", &path[..path.len() - 3]);
            js_output = Some(
                fs::read_to_string(output_path)
                    .expect("File not found")
                    .replace("\r", ""),
            );
            let data = get_parsed_data(
                parser_flags.clone(),
                runner_flags.clone(),
                ParserSource::from_string(code.as_bytes().to_vec()),
            );
            assert_eq!(
                js_output.as_ref().unwrap().to_string(),
                get_js(&data),
                "For file path {}",
                path
            );
        }
        None => (),
    }
    if let Some(js) = js_output {
        match get_editor_property(&contents, ":code-wordier") {
            Some(code) => {
                let data = get_parsed_data(
                    parser_flags.clone(),
                    runner_flags.clone(),
                    ParserSource::from_string(code.as_bytes().to_vec()),
                );
                assert_eq!(js, get_js(&data), "For file path {}", path);
            }
            None => (),
        }
    }
}
