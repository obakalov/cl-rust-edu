use clap::builder::{PossibleValue, TypedValueParser};
use clap::{ArgAction, Command, ValueEnum};
use regex::Regex;
use anyhow::Result;
use walkdir::WalkDir;

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
    for path in args.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{e}"),
                Ok(entry) => println!("{}", entry.path().display())
            }
        }
    }
    Ok(())
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
                .num_args(0..)
            ,
        )
        .get_matches();

    Args {
        paths: match matches.get_many("paths") {
            Some(vals) => vals.cloned().collect(),
            None => vec![".".to_string()],
        },
        names : match matches.get_many::<Regex>("names") {
            Some(vals) => vals.cloned().collect(),
            None => vec![],
        },
        entry_types: match  matches.get_many::<EntryType>("types") {
            Some(vals) => vals.cloned().collect(),
            None => vec![],
        },
    }
}
