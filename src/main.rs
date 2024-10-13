use clap::Parser;
use egrapsa::config::Config;
use std::path::PathBuf;

use std::io::Write;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long)]
    config_path: PathBuf,
    #[arg(short, long)]
    output_path: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let config_file = std::fs::File::open(&cli.config_path).unwrap();
    let config_reader = std::io::BufReader::new(config_file);
    let config = serde_json::from_reader::<_, Config>(config_reader).unwrap();

    println!("Compiling {}.", config.name());

    let mut formatter = config.formatter();
    let source = config.source();

    for work_info in config.take_work_infos() {
        formatter.add_work(work_info.into_work(source.as_ref()));
    }

    let mut output_file = std::fs::File::create(&cli.output_path).unwrap();
    write!(output_file, "{}", formatter.format()).unwrap();
}
