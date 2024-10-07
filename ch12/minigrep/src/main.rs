use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && (args[1] == "-h" || args[1] == "-help") {
        println!("Usage: grep [OPTIONS] <pattern> <files...>");
        println!("Options:");
        println!("-i                Case-insensitive search");
        println!("-n                Print line numbers");
        println!("-v                Invert match (exclude lines that match the pattern)");
        println!("-r                Recursive directory search");
        println!("-f                Print filenames");
        println!("-c                Enable colored output");
        println!("-h, --help        Show help information");
    } else {
        let cmd = &args[3];

        match cmd.as_str() {
            "-i" => {
                println!("-i")
                
            }
            "-n" => println!("-n"),
            "-v" => println!("-v"),
            "-r" => {
                println!("-r");
            }
            "-f" => println!("-f"),
            "-c" => println!("-c"),
            _ => println!("Invalid"),
        }

        // let config = Config::new(&args).unwrap_or_else(|err| {
        //     eprintln!("Problem parsing arguments: {}", err);
        //     process::exit(1);
        // });

        // if let Err(e) = minigrep::run(config) {
        //     eprintln!("Application error: {}", e);
        //     process::exit(1);
        // }
    }
}
