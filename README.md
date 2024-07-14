# pwgen &ndash; CLI to generate passwords

A simple command line utility to generate passwords.

## Usage

```
Usage: pwgen [OPTIONS]

Options:
  -l, --length <LENGTH>
          Set the length of the password.
          
          When combined with the [-w,--word] option, it sets the number of words.

  -c, --clip
          Copy password to clipboard

  -S, --no-symbols
          Exclude symbols from password

  -w, --words
          Use words instead of characters

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Providing your own list of words

Currently, the words used to generate a password are the 10,000 most common English words.

If you want to use a different set of words, overwrite `$XDG_DATA_HOME/pwgen/words.txt`.
Make sure, though, that it's a sufficiently large set of words.

## Installation

### Build from source

Make sure `rustup` is installed.

```sh
git clone https://github.com/aronhoyer/pwgen.git && cd pwgen
cargo build --release
cp target/release/pwgen /usr/local/bin # or somewhere else in your $PATH
mkdir -p "${XDG_DATA_HOME:-$HOME/.local/share}/pwgen"
cp words.txt "${XDG_DATA_HOME:-$HOME/.local/share}/pwgen"
```

## License

pwgen is licensed under the [MIT License](./LICENSE)
