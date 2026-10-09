use crate::fetch::extract_page_contents;
use crate::metrics::InFlightGuard;
use crate::parse::extract_urls;
use crate::storage::dump_contents;
use dashmap::DashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicUsize, Ordering};

const MAX_PAGES: usize = 500;

static SCRAPED_PAGES: AtomicUsize = AtomicUsize::new(0);

static VISITED: LazyLock<DashMap<String, bool>> = LazyLock::new(DashMap::new);

pub fn pages_scraped() -> usize {
    SCRAPED_PAGES.load(Ordering::Relaxed)
}

pub fn scrape_url(url: String) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    Box::pin(async move {
        if SCRAPED_PAGES.load(Ordering::Relaxed) >= MAX_PAGES {
            return;
        }

        if VISITED.get(&url).is_some_and(|visited| *visited) {
            return;
        }

        // Another task may have claimed this url between the get and the insert.
        if VISITED.insert(url.clone(), true).unwrap_or(false) {
            return;
        }

        // Reserve a slot up front so concurrent tasks cannot overshoot MAX_PAGES.
        let reserved = SCRAPED_PAGES.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| {
            (n < MAX_PAGES).then_some(n + 1)
        });
        if reserved.is_err() {
            VISITED.insert(url, false);
            return;
        }

        // In flight covers the fetch and the dump, but not the wait on child tasks.
        let contents = {
            let _in_flight = InFlightGuard::new();
            let contents = match extract_page_contents(url.clone()).await {
                Ok(contents) => contents,
                Err(e) => {
                    eprintln!("failed to fetch {url}: {e}");
                    SCRAPED_PAGES.fetch_sub(1, Ordering::Relaxed);
                    VISITED.insert(url, false);
                    return;
                }
            };

            if let Err(e) = dump_contents(&contents, &url).await {
                eprintln!("failed to dump {url}: {e}");
                SCRAPED_PAGES.fetch_sub(1, Ordering::Relaxed);
                VISITED.insert(url, false);
                return;
            }
            contents
        };

        let handles: Vec<_> = extract_urls(&contents, &url)
            .into_iter()
            .map(|link| tokio::spawn(scrape_url(link)))
            .collect();
        for handle in handles {
            let _ = handle.await;
        }
    })
}
