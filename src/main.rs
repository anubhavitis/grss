use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct CliConfig {
    query: String,
    path: PathBuf,
    #[clap(short, long)]
    print_line: bool,
}

fn main() {
    let config = CliConfig::parse();
    let content = std::fs::read_to_string(&config.path)
        .map_err(|err| {
            eprintln!(
                "Not able to read file: {}\nError: {}",
                config.path.display(),
                err
            );
        })
        .expect("File not found");

    if config.print_line {
        grss::print_matches(&config.query, &content)
    }

    grss::count_matches(&config.query, &content)
}
