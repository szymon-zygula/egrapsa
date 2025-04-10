# Egrapsa

Egrapsa takes text from popular online libraries and converts it to LaTeX files, which can then be used to generate PDFs. For now only [Scaife](https://scaife.perseus.org/) (a collection of antique Roman and Greek texts) is supported.

The code quality at the moment is quite doubtful. There is also no documentation, so have a look at examples in `configs` and at `--help` flag.

## Building
This is a regular Rust project using Cargo. To build and run use
```
cargo run --release
```
