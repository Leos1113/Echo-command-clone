use clap::{Arg, Command};

fn main() {
    let matches = Command::new("EchoTutorial")
        .version("0.1.0")
        .author("Edgar Leon Martín")
        .about("Rust echo clone")
        .arg(
            Arg::new("text")
                .allow_invalid_utf8(true)
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline at the end")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();

    let omit_newline = matches.is_present("omit_newline");

    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text.join(" "), ending);
}
