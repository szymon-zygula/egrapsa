use clap::{Parser, Subcommand};
use egrapsa::formatters::{latex::Latex, TextFormatter};
use egrapsa::text_sources::{scaife::Scaife, TextSource};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Subcommands>,
}

#[derive(Subcommand)]
enum Subcommands {
    Scaife {
        #[arg(short, long)]
        identifier: String,
        #[arg(short, long)]
        title: Option<String>,
        #[arg(short, long)]
        author: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    let Some(subcommand) = cli.command else {
        return;
    };

    match subcommand {
        Subcommands::Scaife { identifier, title, author } => {
            let source = Scaife {};
            let text = source.get_text(&identifier).unwrap();
            let latex = Latex::new().title(title).author(author);
            println!("{}", latex.format(&text));
        }
    }
}
