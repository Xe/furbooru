use async_trait::async_trait;
use furbooru::{Client, Comment, Forum, Image, Post, Result, Topic};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let cli = Client::new("firehose-example", &std::env::var("FURBOORU_API_KEY")?)?;
    log::info!("subscribed and waiting");
    cli.firehose(Adaptor {}).await?;

    Ok(())
}

struct Adaptor;

#[async_trait]
impl furbooru::FirehoseAdaptor for Adaptor {
    async fn image_created(&self, img: Image) -> Result<()> {
        println!(
            "new image: https://furbooru.org/{} {} {}",
            img.id, img.name, img.view_url
        );
        Ok(())
    }

    async fn image_description_updated(
        &self,
        image_id: u64,
        added: String,
        removed: String,
    ) -> Result<()> {
        println!(
            "image description of https://furbooru.org/{} changed from {:?} to {:?}",
            image_id, removed, added
        );
        Ok(())
    }

    async fn image_updated(&self, img: Image) -> Result<()> {
        println!(
            "image https://furbooru.org/{} updated: {}",
            img.id, img.view_url
        );
        Ok(())
    }

    async fn comment_created(&self, cmt: Comment) -> Result<()> {
        println!("new comment on image {}: {}", cmt.image_id, cmt.body);
        Ok(())
    }

    async fn comment_updated(&self, cmt: Comment) -> Result<()> {
        println!("comment on image {} updated: {}", cmt.image_id, cmt.body);
        Ok(())
    }

    async fn post_created(&self, frm: Forum, top: Topic, pst: Post) -> Result<()> {
        // https://furbooru.org/forums/art/topics/nsfw-artists-group-chat?post_id=433#post_433
        println!("new forum post: https://furbooru.org/forums/{forum}/topics/{topic}?post_id={post}#post_{post}", forum=frm.short_name, topic=top.slug, post=pst.id);
        Ok(())
    }
}
