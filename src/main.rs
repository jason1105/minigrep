// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html

// Listing 12-1: Collecting the command line arguments into a vector and printing them
use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // use instead args_os() to process non-unicode


    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("{:?}", args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // run(config).unwrap_or_else(|err| {
    //     println!("Problem running: {}", err);
    //     process::exit(1);
    // });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

