use std::sync::LazyLock;

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
        .build()
        .unwrap()
});

pub async fn extract_page_contents(page: String) -> Result<String, reqwest::Error> {
    CLIENT.get(page).send().await?.error_for_status()?.text().await
}
