fn main() {
    // Read the file path from command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <source-file>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];
    let source = std::fs::read_to_string(file_path).expect("Failed to read source file");

    // Create the interpreter
    let mut interpreter = mimble::Interpreter::new();

    // Set up a console tracer
    let console_tracer = Box::new(mimble::tracer::ConsoleTracer);
    interpreter.set_tracer(console_tracer);

    // Run the source code
    match interpreter.run(&source) {
        Ok(value) => {
            println!("Program finished successfully with value: {}", value);
        },
        Err(_) => {
            eprintln!("An error occurred during execution.");
            interpreter.emit_diagnostics(&source);
        },
    }
}