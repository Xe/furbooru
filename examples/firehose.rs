use anyhow::Result;
use async_trait::async_trait;
use furbooru::{Client, Image, Comment};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Client::new("firehose-example", &std::env::var("FURBOORU_API_KEY")?)?;
    cli.firehose(Adaptor{}).await?;

    Ok(())
}

struct Adaptor;

#[async_trait]
impl furbooru::FirehoseAdaptor for Adaptor {
    async fn image_created(&self, img: Image) -> Result<()> {
        println!("new image: {} {} {}", img.id, img.name, img.view_url);
        Ok(())
    }

    async fn comment_created(&self, cmt: Comment) -> Result<()> {
        println!("new comment on image {}: {}", cmt.image_id, cmt.body);
        Ok(())
    }
}
