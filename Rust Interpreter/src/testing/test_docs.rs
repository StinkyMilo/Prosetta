#![cfg(all(test, not(feature = "no-doc-tests")))]

use ntest::timeout;

use crate::docs_lib::get_editor_property;
use crate::parser::testing::testing::test_lib::get_js;
use crate::parser::{ParserFlags, ParserSource};
use crate::parser_runner::{get_parsed_data, RunnerFlags};
use std::fs;

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
                get_js(&data),
                js_output.as_ref().unwrap().to_string(),
                "For file path {}",
                path
            );
        }
        None => (),
    }
    if let Some(js) = js_output {
        match get_editor_property(&contents, ":code-wordier") {
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
                assert_eq!(get_js(&data), js, "For file path {}", path);
            }
            None => (),
        }
    }
}

// START OF GENERATED TESTS
    #[test]
    #[timeout(2000)]
    fn test__sidebar () -> () {
        test_file("../Frontend/docs/_sidebar.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_frame () -> () {
        test_file("../Frontend/docs/Frame.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_foreach () -> () {
        test_file("../Frontend/docs/Foreach.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_function () -> () {
        test_file("../Frontend/docs/Function.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_roundedrectangle () -> () {
        test_file("../Frontend/docs/RoundedRectangle.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_rectangle () -> () {
        test_file("../Frontend/docs/Rectangle.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_add () -> () {
        test_file("../Frontend/docs/Add.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_cosine () -> () {
        test_file("../Frontend/docs/Cosine.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_modulo () -> () {
        test_file("../Frontend/docs/Modulo.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_append () -> () {
        test_file("../Frontend/docs/Append.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_length () -> () {
        test_file("../Frontend/docs/Length.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_while () -> () {
        test_file("../Frontend/docs/While.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_moveto () -> () {
        test_file("../Frontend/docs/MoveTo.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_imports () -> () {
        test_file("../Frontend/docs/Imports.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_stroke () -> () {
        test_file("../Frontend/docs/Stroke.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_heart () -> () {
        test_file("../Frontend/docs/Heart.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_floor () -> () {
        test_file("../Frontend/docs/Floor.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_if () -> () {
        test_file("../Frontend/docs/If.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_line () -> () {
        test_file("../Frontend/docs/Line.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_multiply () -> () {
        test_file("../Frontend/docs/Multiply.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_comparison () -> () {
        test_file("../Frontend/docs/Comparison.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_ellipse () -> () {
        test_file("../Frontend/docs/Ellipse.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_list () -> () {
        test_file("../Frontend/docs/List.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_ignore () -> () {
        test_file("../Frontend/docs/Ignore.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_index () -> () {
        test_file("../Frontend/docs/Index.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_bezier () -> () {
        test_file("../Frontend/docs/Bezier.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_else () -> () {
        test_file("../Frontend/docs/Else.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_not () -> () {
        test_file("../Frontend/docs/Not.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_punctuation () -> () {
        test_file("../Frontend/docs/Punctuation.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_sine () -> () {
        test_file("../Frontend/docs/Sine.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_tangent () -> () {
        test_file("../Frontend/docs/Tangent.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_delete () -> () {
        test_file("../Frontend/docs/Delete.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_fill () -> () {
        test_file("../Frontend/docs/Fill.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_greaterthan () -> () {
        test_file("../Frontend/docs/GreaterThan.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_and () -> () {
        test_file("../Frontend/docs/And.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_color () -> () {
        test_file("../Frontend/docs/Color.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_kirby () -> () {
        test_file("../Frontend/docs/Kirby.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_int () -> () {
        test_file("../Frontend/docs/Int.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_star () -> () {
        test_file("../Frontend/docs/Star.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_divide () -> () {
        test_file("../Frontend/docs/Divide.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_or () -> () {
        test_file("../Frontend/docs/Or.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_readme () -> () {
        test_file("../Frontend/docs/README.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_linewidth () -> () {
        test_file("../Frontend/docs/LineWidth.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_triangle () -> () {
        test_file("../Frontend/docs/Triangle.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_lit () -> () {
        test_file("../Frontend/docs/Lit.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_shapedrawing () -> () {
        test_file("../Frontend/docs/ShapeDrawing.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_print () -> () {
        test_file("../Frontend/docs/Print.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_return () -> () {
        test_file("../Frontend/docs/Return.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_exponentiate () -> () {
        test_file("../Frontend/docs/Exponentiate.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_polygon () -> () {
        test_file("../Frontend/docs/Polygon.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_replace () -> () {
        test_file("../Frontend/docs/Replace.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_log () -> () {
        test_file("../Frontend/docs/Log.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_rotate () -> () {
        test_file("../Frontend/docs/Rotate.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_literals () -> () {
        test_file("../Frontend/docs/Literals.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_random () -> () {
        test_file("../Frontend/docs/Random.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_find () -> () {
        test_file("../Frontend/docs/Find.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_lessthan () -> () {
        test_file("../Frontend/docs/LessThan.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_glossary () -> () {
        test_file("../Frontend/docs/Glossary.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_subtract () -> () {
        test_file("../Frontend/docs/Subtract.md");
    }


    #[test]
    #[timeout(2000)]
    fn test_variable () -> () {
        test_file("../Frontend/docs/Variable.md");
    }
// END OF GENERATED TESTS
