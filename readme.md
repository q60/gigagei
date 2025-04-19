# ᎩᎦᎨᎢ ![rust](https://img.shields.io/badge/-Rust-DD3516?style=for-the-badge&logo=rust)

**ᎩᎦᎨᎢ** (IPA: \[gigagei\]) is a random quote fetching console utility. Written in Rust.

![screenshot](https://i.imgur.com/TGvihR3.png)


## installing

+ use latest pre-built binary from [releases](https://github.com/q60/gigagei/releases)
+ use provided flake


## options

```text
Usage: gigagei [-l <language>] [-a] [-n] [-j] [-w <wrap-width>]

A random quote fetching console utility

Options:
  -l, --language    quote language, must be one of: en[glish], ru[ssian]
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

