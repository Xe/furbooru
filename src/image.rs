use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Response {
    image: Image,
}

#[derive(Serialize, Deserialize)]
struct ResponseList {
    images: Vec<Image>,
}

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
    pub uploader: Option<String>,
    pub deletion_reason: Option<String>,
    pub width: i64,
    pub processed: bool,
    pub created_at: String,
    pub orig_sha512_hash: String,
    pub view_url: String,
    pub uploader_id: Option<i64>,
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
    pub source_url: Option<String>,
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
    /// Get information about the currently featured image.
    pub async fn featured_image(&self) -> Result<Image> {
        let resp: Response = self
            .request(reqwest::Method::GET, "api/v1/json/images/featured")
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(resp.image)
    }

    /// Get information about an image by ID.
    pub async fn image(&self, id: u64) -> Result<Image> {
        let resp: Response = self
            .request(reqwest::Method::GET, &format!("api/v1/json/images/{}", id))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(resp.image)
    }

    /// Search for images that match a set of tags.
    pub async fn image_search<T: Into<String>>(&self, q: T, page: u64) -> Result<Vec<Image>> {
        let mut req = self
            .request(reqwest::Method::GET, &format!("api/v1/json/search/images"))
            .query(&[("q", q.into())]);

        if page != 0 {
            req = req.query(&[("page", format!("{}", page))]);
        }

        let resp: ResponseList = req.send().await?.error_for_status()?.json().await?;

        Ok(resp.images)
    }
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

    #[tokio::test]
    async fn image() {
        let _ = pretty_env_logger::try_init();
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/image_2336.json")).unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/api/v1/json/images/2336"))
                .respond_with(json_encoded(data)),
        );

        let cli =
            crate::Client::with_baseurl("test", "42069", &format!("{}", server.url("/"))).unwrap();
        cli.image(2336).await.unwrap();
    }

    #[tokio::test]
    async fn image_search() {
        let _ = pretty_env_logger::try_init();
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/search_images.json")).unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/api/v1/json/search/images"))
                .respond_with(json_encoded(data)),
        );

        let cli =
            crate::Client::with_baseurl("test", "42069", &format!("{}", server.url("/"))).unwrap();
        cli.image_search("orca", 0).await.unwrap();
    }
}
