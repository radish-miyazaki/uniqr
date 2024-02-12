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

    let mut out_file: Option<File> = None;
    if let Some(name) = &args.out_file {
        out_file = Some(File::create(name)?);
    }

    count_and_output_duplicate_lines(args, &mut file, &mut out_file)?;

    Ok(())
}

fn count_and_output_duplicate_lines(
    args: Args,
    file: &mut Box<dyn BufRead>,
    mut out_file: &mut Option<File>,
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
            output_line_to_destination(
                args.count,
                &mut out_file,
                prev_line.clone(),
                line_count,
            );

            line_count = 1;
            prev_line = line.clone();
        }

        line.clear();
    }

    // INFO: 最終行を出力するために、ループの外で再度呼び出す
    output_line_to_destination(
        args.count,
        &mut out_file,
        prev_line.clone(),
        line_count,
    );

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

fn write_line_with_optional_count(
    count: bool,
    file: &mut File,
    line: String,
    line_count: usize,
) {
    if count {
        write!(file, "{:>4} {}", line_count, line).unwrap();
    } else {
        write!(file, "{}", line).unwrap();
    }
}

fn print_line_with_optional_count(
    count: bool,
    line: String,
    line_count: usize,
) {
    if count {
        print!("{:>4} {}", line_count, line);
    } else {
        print!("{}", line);
    }
}

fn output_line_to_destination(
    count: bool,
    out_file: &mut Option<File>,
    line: String,
    line_count: usize,
) {
    if line_count <= 0 {
        return;
    }

    if let Some(ref mut file) = out_file {
        write_line_with_optional_count(count, file, line, line_count);
    } else {
        print_line_with_optional_count(count, line, line_count);
    }
}
