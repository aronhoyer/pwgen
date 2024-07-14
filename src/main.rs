use std::{fs::File, io::Read, process::exit, str::Lines};

use clap::Parser;
use copypasta::{ClipboardContext, ClipboardProvider};
use rand::{thread_rng, Rng};

#[derive(Parser, Debug)]
#[command(name = "pwgen")]
#[command(version)]
#[command(about = "Generate a random password", long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    /// Set the length of the password.
    ///
    /// When combined with the [-w,--word] option, it sets the number of words.
    #[arg(short, long)]
    length: Option<u8>,

    /// Copy password to clipboard
    #[arg(short, long)]
    clip: bool,

    /// Exclude symbols from password
    #[arg(short = 'S', long)]
    no_symbols: bool,

    /// Use words instead of characters
    #[arg(short, long)]
    words: bool,
}

fn gen_with_characters(length: u8, no_symbols: bool) -> String {
    let mut charsets = vec![
        "abcdefghklmnopqrstuvwxyz",
        "ABCDEFGHKLMNOPQRSTUVWXYZ",
        "1234567890",
    ];

    if !no_symbols {
        charsets.push("!\"#$%&'()*+,-./\\:;?@[]^_`{|}~ ");
    }

    let mut password = String::new();
    for _ in 0..length {
        let charset = charsets
            .get(thread_rng().gen_range(0..charsets.len()))
            .unwrap();

        password.push(
            charset
                .chars()
                .nth(thread_rng().gen_range(0..charset.len()))
                .unwrap(),
        );
    }

    return password;
}

fn gen_with_words(length: u8, words: Lines) -> String {
    let mut words = words.collect::<Vec<&str>>();

    let mut password: Vec<&str> = vec![];

    for _ in 0..length {
        password.push(words.swap_remove(thread_rng().gen_range(0..words.len())));
    }

    return password.join("-");
}

fn main() {
    let cli = Cli::parse();

    let mut length = cli.length.unwrap_or(20);

    let password: String;

    if cli.words {
        if cli.length.is_none() {
            length = 4;
        }
        let mut file = File::options().read(true).open("./words.txt").unwrap();
        let mut words = String::new();

        file.read_to_string(&mut words).unwrap();

        password = gen_with_words(length, words.lines());
    } else {
        password = gen_with_characters(length, cli.no_symbols);
    }

    if cli.clip {
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(password)
            .expect("couldn't copy password to clipboard");
        exit(0);
    } else {
        println!("{}", password);
    }
}
