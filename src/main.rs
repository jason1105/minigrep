// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html

// Listing 12-1: Collecting the command line arguments into a vector and printing them
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); // use instead args_os() to process non-unicode

    if (args.len() < 3) {
        println!("Usage: minigrep <query> <filename>");
        return 
    }

    let config = parse_config(&args);

    let query = &args[1];
    let filename = &args[2];

    println!("{:?}", args);
    println!("Searching for {}", query);
    println!("In file {}", filename);

    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", content);

}
