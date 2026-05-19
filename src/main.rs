use scraper::{Html, Selector};
use clap::Args;

mod crawler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_url = "http://localhost:8080";
    println!("Crawling {start_url}");
    let results = crawler::crawl(start_url).await?;

    for result in results {
        println!("{:?}", result);
    }

    Ok(())
}

