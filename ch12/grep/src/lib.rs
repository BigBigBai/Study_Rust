use colored::*;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
    pub line_number: bool,
    pub invert_match: bool,
    pub print_filename: bool,
    pub print_color: bool,
}

impl Config {
    pub fn new(
        query: &String,
        filename: &String,
        case_insensitive: bool,
        line_number: bool,
        invert_match: bool,
        print_filename: bool,
        print_color: bool,
    ) -> Config {
        Config {
            query: query.clone(),
            filename: filename.clone(),
            case_insensitive,
            line_number,
            invert_match,
            print_filename,
            print_color,
        }
    }
}

pub fn run(config: Config, count: &mut i32) {
    let contents = match fs::read_to_string(&config.filename) {
        Ok(file) => file,
        Err(_) => return,
    };

    let results = if config.case_insensitive {
        search_case_insensitive(&config.query, &contents, config.invert_match)
    } else {
        search(&config.query, &contents, config.invert_match)
    };

    for line in results {
        if config.line_number {
            print!("{}: ", count);
            *count += 1;
        }

        if config.print_filename {
            print!("{}: ", &config.filename);
        }

        if config.print_color {
            let mut remaining = line;
            while let Some(index) = remaining.find(&config.query) {
                let (before, after) = remaining.split_at(index);
                let (highlight, after) = after.split_at(config.query.len());

                print!("{}", before);
                print!("{}", highlight.red());

                remaining = after;
            }
            println!("{}", remaining);
        } else {
            println!("{}", line);
        }
    }
}

pub fn search<'a>(query: &str, contents: &'a str, invert: bool) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) && !invert {
            results.push(line);
        } else if !line.contains(query) && invert && !line.is_empty() {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str, invert: bool) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) && !invert {
            results.push(line);
        } else if !line.to_lowercase().contains(&query) && invert && !line.is_empty() {
            results.push(line);
        }
    }

    results
}
