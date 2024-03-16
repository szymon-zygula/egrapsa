use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Subcommands>
}

#[derive(Subcommand)]
enum Subcommands {
    Book {
        #[arg(short, long)]
        text_source: String
    },
    OperaOmnia {
        #[arg(short, long)]
        author_source: String
    }
}

fn main() {
    let cli = Cli::parse();
}
