use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Response {
    comment: Comment,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub author: String,
    pub avatar: String,
    pub body: String,
    pub created_at: String,
    pub edit_reason: ::serde_json::Value,
    pub edited_at: ::serde_json::Value,
    pub id: i64,
    pub image_id: i64,
    pub updated_at: String,
    pub user_id: i64,
}

impl crate::Client {
    pub async fn comment(&self, id: u64) -> Result<Comment> {
        let resp: Response = self
            .request(
                reqwest::Method::GET,
                &format!("api/v1/json/comments/{}", id),
            )
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp.comment)
    }
}

#[cfg(test)]
mod tests {
    use httptest::{matchers::*, responders::*, Expectation, Server};

    #[tokio::test]
    async fn comment() {
        let _ = pretty_env_logger::try_init();
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/comment_1.json")).unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/api/v1/json/comments/1"))
                .respond_with(json_encoded(data)),
        );

        let cli =
            crate::Client::with_baseurl("test", "42069", &format!("{}", server.url("/"))).unwrap();
        cli.comment(1).await.unwrap();
    }
}
