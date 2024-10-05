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
        #[arg(short, long, value_parser, num_args = 1.., value_delimiter = '|')]
        identifiers: Vec<String>,
        #[arg(short, long, value_parser, num_args = 1.., value_delimiter = '|')]
        titles: Vec<String>,
        #[arg(short = 'A', long, value_parser, num_args = 1.., value_delimiter = '|')]
        alt_titles: Vec<String>,
        #[arg(short, long)]
        main_title: Option<String>,
        #[arg(short, long)]
        author: Option<String>,
        #[arg(short, long)]
        catchwords: bool,
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
            alt_titles,
            author,
            catchwords,
        } => {
            let mut latex = Latex::new()
                .title(main_title)
                .author(author)
                .catchwords(catchwords);

            let alt_titles = if alt_titles.is_empty() {
                vec![None; titles.len()]
            } else {
                alt_titles.into_iter().map(|x| Some(x)).collect()
            };

            for ((id, title), alt_title) in identifiers.iter().zip_eq(titles).zip_eq(alt_titles) {
                let source = Scaife {};
                let text = source.get_text(id).unwrap();
                latex = latex.add_work(Work {
                    title,
                    alt_title,
                    text,
                });
            }

            println!("{}", latex.format());
        }
    }
}
