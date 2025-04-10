# Egrapsa

Egrapsa takes text from popular online libraries and converts it to LaTeX files, which can then be used to generate PDFs. For now only [Scaife](https://scaife.perseus.org/) (a collection of antique Roman and Greek texts) is supported. The style of typography is based on books printed in 17th and 18th centuries. So expect things like strange ligatures, long s, text ornaments, catch words and so on. All these will be configurable in future versions.

The code quality at the moment is quite doubtful. There is also no documentation, so have a look at examples in `configs` and at `--help` flag.

## Building
This is a regular Rust project using Cargo. To build and run use
```
cargo run --release
```
