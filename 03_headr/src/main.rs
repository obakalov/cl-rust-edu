use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result};

#[derive(Debug, clap::Parser)]
#[command(
    author = "Obakalov",
    version = "0.1.0",
    about = "Print the first part of a file."
)]
struct Args {
    #[arg(value_name = "FILES", required = true, default_value = "-")]
    files: Vec<String>,
    #[arg(
        short = 'n',
        long = "lines",
        value_name = "LINES",
        default_value = "10"
    )]
    lines: u64,
    #[arg(
        short = 'c',
        long = "bytes",
        value_name = "BYTES",
        conflicts_with = "lines"
    )]
    bytes: Option<u64>,
}

fn main() {
    let args = get_args();
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let file_count = args.files.len();

    for (idx, filename) in &args.files.iter().enumerate() {
        if file_count > 1 {
            println!("{}==> {} <==", if idx > 0 { "\n" } else { "" }, filename);
        }
        let mut file = open_file(filename)?;
        if let Some(bytes_from_read) = args.bytes {
            let mut buffer = Vec::new();
            file.take(bytes_from_read).read_to_end(&mut buffer)?;
            let text = String::from_utf8_lossy(&buffer);
            print!("{}", text.trim_end());
        } else {
            let mut line = String::new();
            for _ in 0..args.lines {
                line.clear();
                let bytes_read = file.read_line(&mut line)?;
                if bytes_read == 0 {
                    break;
                }
                print!("{}", line);
            }
        }
        // if file_count > 1 {
        //     println!();
        // }
    }
    Ok(())
}

fn get_args() -> Args {
    let args = Args::parse();
    // dbg!(&args);
    args
}

fn open_file(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        file_name => Ok(Box::new(BufReader::new(File::open(file_name)?))),
    }
}
