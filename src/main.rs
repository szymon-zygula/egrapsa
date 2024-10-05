use clap::{Parser, Subcommand};
use egrapsa::formatters::{latex::Latex, TextFormatter, Work};
use egrapsa::text_sources::{scaife::Scaife, TextSource};
use itertools::Itertools;

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
        identifiers: Vec<String>,
        #[arg(short, long)]
        main_title: Option<String>,
        #[arg(short, long)]
        titles: Option<String>,
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
        Subcommands::Scaife {
            identifiers,
            main_title,
            titles,
            author,
        } => {
            let mut latex = Latex::new().title(main_title).author(author);

            for (id, title) in identifiers.iter().zip_eq(titles) {
                let source = Scaife {};
                let text = source.get_text(id).unwrap();
                latex = latex.add_work(Work { text, title });
            }

            println!("{}", latex.format());
        }
    }
}
