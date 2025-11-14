mod config;
mod utilities;

use crate::{config::Config, utilities::Ret};
use clap::{Parser, Subcommand};
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
    output: Option<String>,

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
    
    let config = fs::read_to_string(cli.config_path)?;
    let cnf = serde_yaml::from_str::<Config>(&config)?;
    let keys = cnf.api_keys;
    
    for key in keys {
        // okay, here they are, randomly pick orr?
    }
    Ok(())
}

pub enum SearchEngine {
    Google,
    Bing,
    DuckDuckGo,
}

pub struct QueryApi {
    pub query: String,
    pub engine: SearchEngine,
    pub api_key:String
}

impl QueryApi {
    pub fn new(query: &str, engine: SearchEngine, api_key:&str) -> Self {
        Self {
            query: query.into(),
            engine,
            api_key: api_key.into()
        }
    }

    pub fn search(&self) -> Ret<()> {
        let req = ureq::get(format!("https://serpapi.com/search?engine={}&q={}&api_key={}", self.gengine(), self.query, self.api_key)).call()?.body_mut().read_to_string()?;
        
        dbg!(req);
        Ok(())
    }

    pub fn gengine(&self) -> &str {
        match &self.engine {
            SearchEngine::Google => "google",
            SearchEngine::Bing => "bing",
            SearchEngine::DuckDuckGo => "duckduckgo",
        }
    }
}
