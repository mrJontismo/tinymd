use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;

fn get_title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("), ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    title
}

fn parse_markdown_file(filename: &str) {
    println!("Parsing {}...", filename);

    let input_path = Path::new(filename);
    let file = File::open(&input_path).expect("ERROR! Failed to open file.");
    let file_reader = BufReader::new(file);

    let mut ptag: bool = false;
    let mut htag: bool = false;

    let mut tokens: Vec<String> = Vec::new();

    for line in file_reader.lines() {
        let line_contents = line.unwrap();
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
        let mut output_line = String::new();

        match first_char.pop() {
            Some('#') => {
                if ptag {
                    ptag = false;
                    output_line.push_str("</p>\n");
                }

                if htag {
                    htag = false;
                    output_line.push_str("</h1>\n");
                }

                htag = true;
                output_line.push_str("\n\n<h1>");
                output_line.push_str(&line_contents[2..]);
            },

            _ => {
                if !ptag {
                    ptag = true;
                    output_line.push_str("<p>");
                }

                output_line.push_str(&line_contents);
            }
        }

        if ptag {
            ptag = false;
            output_line.push_str("</p>\n");
        }

        if htag {
            htag = false;
            output_line.push_str("</h1>\n");
        }

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    let mut output_filename = String::from(&filename[..filename.len() - 3]);
    output_filename.push_str(".html");
    let mut outfile = File::create(output_filename).expect("ERROR! Could not create output file!");

    for line in &tokens {
        outfile.write_all(line.as_bytes()).expect("ERROR! Could not write to output file!");
    }

    println!("Parsing complete!");
}

fn usage() {
    println!("{}", get_title());
    println!("Written by: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));
    println!("Usage: {} <somefile>.md", env!("CARGO_PKG_NAME"));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        parse_markdown_file(&args[1]);
    } else {
        println!("ERROR! Invalid arguments.");
        usage();
    }
}
