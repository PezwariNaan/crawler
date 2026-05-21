use scraper::{Html, Selector};
use std::collections::{HashSet, VecDeque};

// TODO:
// Add max depth argument
// Option to print links per page
// Export option

#[derive(Debug)]
pub struct PageResult {
    page: String,
    links: Vec<String>,
    link_count: usize,
}

pub struct Crawler {
    queue: VecDeque<(String, usize)>,
    queued: HashSet<String>,
    visited: HashSet<String>,
    allowed_domain: String,
    max_depth: Option<usize>,
    results: Vec<PageResult>,
}

impl Crawler {
    pub fn new(
        start_url: &str,
        max_depth: Option<usize>
    ) -> Result<Self, Box< dyn std::error::Error>> {
        let start = url::Url::parse(start_url)?;

        let allowed_domain = start
            .domain()
            .unwrap()
            .to_string();

        let mut crawler = Self {
            queue: VecDeque::new(),
            queued: HashSet::new(),
            visited: HashSet::new(),
            results: Vec::new(),
            allowed_domain,
            max_depth,
        };

        crawler.enqueue_url(start_url, 0);
        
        Ok(crawler)
    }

    pub async fn crawl(
        &mut self
    ) -> Result<(), Box<dyn std::error::Error>> {
        while let Some((url, depth)) = self.queue.pop_front() {
            println!("Visiting: {}", url);
            self.queued.remove(&url);

            if let Some(max_depth) = self.max_depth {
                if depth > max_depth {
                    continue;
                }
            }

            if self.visited.contains(&url) {
                continue;
            }

            self.visited.insert(url.clone());

            let page_result = get_page_links(&url).await?;

            for link in &page_result.links {
                self.enqueue_url(link, depth + 1);
            }

            self.results.push(page_result);
        }

        Ok(())
    }

    pub fn results(&self) -> &[PageResult] {
        &self.results
    }

    pub fn into_results(self) -> Vec<PageResult> {
        self.results
    }

    fn normalise_url(&self, url: &url::Url) -> String {
        url.as_str().trim_end_matches('/').to_string()
    }

    fn in_scope(&self, url: &url::Url) -> bool {
        url.domain() == Some(&self.allowed_domain)
    }

    fn enqueue_url(
        &mut self,
        url: &str,
        depth: usize,
    ) {
        let parsed = match url::Url::parse(url) {
            Ok(u) => u,
            Err(_) => return,
        };

        let normalised = self.normalise_url(&parsed);

        if self.in_scope(&parsed)
            && !self.visited.contains(&normalised)
                && !self.queued.contains(&normalised)
        {
            self.queued.insert(normalised.clone());
            self.queue.push_back((normalised, depth));
        }
    }
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

