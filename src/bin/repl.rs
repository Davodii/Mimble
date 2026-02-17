fn main() {
    // Create the interpreter
    let mut interpreter = mimble::Interpreter::new();

    // Set up a console tracer
    let console_tracer: Box<mimble::tracer::ConsoleTracer> = Box::new(mimble::tracer::ConsoleTracer);
    interpreter.set_tracer(console_tracer);

    // Start REPL
    println!("Welcome to the Mimble REPL!");
    use std::io::{self, Write};
    let mut input = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        let val = interpreter.run(&input);

        match val {
            Ok(value) => println!("=> {}", value),
            Err(_) => {
                println!("An error occurred during execution.");
                interpreter.emit_diagnostics(&input);

                interpreter.clear_diagnostics();
            },
        }
    }
}
