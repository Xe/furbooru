use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Response {
    post: Post,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct ResponseList {
    posts: Vec<Post>,
}

/// A forum post
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
    pub user_id: Option<i64>,
}

impl crate::Client {
    /// Fetches a forum post by ID.
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

    /// Creates a new forum post with the given parameters.
    pub async fn create_post<T>(
        &self,
        forum: T,
        thread: T,
        body: String,
        anonymous: bool,
    ) -> Result<Post>
    where
        T: Into<String>,
    {
        #[derive(Serialize, Clone)]
        struct MakePost {
            body: String,
            anonymous: bool,
        }

        #[derive(Serialize, Clone)]
        struct Body {
            post: MakePost,
        }

        let resp: Response = self
            .request(
                reqwest::Method::POST,
                &format!(
                    "api/v1/json/forums/{}/topics/{}/posts",
                    forum.into(),
                    thread.into()
                ),
            )
            .json(&Body {
                post: MakePost {
                    body: body,
                    anonymous: anonymous,
                },
            })
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp.post)
    }

    /// Fetches page n of posts in a thread in a forum.
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
    async fn create_post() {
        let _ = pretty_env_logger::try_init();
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/post_1002.json")).unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "POST",
                "/api/v1/json/forums/dis/topics/ask-the-mods-anything/posts",
            ))
            .respond_with(json_encoded(data)),
        );

        let cli =
            crate::Client::with_baseurl("test", "42069", &format!("{}", server.url("/"))).unwrap();
        cli.create_post("dis", "ask-the-mods-anything", "Hi there".into(), false)
            .await
            .unwrap();
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
