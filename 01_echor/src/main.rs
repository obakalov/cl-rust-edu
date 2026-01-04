use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "OBakalov oleksii.bakalov@gmail.com",
    version = "0.1.0",
    about = "Rust version of echo"
)]
struct Args {
    #[arg(value_name = "TEXT", required = true)]
    text: Vec<String>,
    #[arg(short = 'n', long = "no-newline")]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    )
}
