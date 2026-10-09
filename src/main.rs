mod crawl;
mod fetch;
mod metrics;
mod parse;
mod storage;

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    crawl::scrape_url("https://en.wikipedia.org/wiki/Computer".to_string()).await;
    let elapsed = start.elapsed();
    let pages = crawl::pages_scraped();
    println!(
        "{pages} pages in {elapsed:.2?} = {:.1} pages/sec",
        pages as f64 / elapsed.as_secs_f64()
    );
    println!("peak concurrent fetch+dump: {}", metrics::peak_in_flight());
}
