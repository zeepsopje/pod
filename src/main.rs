use pod::Pod;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pod = Pod::new();
    let docs = pod.get_crate_docs("chrono").await?;

    for _struct in docs.structs {
        print!("{}", _struct.name);
        if _struct.is_deprecated {
            print!(" (deprecated)");
        }
        print!("\n");
    }

    Ok(())
}
