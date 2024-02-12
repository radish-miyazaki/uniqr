use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::Write;

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

    let mut out_file: Box<dyn Write> = match &args.out_file {
        Some(name) => Box::new(File::create(name)?),
        None => Box::new(io::stdout()),
    };

    count_and_output_duplicate_lines(args, &mut file, &mut out_file)?;

    Ok(())
}

fn count_and_output_duplicate_lines(
    args: Args,
    file: &mut Box<dyn BufRead>,
    mut out_file: &mut Box<dyn Write>,
) -> MyResult<()> {
    let mut line = String::new();
    let mut prev_line = String::new();
    let mut line_count: usize = 0;

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() == prev_line.trim_end() {
            line_count += 1;
        } else {
            output_line(args.count, &mut out_file, prev_line.clone(), line_count)?;

            line_count = 1;
            prev_line = line.clone();
        }

        line.clear();
    }

    // INFO: 最終行を出力するために、ループの外で再度呼び出す
    output_line(args.count, &mut out_file, prev_line.clone(), line_count)?;

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

fn output_line(
    count: bool,
    out_file: &mut Box<dyn Write>,
    line: String,
    line_count: usize,
) -> MyResult<()> {
    if line_count <= 0 { return Ok(()); }

    if count {
        write!(out_file, "{:>4} {}", line_count, line)?;
    } else {
        write!(out_file, "{}", line)?;
    }

    Ok(())
}
