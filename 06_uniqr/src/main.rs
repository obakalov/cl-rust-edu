use anyhow::{anyhow, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Parser)]
#[command(
    author = "Obakalov",
    version = "0.1.0",
    about = "A simple uniqr on Rust"
)]
struct Args {
    #[arg(value_name = "IN_FILE", required = false, default_value = "-")]
    in_file: String,
    #[arg(value_name = "OUT_FILE", required = false)]
    out_file: Option<String>,
    #[arg(short = 'c', long = "count", help = "Print the number of lines")]
    count: bool,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let mut file = open(&args.in_file)
        .map_err(|e| anyhow!("{}: {e}", args.in_file))?;
    let mut line = String::new();
    let mut prev_line = String::new();
    let mut count: u64 = 0;
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break Ok(());
        }
        if prev_line.is_empty() {
            prev_line = line.clone();
            continue;
        }
        if prev_line == line {

        }

        print!("{line}");
        line.clear();
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        name => Ok(Box::new(BufReader::new(File::open(name)?))),
    }
}
