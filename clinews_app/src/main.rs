use newsapi::{get_articles, Articles};
use colour::{dark_green_ln, yellow_ln};
use std::error::Error;



fn render_articles(articles: &Articles) {
    for i in &articles.articles {
        dark_green_ln!("> {}", i.title);
        yellow_ln!("> {}\n\n", i.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let url = "https://newsapi.org/v2/top-headlines?country=us&category=business&apiKey=789c58dc4026459fb27b1f142a9844f1";
    let articles = get_articles(url)?;

    render_articles(&articles); // Call the correct function name 'render_articles'

    Ok(())
}
