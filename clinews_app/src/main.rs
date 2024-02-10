use newsapi::{get_articles, Articles};
use colour::{dark_green_ln, yellow_ln};
use std::error::Error;
use dotenv::dotenv;
use std::env;



fn render_articles(articles: &Articles) {
    for i in &articles.articles {
        dark_green_ln!("> {}", i.title);
        yellow_ln!("> {}\n\n", i.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    dotenv()?;

    let api_key = env::var("API_KEY")?;

    let url = format!("https://newsapi.org/v2/top-headlines?country=us&category=business&apiKey={}", api_key);
    let articles = get_articles(&url)?;

    render_articles(&articles); 

    Ok(())
}
