use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Response {
    post: Post,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub author: String,
    pub avatar: String,
    pub body: String,
    pub created_at: String,
    pub edit_reason: Option<String>,
    pub edited_at: Option<String>,
    pub id: i64,
    pub updated_at: String,
    pub user_id: i64,
}

impl crate::Client {
    pub async fn post(&self, id: u64) -> Result<Post> {
        let resp: Response = self
            .request(reqwest::Method::GET, &format!("api/v1/json/posts/{}", id))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp.post)
    }
}

#[cfg(test)]
mod tests {
    use httptest::{matchers::*, responders::*, Expectation, Server};

    #[tokio::test]
    async fn post() {
        let _ = pretty_env_logger::try_init();
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/post_1002.json")).unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/api/v1/json/posts/1002"))
                .respond_with(json_encoded(data)),
        );

        let cli =
            crate::Client::with_baseurl("test", "42069", &format!("{}", server.url("/"))).unwrap();
        cli.post(1002).await.unwrap();
    }
}
