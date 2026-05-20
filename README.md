# Rust Web Crawler

A lightweight asynchronous web crawler written in Rust.

This project was built as part of a larger offensive security / recon tooling project to better understand how crawling, endpoint discovery, and fuzzing engines work internally.

The crawler currently supports:

* Breadth-first crawling
* Internal-domain scoping
* Relative URL resolution
* URL normalization
* Queue deduplication
* Async HTTP requests
* HTML link extraction

---

# Features

## Internal Link Crawling

The crawler automatically restricts itself to the starting domain.

Example:

```text
https://example.com
```

will crawl:

```text
https://example.com/about
https://example.com/blog
```

but ignore:

```text
https://github.com/
https://twitter.com/
```

---

## Relative URL Resolution

Relative paths are automatically resolved into full URLs.

Example:

```html
<a href="/about">
```

becomes:

```text
https://example.com/about
```

---

## URL Normalization

URLs are normalized before being queued to prevent duplicate crawling.

Current normalization includes:

* trailing slash removal

Example:

```text
https://example.com/
```

becomes:

```text
https://example.com
```

---

## Queue Deduplication

The crawler tracks:

* queued URLs
* visited URLs

This prevents duplicate scheduling and infinite crawl loops.

---

# Tech Stack

* Rust
* Tokio
* Reqwest
* Scraper
* Clap
* Url

---

# Installation

Clone the repository:

```bash
git clone https://github.com/yourname/rust-crawler.git
cd rust-crawler
```

Build the project:

```bash
cargo build --release
```

---

# Usage

Run the crawler:

```bash
cargo run -- --url https://example.com
```

Or using the compiled binary:

```bash
./target/release/rust-crawler --url https://example.com
```

---

# Example Output

```text
Crawling https://example.com

Visiting: https://example.com
Visiting: https://example.com/about
Visiting: https://example.com/blog
```

---

# Project Structure

```text
src/
├── main.rs
└── crawler.rs
```

---

# Future Improvements

Planned features include:

* Crawl depth limiting
* JavaScript extraction
* Form extraction
* Endpoint discovery
* Parameter extraction
* Response fingerprinting
* Wordlist generation
* Smart fuzzing integration
* Concurrent workers
* robots.txt support

---

# Why Build This?

The goal of this project is not just to crawl websites, but to learn:

* async systems programming
* graph traversal
* crawler architecture
* recon tooling design
* fuzzing workflows
* Rust ownership and state management

This project is part of a broader effort to build custom offensive security and bug bounty tooling from scratch.

---

# Disclaimer

This tool is intended for:

* educational use
* local testing
* authorized security research

Only test systems you own or have permission to assess.

---

# License

MIT

