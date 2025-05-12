# ᎩᎦᎨᎢ ![rust](https://img.shields.io/badge/-Rust-DD3516?style=for-the-badge&logo=rust)

[![Crates.io Version](https://img.shields.io/crates/v/gigagei?style=for-the-badge)](https://crates.io/crates/gigagei)

**ᎩᎦᎨᎢ** (IPA: \[gigagei\]) is a random quote fetching console utility. Written in Rust.

![screenshot](https://github.com/user-attachments/assets/e4c1f7a0-e67e-42b7-91fe-81a1dc0f99d0)


## installing

+ use via the flake
+ `cargo install gigagei`
+ use latest pre-built binary from [releases](https://github.com/q60/gigagei/releases)


## options

```text
Usage: gigagei [-b <backend>] [-l <language>] [-a] [-n] [-j] [-w <wrap-width>]

A random quote fetching console utility

Options:
  -b, --backend     quote fetching API, must be one of: forismatic, hapesire.
                    default is hapesire
  -l, --language    quote language, must be one of: en[glish], ru[ssian].
                    default is en
  -a, --ascii-quotation
                    force ASCII quotation marks
  -n, --no-colors   disables colors
  -j, --json        return quote in JSON
  -w, --wrap-width  wrap width in characters, default is terminal width
  -h, --help, help  display usage information
```


## building and installing manually

you need *Rust* to build **ᎩᎦᎨᎢ**.

```sh
cargo build --release
```


## running

```sh
gigagei
```

