use clap::Parser;

mod crawler;

#[derive(Parser, Debug)]
struct Args { 
    /// Url to crawl
    #[arg(short, long, required = true)]
    url: String,

    /// Maximum depth of search
    #[arg(short, long)]
    max_depth: Option<usize>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let start_url = &args.url;
    println!("Crawling {start_url}");
    let mut crawler = crawler::Crawler::new(start_url, args.max_depth)?;
    crawler.crawl().await?;

    for result in crawler.results() {
        println!("{:?}", result);
    }

    Ok(())
}

