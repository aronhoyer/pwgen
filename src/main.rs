use clap::Parser;
use copypasta::{ClipboardContext, ClipboardProvider};
use rand::{thread_rng, Rng};

#[derive(Parser, Debug)]
struct Cli {
    /// Set the length of the password
    #[arg(short, long, default_value_t = 20)]
    length: u8,

    /// Copy password to clipboard
    #[arg(short, long)]
    clip: bool,

    /// Exclude symbols from password
    #[arg(short = 'S', long)]
    no_symbols: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut charsets = vec![
        "abcdefghklmnopqrstuvwxyz",
        "ABCDEFGHKLMNOPQRSTUVWXYZ",
        "1234567890",
    ];

    if !cli.no_symbols {
        charsets.push("!\"#$%&'()*+,-./\\:;?@[]^_`{|}~ ");
    }

    let mut password = String::new();
    for _ in 0..cli.length {
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

    if cli.clip {
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(password)
            .expect("couldn't copy password to clipboard");
    } else {
        println!("{}", password);
    }
}
