# pwgen &ndash; CLI to generate passwords

A simple command line utility to generate passwords.

## Installation

### Build from source

Make sure `rustup` is installed.

```sh
git clone https://github.com/aronhoyer/pwgen.git && cd pwgen
cargo build --release
cp target/release/pwgen /usr/local/bin # or somewhere else in your $PATH
mkdir -p $HOME/.local/share/pwgen && cp words.txt $HOME/.local/share/pwgen
```

## License

pwgen is licensed under the [MIT License](./LICENSE)
