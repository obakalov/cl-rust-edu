use anyhow::Result;
use clap::builder::PossibleValue;
use clap::{ArgAction, ValueEnum};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
struct Args {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d"),
            EntryType::File => PossibleValue::new("f"),
            EntryType::Link => PossibleValue::new("l"),
        })
    }
}

fn main() {
    if let Err(e) = run(get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    for path in args.paths.iter() {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{e}"),
                Ok(entry) => {
                    if check_enty_type(&args, &entry) && check_name(&args, &entry) {
                        println!("{}", entry.path().display())
                    }
                }
            }
        }
    }
    Ok(())
}

fn check_enty_type(args: &Args, entry: &DirEntry) -> bool {
    args.entry_types.is_empty()
        || args.entry_types.iter().any(|t| match t {
            EntryType::Dir => entry.file_type().is_dir(),
            EntryType::File => entry.file_type().is_file(),
            EntryType::Link => entry.file_type().is_symlink(),
        })
}

fn check_name(args: &Args, entry: &DirEntry) -> bool {
    args.names.is_empty()
        || args
            .names
            .iter()
            .any(|r| r.is_match(&entry.file_name().to_string_lossy()))
}

fn get_args() -> Args {
    let matches = clap::Command::new("findr")
        .version("0.1.0")
        .author("Obakalov")
        .about("A simple file finder")
        .arg(
            clap::Arg::new("paths")
                .value_name("PATH")
                .help("Paths to search")
                .default_value(".")
                .num_args(0..),
        )
        .arg(
            clap::Arg::new("names")
                .value_name("NAME")
                .long("name")
                .short('n')
                .help("Names to search")
                .value_parser(Regex::new)
                .action(ArgAction::Append)
                .num_args(0..),
        )
        .arg(
            clap::Arg::new("types")
                .value_name("TYPE")
                .long("type")
                .short('t')
                .help("Entry types to search")
                .value_parser(clap::value_parser!(EntryType))
                .action(ArgAction::Append)
                .num_args(0..),
        )
        .get_matches();

    Args {
        paths: match matches.get_many("paths") {
            Some(vals) => vals.cloned().collect(),
            None => vec![".".to_string()],
        },
        names: match matches.get_many::<Regex>("names") {
            Some(vals) => vals.cloned().collect(),
            None => vec![],
        },
        entry_types: match matches.get_many::<EntryType>("types") {
            Some(vals) => vals.cloned().collect(),
            None => vec![],
        },
    }
}
