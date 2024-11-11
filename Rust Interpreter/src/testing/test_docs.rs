#[cfg(test)]
mod test_docs {
    use crate::docs_lib::get_code;
    use crate::parser::testing::testing::test_lib::get_js;
    use crate::parser::{ParserFlags, ParserSource};
    use crate::parser_runner::{get_parsed_data, RunnerFlags};
    use ntest::timeout;
    use std::fs;

    #[test]
    #[timeout(1000)]
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
        let contents = fs::read_to_string(path).expect("File not found");
        match get_code(&contents) {
            Some(code) => {
                let data = get_parsed_data(
                    ParserFlags { title: true },
                    RunnerFlags {
                        assert_steps: false,
                        input: false,
                        whole_program: true,
                        line_rendered: false,
                        linted: false,
                        word_trigger: false,
                    },
                    ParserSource::from_string(code.as_bytes().to_vec()),
                );
                let output_path = format!("{}_exp.js", &path[..path.len() - 3]);
                let expected_output = fs::read_to_string(output_path).expect("File not found");
                assert_eq!(expected_output, get_js(&data));
            }
            None => (),
        }
    }
}
