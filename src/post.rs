use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Response {
    post: Post,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ResponseList {
    posts: Vec<Post>,
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

    pub async fn forum_thread<T: Into<String>>(
        &self,
        forum: T,
        thread: T,
        page: u64,
    ) -> Result<Vec<Post>> {
        let mut req = self.request(
            reqwest::Method::GET,
            &format!(
                "api/v1/json/forums/{}/topics/{}/posts",
                forum.into(),
                thread.into()
            ),
        );

        if page != 0 {
            req = req.query(&[("page", format!("{}", page))])
        }

        let resp: ResponseList = req.send().await?.error_for_status()?.json().await?;
        Ok(resp.posts)
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

    #[tokio::test]
    async fn thread() {
        let _ = pretty_env_logger::try_init();
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/forum_dis_thread_amamods.json"))
                .unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/api/v1/json/forums/dis/topics/ask-the-mods-anything/posts",
            ))
            .respond_with(json_encoded(data)),
        );

        let cli =
            crate::Client::with_baseurl("test", "42069", &format!("{}", server.url("/"))).unwrap();
        cli.forum_thread("dis", "ask-the-mods-anything", 0)
            .await
            .unwrap();
    }
}
