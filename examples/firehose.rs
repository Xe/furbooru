use anyhow::Result;
use furbooru::{Client, Message};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Client::new("firehose-example", &std::env::var("FURRYBOORU_API_TOKEN")?)?;
    cli.firehose(callback).await?;

    Ok(())
}

async fn callback(msg: Message) -> Result<()> {
    println!("{:?}", msg);

    Ok(())
}
