use std::error::Error;

use serde::Deserialize;

use thiserror::Error;

#[derive(Error, Debug)]
enum NewsApIError {
    #[error("Failed fetching articles ")]
    RequestFailed,
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

pub fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| NewsApIError::RequestFailed)?
        .into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}