use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ResponseList {
    topics: Vec<Topic>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Response {
    topic: Topic,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Topic {
    pub author: String,
    pub last_replied_to_at: String,
    pub locked: bool,
    pub post_count: i64,
    pub slug: String,
    pub sticky: bool,
    pub title: String,
    pub user_id: i64,
    pub view_count: i64,
}

impl crate::Client {
    pub async fn topic<T: Into<String>>(&self, forum: T, thread: T) -> Result<Topic> {
        let resp: Response = self
            .request(
                reqwest::Method::GET,
                &format!(
                    "api/v1/json/forums/{}/topics/{}",
                    forum.into(),
                    thread.into()
                ),
            )
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp.topic)
    }
}

#[cfg(test)]
mod tests {
    use httptest::{matchers::*, responders::*, Expectation, Server};

    #[tokio::test]
    async fn forums() {
        let _ = pretty_env_logger::try_init();
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/forum_dis_topic_amamods.json"))
                .unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/api/v1/json/forums/dis/topics/ask-the-mods-anything",
            ))
            .respond_with(json_encoded(data)),
        );

        let cli =
            crate::Client::with_baseurl("test", "42069", &format!("{}", server.url("/"))).unwrap();
        cli.topic("dis", "ask-the-mods-anything").await.unwrap();
    }
}
