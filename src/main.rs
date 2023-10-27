use std::env;
use std::fs;
use std::process;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    // for now we take just one filename as argument, later we'll probably 
    // read the files from the arguments vector starting from index 1
    let config = Config::new(&arguments).unwrap_or_else(|_err|{
        println!("Usage: rust_cat <options> <file_name>");
        process::exit(1);
    });

    let display_options = config.display_options;
    let filename = config.file_path;

    println!("We shall process file {}", filename);
    let file_contents = fs::read_to_string(filename).expect("Unable to open file specified");

    // add a match construct here
    match display_options.as_str() {
        "-n" => {
            let lines = file_contents.split("\n");
            for (i, line) in lines.enumerate() {
                println!("{} {}", i, line);
            }
        },
        "" => {
            println!("{file_contents}");
        }, 
        _ => {
            // we don't cover this yet
        }
    }
}

struct Config {
    display_options: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str>{
        let display_options = args[1].clone();
        let file_path = args[2].clone();

        if args.len() < 3 {
            return Err("Incorrect number of arguments");
        }

        Ok(Config {
            display_options,
            file_path
        })
    }
}