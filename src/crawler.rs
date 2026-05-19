use scraper::{Html, Selector};

pub async fn get_page_links(
    url: &str
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {

    let body = reqwest::get(url).await?.text().await?;
    let mut urls = Vec::new();

    let document = Html::parse_document(&body);
    let selector = Selector::parse("a")?;

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            println!("{href}");
            urls.push(href.to_string());
        }
    }

    Ok(urls)
}

