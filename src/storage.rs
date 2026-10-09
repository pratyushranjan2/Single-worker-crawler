pub async fn dump_contents(contents: &str, url: &str) -> std::io::Result<()> {
    let filename: String = url
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || matches!(c, '.' | '-') { c } else { '_' })
        .collect();
    tokio::fs::create_dir_all("data").await?;
    tokio::fs::write(format!("data/{filename}"), contents).await
}
