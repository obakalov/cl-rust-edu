use anyhow::{Result, bail};
use clap::Parser;
use std::io::Read;
use std::ops::Range;
use std::str::Split;

#[derive(Debug, Parser)]
#[command(name = "cutr")]
#[clap(author = "Obakalov", version = "0.1.0", about = "A simple file cutter")]
struct Args {
    #[arg(default_value = "-", value_name = "FILES", help = "Input files")]
    files: Vec<String>,
    #[arg(
        short = 'd',
        long = "delim",
        help = "File delimiter",
        default_value = "\t"
    )]
    delimer: String,
    #[command(flatten)]
    extract: ArgsExtract,
}

#[derive(Debug, clap::Parser)]
struct ArgsExtract {
    #[arg(short = 'f', long = "fields", help = "Extract fields", value_name = "FIELDS", num_args(1..))]
    fields: Option<String>,
    #[arg(short = 'b', long = "bytes", help = "Extract bytes", value_name = "BYTES", num_args(1..))]
    bytes: Option<String>,
    #[arg(short = 'c', long = "chars", help = "Extract characters", value_name = "CHARS", num_args(1..))]
    chars: Option<String>,
}

type PositionList = Vec<Range<usize>>;

pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let delim_bytes = args.delimer.as_bytes();
    if delim_bytes.len() != 1 {
        bail!(r#"--delim "{}" must be a single byte "#, args.delimer);
    }
    let _delimiter: u8 = *delim_bytes.first().unwrap();

    args.files.iter().for_each(|file_name| {
        let mut file = std::fs::File::open(file_name).unwrap_or_else(|e| {
            eprintln!("Failed to open file {file_name}: {e}");
            std::process::exit(1);
        });
        let mut file_content = String::new();

        file.read_to_string(&mut file_content).unwrap_or_else(|e| {
            eprintln!("Failed to read file {file_name}: {e}");
            std::process::exit(1);
        });

        file_content.lines().for_each(|line| {
            println!("{line}");
        });
    });

    Ok(())
}

fn parse_pos(raw_string: String) -> Result<PositionList> {
    if raw_string.contains("+") {
        bail!("illegal list value: \"{}\"", raw_string);
    }
    raw_string
        .split(",")
        .map(|part| {
            let numbers: Vec<usize> = part
                .split("-")
                .map(|s| s.trim().parse::<usize>())
                .collect::<Result<Vec<usize>, _>>()
                .map_err(|_| anyhow::anyhow!("illegal list value: \"{}\"", raw_string))?;
            match numbers.as_slice() {
                [] | [0] | [0, _] => bail!("illegal list value: \"{}\"", raw_string),
                [n] => Ok(*n-1..*n),
                [start, end] if start < end => Ok((*start-1)..*end),
                [start, end] if start >= end => bail!(
                    "First number in range ({}) must be less than second ({})",
                    start ,
                    end
                ),
                _ => bail!("illegal list value: \"{}\"", raw_string),
            }
        })
        .collect()
}

#[cfg(test)]
mod unit_tests {
    use super::parse_pos;

    #[test]
    fn test_parse_pos_error_values() {
        assert!(parse_pos("".to_string()).is_err());

        let res = parse_pos("0".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "0""#);

        let res = parse_pos("0-1".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "0-1""#);

        let res = parse_pos("+1".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "+1""#);

        let res = parse_pos("+1-2".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "+1-2""#
        );
        let res = parse_pos("1-+2".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "1-+2""#
        );
    }

    #[test]
    fn test_parse_pos_error_non_number() {
        let res = parse_pos("a".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a""#);

        let res = parse_pos("1,a".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "1,a""#);

        let res = parse_pos("1-a".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "1-a""#);

        let res = parse_pos("a-1".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a-1""#);
    }

    #[test]
    fn test_parse_pos_wonky_ranges() {
        let res = parse_pos("-".to_string());
        assert!(res.is_err());

        let res = parse_pos(",".to_string());
        assert!(res.is_err());

        let res = parse_pos("1,".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-1-1,".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-1-a".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-1".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"First number in range (1) must be less than second (1)"#
        );

        let res = parse_pos("2-1".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"First number in range (2) must be less than second (1)"#
        );
    }

    #[test]
    fn test_parse_pos_ok() {
        let res = parse_pos("1".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("01".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("1,3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("001,003".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("1-3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("0001-3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("1,7,3-5".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

        let res = parse_pos("15, 19-20".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }
}
