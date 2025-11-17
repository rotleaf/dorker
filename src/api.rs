use crate::Ret;
use serde::Deserialize;
use serde_json::from_str;
use std::fs::File;
use std::io::Write;
use urlencoding::encode;

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    organic_results: Vec<OrganicResult>,
    #[serde(rename = "serpapi_pagination")]
    pagination: Pagination,
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    current: i16,
    #[serde(default)]
    next: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OrganicResult {
    link: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountInfo {
    #[serde(rename = "total_searches_left")]
    searches_left: i64,
}

pub enum SearchEngine {
    Google,
    Bing,
    DuckDuckGo,
}

pub struct Api {
    pub api_key: String,
    base: String,
}

pub struct Query {
    pub engine: SearchEngine,
    pub query: String,
}

pub struct SearchOptions {
    pub max_pages: Option<usize>,
    pub delay_ms: u64,
    pub output_file: Option<String>,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            max_pages: None, // None means fetch all pages
            delay_ms: 500,
            output_file: None,
        }
    }
}

impl Api {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.into(),
            base: "https://serpapi.com".into(),
        }
    }

    pub fn account_info(&self) -> Ret<i64> {
        let request = ureq::get(&format!("{}/account?api_key={}", self.base, self.api_key))
            .call()?
            .body_mut()
            .read_to_string()?;

        Ok(from_str::<AccountInfo>(&request)?.searches_left)
    }

    /// Simple search that returns links from the first page only
    pub fn search(&self, query: Query) -> Ret<Vec<String>> {
        let url = format!(
            "{}/search?engine={}&q={}&api_key={}",
            self.base,
            self.gengine(&query),
            query.query,
            self.api_key
        );

        let req = ureq::get(&url).call()?.body_mut().read_to_string()?;

        let parse = serde_json::from_str::<SearchResult>(&req)?;
        let links: Vec<String> = parse
            .organic_results
            .into_iter()
            .map(|ores| ores.link)
            .collect();

        Ok(links)
    }

    /// Search with pagination support
    pub fn search_paginated(&self, query: Query, options: SearchOptions) -> Ret<Vec<String>> {
        let mut all_links: Vec<String> = Vec::new();
        let initial_url = format!(
            "{}/search?engine={}&q={}&api_key={}",
            self.base,
            self.gengine(&query),
            encode(&query.query),
            self.api_key
        );

        let mut current_url = Some(initial_url);
        let mut page_count = 0;

        while let Some(url) = current_url {
            page_count += 1;

            // Check if we've reached max pages
            if let Some(max) = options.max_pages
                && page_count > max
            {
                println!("Reached maximum page limit: {}", max);
                break;
            }

            let url = if url.contains("api_key") {
                url
            } else {
                format!("{url}&api_key={}", self.api_key)
            };
            println!(">> Fetching page {}", page_count);

            // Fetch the page
            let req = ureq::get(&url).call()?.body_mut().read_to_string()?;

            // Parse results
            let parse = serde_json::from_str::<SearchResult>(&req)?;

            // Extract and collect links
            for ores in parse.organic_results {
                println!("{}", ores.link);
                all_links.push(ores.link);
            }

            // Get next page URL
            current_url = parse.pagination.next;

            if current_url.is_some() {
                println!("Moving to next page...\n");
                // Add delay to be respectful to the API
                std::thread::sleep(std::time::Duration::from_millis(options.delay_ms));
            } else {
                println!("No more pages available.\n");
            }
        }

        println!("Total links collected: {}", all_links.len());
        println!("Total pages fetched: {}", page_count);

        // Save to file if specified
        if let Some(filename) = options.output_file {
            self.save_links_to_file(&all_links, &filename)?;
        }

        Ok(all_links)
    }

    /// Helper function to save links to a file
    fn save_links_to_file(&self, links: &[String], filename: &str) -> Ret<()> {
        let mut file = File::create(filename)?;
        for link in links {
            writeln!(file, "{}", link)?;
        }
        println!("Links saved to: {}", filename);
        Ok(())
    }

    pub fn gengine(&self, query: &Query) -> &str {
        match &query.engine {
            SearchEngine::Google => "google",
            SearchEngine::Bing => "bing",
            SearchEngine::DuckDuckGo => "duckduckgo",
        }
    }
}
