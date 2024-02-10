use std::error::Error;

use serde::Deserialize;

use thiserror::Error;

#[derive(Error, Debug)]
enum NewsApIError {
    #[error("Failed fetching articles ")]
    RequestFailed(ureq::Error),
    #[error("Failed converting response to string ")]
    FailedResponseToString(std::io::Error),
    #[error("Article Parsing failed ")]
    FailedResponseToJson(serde_json::Error),
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
        .map_err(|e| NewsApIError::RequestFailed(e))?
        .into_string()
        .map_err(|e| NewsApIError::FailedResponseToString(e))?;

    let articles: Articles =
        serde_json::from_str(&response).map_err(|e| NewsApIError::FailedResponseToJson(e))?;

    Ok(articles)
}
