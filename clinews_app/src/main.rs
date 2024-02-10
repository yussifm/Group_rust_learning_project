use newsapi::{get_articles, Articles};
use std::error::Error;
use dotenv::dotenv;
use std::env;

mod theme;


use theme::default;


fn render_articles(articles: &Articles) {
    let theme = default();
    theme.print_text("# Top headlines\n\n");
    for i in &articles.articles {
        theme.print_text(&format!("`{}`", i.title));
        theme.print_text(&format!("> *{}*", i.url));
        theme.print_text("---");
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
