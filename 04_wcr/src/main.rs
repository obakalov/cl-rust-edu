use clap::{Parser};
use std::io::{BufRead, Result};

#[derive(Debug, Parser)]
#[command(author = "Obakalov", version = "0.1.0", about = "A simple wc on Rust")]
struct Args {
    #[arg(value_name = "FILES", required = true)]
    files: Vec<String>,
    #[arg(short = 'l', long = "lines", help = "Print the number of lines")]
    lines: bool,
    #[arg(short = 'w', long = "words", help = "Print the number of words")]
    words: bool,
    #[arg(short = 'b', long = "bytes", help = "Print the number of bytes")]
    bytes: bool,
    #[arg(
        short = 'm',
        long = "chars",
        help = "Print the number of characters",
        conflicts_with = "bytes"
    )]
    chars: bool,
}

#[derive(Debug, PartialEq)]
struct FileInfo {
    num_lines: u64,
    num_words: u64,
    num_bytes: u64,
    num_chars: u64,
}

fn main() {
    let args = get_args();
    for file_name in &args.files {
        match file_open(file_name) {
            Err(e) => eprintln!("Error: {}: {}", file_name, e),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    args.lines.then(|| print!("{:>8}", info.num_lines));
                    args.words.then(|| print!("{:>8}", info.num_words));
                    args.bytes.then(|| print!("{:>8}", info.num_bytes));
                    args.chars.then(|| print!("{:>8}", info.num_chars));
                    println!(" {}", if file_name == "-" { "" } else { file_name });
                }
            }
        }
    }
}

fn get_args() -> Args {
    let mut args = Args::parse();
    if [args.words, args.lines, args.bytes, args.chars]
        .iter()
        .all(|v| !v)
    {
        args.words = true;
        args.lines = true;
        args.bytes = true;
    }
    args
}

fn file_open(filename: &str) -> Result<Box<dyn std::io::BufRead>> {
    match filename {
        "-" => Ok(Box::new(std::io::BufReader::new(std::io::stdin()))),
        file_name => Ok(Box::new(std::io::BufReader::new(std::fs::File::open(
            file_name,
        )?))),
    }
}

fn count(mut file: impl BufRead) -> Result<FileInfo> {
    let mut num_bytes = 0;
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes as u64;
        num_lines += 1;
        num_words += line.split_whitespace().count() as u64;
        num_chars += line.chars().count() as u64;
        line.clear();
    }

    Ok(FileInfo {
        num_bytes,
        num_lines,
        num_words,
        num_chars,
    })
}
