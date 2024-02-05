use crawl::Crawl;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let crawl = Crawl::new();
    let result = crawl.search("hello").await?;

    let docs = crawl.get_crate_docs("chrono").await?;

    // dbg!(&docs);

    Ok(())
}
