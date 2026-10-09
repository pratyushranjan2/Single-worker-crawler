pub fn extract_urls(contents: &str, base: &str) -> Vec<String> {
    let Ok(base) = url::Url::parse(base) else {
        return Vec::new();
    };
    let document = scraper::Html::parse_document(contents);
    let selector = scraper::Selector::parse("a[href]").unwrap();
    document
        .select(&selector)
        .filter_map(|a| a.value().attr("href"))
        .filter_map(|href| base.join(href).ok())
        .filter(|link| matches!(link.scheme(), "http" | "https"))
        .map(|mut link| {
            link.set_fragment(None);
            link.to_string()
        })
        .collect()
}
