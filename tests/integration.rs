use mimble::{self, Interpreter, evaluator::value::Value};

fn run_test_assertion(file_path: &str, source: String) {
    let mut expected_value = None;
    let mut expected_error = None;

    // Parse expectations from the file comments
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(val) = trimmed.strip_prefix("# expect: ") {
            expected_value = Some(val.to_string());
        } else if let Some(val) = trimmed.strip_prefix("# error: ") {
            expected_error = Some(val.to_string());   
        }
    }

    assert!(
        expected_value.is_some() || expected_error.is_some(),
        "Test file {} must contain either an '# expect: <val>' or '# error: <type>' comment.",
        file_path
    );

    // Setup and run the interpreter
    let mut interpreter = Interpreter::new();
    let result = interpreter.run(&source);

    if let Some(expected) = expected_value {
        match result {
            Ok(value) => {
                // Convert your Value to a string for comparison. 
                // You might need to implement Display for Value or use format!("{:?}", value)
                let actual_string = value_to_string(&value); 
                assert_eq!(
                    actual_string, expected,
                    "Mismatch in {}: expected {}, got {}",
                    file_path, expected, actual_string
                );
            }
            Err(_) => {
                let diagnostics = interpreter.diagnotics();

                diagnostics.borrow().emit_all(&source); // Emit diagnostics to help debug

                // If it failed, print the diagnostics to help debug
                panic!("Test {} failed, but expected value '{}'. Check diagnostics.", file_path, expected);
            }
        }
    } else if let Some(_expected_err) = expected_error {
        match result {
            Ok(value) => {
                panic!(
                    "Test {} expected an error, but successfully returned {:?}",
                    file_path, value
                );
            }
            Err(_) => {
                // Success! It errored as expected. 
                // Advanced: You could peek into interpreter.ctx.diagnostics here 
                // to verify it's the *correct* type of error.
                // TODO: Implement error type checking in diagnostics and compare against _expected_err string for more robust testing.
            }
        }
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::Integer(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s.clone(),
        Value::Boolean(b) => b.to_string(),
        Value::Nil => "nil".to_string(),
        // Handle Arrays and Functions appropriately
        _ => format!("{:?}", value),
    }
}

macro_rules! generate_tests {
    ($dir:expr, $func:ident) => {
        #[test]
        fn $func() {
            let paths = std::fs::read_dir($dir).unwrap();
            for path in paths {
                let path = path.unwrap().path();
                if path.extension().map_or(false, |ext| ext == "mbl") {
                    let source = std::fs::read_to_string(&path).unwrap();
                    run_test_assertion(&path.to_string_lossy(), source);
                }
            }
        }  
    };
}

generate_tests!("tests/lexer", test_lexer_scripts);
generate_tests!("tests/parser", test_parser_scripts);
generate_tests!("tests/analyser", test_analyser_scripts);

generate_tests!("tests/evaluator", test_evaluator_scripts);