use reqwest::Client;
use serde::{Serialize, Deserialize};

use crate::{
    error::Error,
    docs::Documentation,
};

static DOCS_BASE: &str = "https://docs.rs";
static CRATES_BASE: &str = "https://crates.io/api/v1";
static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

#[derive(Deserialize, Debug)]
pub struct Crate {
    pub name: String,
    pub newest_version: String,
}

#[derive(Deserialize, Debug)]
pub struct SearchResults {
    pub crates: Vec<Crate>,
}

#[derive(Serialize)]
pub struct SearchQueryParams {
    q: String,
    page: Option<u64>,
    per_page: Option<u64>,
}

pub struct Pod {
    crates_client: reqwest::Client,
    docs_client: reqwest::Client,
}

impl Pod {
    pub fn new() -> Self {
        let crates_client = Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .unwrap();
        let docs_client = Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .unwrap();

        Self {
            crates_client,
            docs_client,
        }
    }

    pub async fn search(&self, term: &str) -> Result<SearchResults, Error> {
        let params = SearchQueryParams { q: term.into(), page: None, per_page: None };
        let params = serde_qs::to_string(&params).unwrap();
        let res = self.crates_client.get(format!("{}/crates?{}", CRATES_BASE, params))
            .send()
            .await?
            .json::<SearchResults>()
            .await?;

        Ok(res)
    }

    pub async fn get_crate_docs(&self, crate_name: &str) -> Result<Documentation, Error> {
        let uri = format!("{0}/{1}/latest/{1}/", DOCS_BASE, crate_name);
        let html = self.docs_client.get(uri)
            .send()
            .await?
            .text()
            .await?;
        let docs = Documentation::from_raw_html(&html)?;

        Ok(docs)
    }
}
