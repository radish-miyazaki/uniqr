use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(name = "uniqr")]
#[command(version = "0.1.0")]
#[command(about = "Rust uniq")]
#[command(author = "Radish-Miyazaki <y.hidaka.kobe@gmail.com>")]
pub struct Args {
    #[arg(value_name = "IN_FILE", help = "Input file", default_value = "-")]
    in_file: String,
    #[arg(value_name = "OUT_FILE", help = "Output file")]
    out_file: Option<String>,
    #[arg(short, long, help = "Show counts")]
    count: bool,
}

pub fn get_args() -> MyResult<Args> {
    Ok(Args::parse())
}

pub fn run(args: Args) -> MyResult<()> {
    let mut file = open(&args.in_file)
        .map_err(|e| format!("{}: {}", args.in_file, e))?;

    let mut line = String::new();
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        print!("{}", line);
        line.clear();
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
