use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Response {
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub avatar_url: String,
    pub awards: Vec<Award>,
    pub comments_count: i64,
    pub created_at: String,
    pub description: ::serde_json::Value,
    pub id: i64,
    pub links: Vec<Link>,
    pub name: String,
    pub posts_count: i64,
    pub role: String,
    pub slug: String,
    pub topics_count: i64,
    pub uploads_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Award {
    pub awarded_on: String,
    pub id: i64,
    pub image_url: String,
    pub label: Option<String>,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub created_at: String,
    pub state: String,
    pub tag_id: i64,
    pub user_id: i64,
}

impl crate::Client {
    pub async fn profile(&self, id: u64) -> Result<User> {
        let resp: Response = self
            .request(
                reqwest::Method::GET,
                &format!("api/v1/json/profiles/{}", id),
            )
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp.user)
    }
}

#[cfg(test)]
mod tests {
    use httptest::{matchers::*, responders::*, Expectation, Server};

    #[tokio::test]
    async fn profile() {
        let _ = pretty_env_logger::try_init();
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/profile_237.json")).unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/api/v1/json/profiles/237"))
                .respond_with(json_encoded(data)),
        );

        let cli =
            crate::Client::with_baseurl("test", "42069", &format!("{}", server.url("/"))).unwrap();
        cli.profile(237).await.unwrap();
    }
}
