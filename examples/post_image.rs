use furbooru::{Client, Image, ImageMeta, Result};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let cli = Client::with_baseurl(
        "Testificate",
        &std::env::var("BOORU_API_KEY")?,
        "http://127.0.0.1:8080/",
    )?;

    let img = cli.post_image("https://christine.website/static/img/tarot_death.jpg".into(), ImageMeta{
        description: "this is a test upload".into(),
        tag_input: "oc:cadey (cadey), artist:cadey, vector, haskell, orcadragon, orca, dragon, safe".into(),
        source_url: "https://christine.website/static/img/".into(),
    }).await?;

    println!("{:?}", img);

    Ok(())
}
