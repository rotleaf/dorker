use crate::Ret;
use serde::Deserialize;
use serde_json::from_str;

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    // google search for now, idk the others yet
    organic_results: Vec<OrganicResult>,
    #[serde(rename = "serpapi_pagination")]
    pagination: Pagination,
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    current: i16,
    next: String,
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
    engine: SearchEngine,
    query: String,
}

impl Api {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.into(),
            base: "https://serpapi.com".into(),
        }
    }

    pub fn account_info(&self) -> Ret<i64> {
        let request = ureq::get(format!("{}/account?api_key={}", self.base, self.api_key))
            .call()?
            .body_mut()
            .read_to_string()?;

        Ok(from_str::<AccountInfo>(&request)?.searches_left)
    }

    pub fn search(&self, query: Query) -> Ret<()> {
        let req = ureq::get(format!(
            "{}/search?engine={}&q={}&api_key={}",
            self.base,
            self.gengine(&query),
            query.query, // 69
            self.api_key
        ))
        .call()?
        .body_mut()
        .read_to_string()?;

        dbg!(req);
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
