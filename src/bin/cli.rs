fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let code = std::fs::read_to_string(&args[1]).unwrap();

//     match mimble::run(&code) {
//         Ok(output) => println!("{}", output),
//         Err(e) => eprintln!("Error: {}", e),
//     }
}
