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

fn run(args: Args) -> Result<()> {
    let mut line_num = 0;
    for filename in &args.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {filename}: {e}"),
            Ok(mut file) =>
                line_num = run_file(&args, &mut file, line_num)?,
        }
    }
    Ok(())
}

fn run_file(
    args: &Args,
    file: &mut dyn BufRead,
    line_num: i32,
) -> Result<i32> {
    let mut line = String::new();
    let mut new_line_num = line_num;
    if line_num > 0 {
        println!()
    }
    loop {
        let read_bytes = file.read_line(&mut line)?;
        if read_bytes == 0 {
            break;
        }
        if args.number || (args.number_nonblank_lines && !line.trim().is_empty()) {
            new_line_num += 1;
            print!("{new_line_num:>6}\t");
        }
        print!("{line}");
        line.clear()
    }
    Ok(new_line_num)
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        file_name => Ok(Box::new(BufReader::new(File::open(file_name)?))),
    }
}
