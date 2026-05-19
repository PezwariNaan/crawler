use scraper::{Html, Selector};
use std::collections::{HashSet, VecDeque};

// TODO:
// Stop domains being added PageResult.links 
// Filter none https / http refs

#[derive(Debug)]
pub struct PageResult {
    page: String,
    links: Vec<String>,
    link_count: usize,
}

pub async fn crawl(
    start_url: &str
    ) -> Result<Vec<PageResult>, Box<dyn std::error::Error>> {
    
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut page_results = Vec::new();

    queue.push_back(start_url.to_string());

    let start = url::Url::parse(start_url)?;
    let allowed_domain = start.domain().unwrap();

    while let Some(url) = queue.pop_front() {

        let parsed = match url::Url::parse(&url) {
            Ok(u) => u,
            Err(_) => continue,
        };

        let normalised = parsed
            .as_str()
            .trim_end_matches('/')
            .to_string();

        if visited.contains(&normalised) {
            continue;
        }

        if parsed.domain() != Some(&allowed_domain) {
            continue;
        }

        visited.insert(normalised.clone());

        let page_result = get_page_links(&url).await?;

        for link in &page_result.links {
            if let Ok(parsed_link) = url::Url::parse(link) {
                if parsed_link.domain() == Some(&allowed_domain) &&
                    !visited.contains(link) {
                        queue.push_back(link.clone());
                }
            }
        }

        page_results.push(page_result);
    }

    Ok(page_results)
}

async fn get_page_links(
    url: &str
    ) -> Result<PageResult, Box<dyn std::error::Error>> {

    let body = reqwest::get(url).await?.text().await?;
    let base = url::Url::parse(url)?;

    let mut urls = Vec::new();

    let document = Html::parse_document(&body);
    let selector = Selector::parse("a")?;

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(resolved) = base.join(href) {
                urls.push(resolved.to_string());
            }
        }
    }

    let link_count = urls.len();
    let results = PageResult {
        links: urls,
        link_count,
        page: url.to_string(),
    };

    Ok(results)
}

