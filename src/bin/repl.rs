use mimble::run;

fn main() {
    // Create the sink
    let mut sink = mimble::DiagnosticsSink::new();

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

        match run(&input, &mut sink) {
            Ok(value) => println!("=> {}", value),
            Err(_) => {
                println!("An error occurred during execution.");
                sink.emit_all();
            },
        }
    }
}
