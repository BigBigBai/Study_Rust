use grep::Config;
use std::env;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 1. grep -h/--help
    if args.len() == 2 && (&args[1] == "-h" || &args[1] == "--help") {
        println!("Usage: grep [OPTIONS] <pattern> <files...>");
        println!("Options:");
        println!("-i                Case-insensitive search");
        println!("-n                Print line numbers");
        println!("-v                Invert match (exclude lines that match the pattern)");
        println!("-r                Recursive directory search");
        println!("-f                Print filenames");
        println!("-c                Enable colored output");
        println!("-h, --help        Show help information");
    }

    // 1. grep <pattern> <files...>
    // 2. grep [OPTIONS] <pattern> <files...>
    // 3. grep <pattern> <files...> [OPTIONS]
    // 4. grep <pattern> *.md, auto-complete
    if args.len() >= 3 {
        let mut lock_query = 0;
        let mut query = String::new();
        let mut filenames = Vec::new();
        let mut case_insensitive = false;
        let mut line_number = false;
        let mut invert_match = false;
        let mut find_r = false;
        let mut print_filename = false;
        let mut print_color = false;

        for i in 1..args.len() {
            let cmd = &args[i];

            match cmd.as_str() {
                "-i" => {
                    case_insensitive = true;
                }
                "-n" => {
                    line_number = true;
                }
                "-v" => {
                    invert_match = true;
                }
                "-r" => {
                    find_r = true;
                }
                "-f" => {
                    print_filename = true;
                }
                "-c" => {
                    print_color = true;
                }
                _ => {
                    if lock_query == 0 {
                        query.push_str(cmd);
                        lock_query = 1;
                    } else {
                        filenames.push(cmd);
                    }
                }
            }
        }
        println!("{:?}", filenames);

        let mut count = 1;
        if find_r {
            // recursively search folder
            for filename in &filenames {
                for entry in WalkDir::new(filename) {
                    let entry = entry.unwrap();
                    let entry = entry.path().to_str().unwrap().to_string();

                    let config = Config::new(
                        &query,
                        &entry,
                        case_insensitive,
                        line_number,
                        invert_match,
                        print_filename,
                        print_color,
                    );

                    grep::run(config, &mut count);
                }
            }
        } else {
            // only search file
            for filename in filenames {
                let config = Config::new(
                    &query,
                    &filename,
                    case_insensitive,
                    line_number,
                    invert_match,
                    print_filename,
                    print_color,
                );

                grep::run(config, &mut count);
            }
        }
    }
}
