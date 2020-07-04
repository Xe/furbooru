use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct ResponseList {
    pub forums: Vec<Forum>,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Response {
    pub forum: Forum,
}

/// A discussion forum.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Forum {
    pub description: String,
    pub name: String,
    pub post_count: i64,
    pub short_name: String,
    pub topic_count: i64,
}

impl crate::Client {
    /// Get the list of forums.
    pub async fn forums(&self) -> Result<Vec<Forum>> {
        let resp: ResponseList = self
            .request(reqwest::Method::GET, "api/v1/json/forums")
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp.forums)
    }

    /// Get details about an individual forum by ID.
    pub async fn forum<T: Into<String>>(&self, id: T) -> Result<Forum> {
        let resp: Response = self
            .request(
                reqwest::Method::GET,
                &format!("api/v1/json/forums/{}", id.into()),
            )
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp.forum)
    }
}

#[cfg(test)]
mod tests {
    use httptest::{matchers::*, responders::*, Expectation, Server};

    #[tokio::test]
    async fn forums() {
        let _ = pretty_env_logger::try_init();
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/forums.json")).unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/api/v1/json/forums"))
                .respond_with(json_encoded(data)),
        );

        let cli =
            crate::Client::with_baseurl("test", "42069", &format!("{}", server.url("/"))).unwrap();
        cli.forums().await.unwrap();
    }

    #[tokio::test]
    async fn forum() {
        let _ = pretty_env_logger::try_init();
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/forum_dis.json")).unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/api/v1/json/forums/dis"))
                .respond_with(json_encoded(data)),
        );

        let cli =
            crate::Client::with_baseurl("test", "42069", &format!("{}", server.url("/"))).unwrap();
        cli.forum("dis").await.unwrap();
    }
}
