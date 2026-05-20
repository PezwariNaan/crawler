use clap::Parser;

mod crawler;

#[derive(Parser, Debug)]
struct Args { 
    /// Url to crawl
    #[arg(short, long, required = true)]
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let start_url = &args.url;
    println!("Crawling {start_url}");
    let crawler = crawler::Crawler::new(start_url);
    let results = crawler.expect("Failed to create crawler").crawl().await;

    for result in results {
        println!("{:?}", result);
    }

    Ok(())
}

