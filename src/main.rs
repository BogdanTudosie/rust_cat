extern crate clap;

use std::fs;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Config {
    #[arg(short='n')]
    display_options: Option<bool>,
    #[clap(parse(from_os_str), required=true)]
    file_path: std::path::PathBuf
}

fn main() {
    //let arguments: Vec<String> = env::args().collect();
    let arguments = Config::parse();

    let display_options = arguments.display_options;
    let filename = arguments.file_path;

    println!("We shall process file {:?}", filename);
    let file_contents = fs::read_to_string(filename).expect("Unable to open file specified");

    // add a match construct here
    match display_options {
        Some(_display_options) => {
            let lines = file_contents.split("\n");
            for (i, line) in lines.enumerate() {
                println!("{} {}", i, line);
            }
        },
        None => {
            println!("{}", file_contents)
        }
    }
}