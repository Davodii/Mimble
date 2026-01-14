use mimble;
use mimble::Value; 

// fn eval(source: &str) -> RuntimeValue {
//     let mut sink = mimble::DiagnosticsSink::new();
//     mimble::run(source, &mut sink).expect("Execution failed")
// }

// #[test]
// fn test_math_logic(){
//     assert_eq!(eval("5 + (10 / 2)"), RuntimeValue::Number(10.0));

//     // TODO: add more tests
// }

// #[test]
// fn test_variable_assignment(){
//     assert_eq!(eval("x = 10\n x * 2"), RuntimeValue::Number(20.0));
// }