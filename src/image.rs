use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub name: String,
    pub faves: i64,
    pub format: String,
    pub updated_at: String,
    pub downvotes: i64,
    pub duplicate_of: Option<u64>,
    pub tag_count: i64,
    pub spoilered: bool,
    pub uploader: String,
    pub deletion_reason: Option<String>,
    pub width: i64,
    pub processed: bool,
    pub created_at: String,
    pub orig_sha512_hash: String,
    pub view_url: String,
    pub uploader_id: i64,
    pub intensities: Intensities,
    pub score: i64,
    pub height: i64,
    pub mime_type: String,
    pub tag_ids: Vec<i64>,
    pub wilson_score: f64,
    pub first_seen_at: String,
    pub tags: Vec<String>,
    pub id: i64,
    pub upvotes: i64,
    pub comment_count: i64,
    pub representations: Representations,
    pub thumbnails_generated: bool,
    pub aspect_ratio: f64,
    pub hidden_from_users: bool,
    pub sha512_hash: String,
    pub source_url: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Intensities {
    pub ne: f64,
    pub nw: f64,
    pub se: f64,
    pub sw: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Representations {
    pub full: String,
    pub large: String,
    pub medium: String,
    pub small: String,
    pub tall: String,
    pub thumb: String,
    pub thumb_small: String,
    pub thumb_tiny: String,
}

impl crate::Client {
    async fn featured_image(&self) -> anyhow::Result<Image> {
        let resp: Response = self
            .request(reqwest::Method::GET, "api/v1/json/images/featured")
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(resp.image)
    }
}

#[derive(Serialize, Deserialize)]
struct Response {
    image: Image,
    interactions: Vec<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use httptest::{matchers::*, responders::*, Expectation, Server};
    #[tokio::test]
    async fn featured_image() {
        let _ = pretty_env_logger::try_init();
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/featured.json")).unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/api/v1/json/images/featured"))
                .respond_with(json_encoded(data)),
        );

        let cli =
            crate::Client::with_baseurl("test", "42069", &format!("{}", server.url("/"))).unwrap();
        cli.featured_image().await.unwrap();
    }
}
