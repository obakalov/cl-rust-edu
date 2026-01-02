fn main() {
    let matches = clap::Command::new("echor")
        .version("0.1.0")
        .author("OBakalov oleksii.bakalov@gmail.com")
        .about("Rust version of echo")
        .arg(
            clap::Arg::new("text")
                .value_name("TEXT")
                .help("Text to echo")
                .required(true)
                .num_args(1..),
        )
        .arg(
            clap::Arg::new("omit_newline")
                .short('n')
                .action(clap::ArgAction::SetTrue)
                .help("Do not print the trailing newline"),
        )
        .get_matches();
    let text: Vec<String> = matches.get_many("text").unwrap().cloned().collect();
    let omit_newline: bool = matches.get_flag("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
