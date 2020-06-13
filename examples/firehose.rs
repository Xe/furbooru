use anyhow::Result;
use furbooru::{Client, Message};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::try_init()?;
    let cli = Client::new("firehose-example", &std::env::var("FURBOORU_API_KEY")?)?;
    cli.firehose(callback).await?;

    log::info!("this should be impossible");
    Ok(())
}

async fn callback(msg: Message) -> Result<()> {
    println!("{:?}", msg);

    match msg {
        Message::ImageCreate(img) => {
            println!("new image: {} {} {}", img.id, img.name, img.view_url);
        }
        Message::CommentCreate(cmt) => {
            println!("new comment on image {}: {}", cmt.image_id, cmt.body);
        }
        _ => {}
    }

    Ok(())
}
