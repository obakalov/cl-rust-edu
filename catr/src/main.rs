use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Parser, Debug)]
#[command(
    author = "OBakalov oleksii.bakalov@gmail.com",
    version = "0.1.0",
    about = "Rust version of cat"
)]
struct Args {
    #[arg(value_name = "files", required = true, default_value = "-")]
    files: Vec<String>,
    #[arg(long = "number", short = 'n', conflicts_with = "number_nonblank_lines")]
    number: bool,
    #[arg(long = "number-nonblank", short = 'b')]
    number_nonblank_lines: bool,
    // #[arg(value_name = "squeeze-blank", short = 's')]
    // squeeze_blank: bool,
    // #[arg(value_name = "show-all", short = 'A')]
    // show_all: bool,
    // #[arg(value_name = "show-ends", short = 'E')]
    // show_ends: bool,
    // #[arg(value_name = "show-tabs", short = 'T')]
    // show_tabs: bool,
    // #[arg(value_name = "show-nonprinting", short = 'v')]
    // show_nonprinting: bool,
}

fn main() {
    if let Err(e) = run(get_args()) {
        eprintln!("{}", e);
        std::process::exit(1)
    }
}

fn get_args() -> Args {
    let args = Args::parse();
    args
}

fn run(_args: Args) -> Result<()> {
    for filename in _args.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {filename}: {e}"),
            Ok(data) => run_file(&data)?,
        }
    }
    Ok(())
}

fn run_file(_buf: &Box<dyn BufRead>) -> Result<()>  {
    // buf.lines().for_each(|line| println!("{}", line?));
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        file_name => Ok(Box::new(BufReader::new(File::open(file_name)?))),
    }
}
