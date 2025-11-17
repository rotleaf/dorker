mod api;
mod config;
mod utilities;

use crate::{api::*, config::Config, utilities::*};
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::fs;

#[derive(Parser)]
#[command(name = "dorker")]
#[command(about = "makes dorking easier", long_about = None)]
struct Cli {
    #[arg(
        short,
        long,
        default_value = "config.toml",
        help = "the file holding your API keys"
    )]
    config_path: String,

    #[arg(short, long, default_value = "dorks.txt", help = "your output file")]
    output: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// use only google for dorking
    Google {
        #[arg(short, long, help = "your search query, preferrably a dork, lol")]
        query: String,
    },
}

fn main() -> Ret<()> {
    let cli = Cli::parse();

    let config_path = &cli.config_path;
    let config = fs::read_to_string(config_path)?;
    let cnf = serde_yaml::from_str::<Config>(&config)?;
    let keys = cnf.api_keys;

    println!(
        "{}",
        format!(
            "> Config {} contains {} api key/s",
            format!("@{}", config_path).yellow(),
            keys.len()
        )
    );

    let chosen = best_key(&keys)?;
    // search right away,

    let api = Api::new(&chosen);
    let query = match &cli.command {
        Commands::Google { query } => query.clone(),
    };
    api.search_paginated(
        Query {
            engine: SearchEngine::Google, // we love google, right,
            query,
        },
        SearchOptions {
            max_pages: None,
            delay_ms: 100,
            output_file: Some(cli.output),
        },
    )?;

    Ok(())
}
