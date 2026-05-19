use scraper::{Html, Selector};
use clap::Args;

mod crawler;

#[tokio::main]
async fn main() -> () {
    let url = "http://localhost:8080";
    println!("Crawling {url}");

    match crawler::get_page_links(url).await {
        Ok(urls) => println!("Crawling {url}"),
        Err(e) => eprintln!("{e}\nInvalid URL: {url}"),
    };
}

