/*!
A [Furbooru](https://furbooru.org) and [Derpibooru](https://derpibooru.org) client
written in Rust. The APIs for these two sites are near identical, so this crate
can work with both; however it is optimized for Furbooru. Any time Furbooru diverges
from Derpibooru, this crate will follow the Furbooru changes first.

Usage is simple:

```
// don't put this in your code
std::env::set_var("API_USERNAME", "Alicia");
std::env::set_var("API_TOKEN", "42069");

let user_agent = format!(
  "{}/{} ({}, +{})",
  env!("CARGO_PKG_NAME"),
  env!("CARGO_PKG_VERSION"),
  std::env::var("API_USERNAME").unwrap(),
  env!("CARGO_PKG_REPOSITORY"),
);

let cli = furbooru::Client::new(
  user_agent,
  std::env::var("API_TOKEN").unwrap(),
).unwrap();
```

Set the environment variables `API_USERNAME` and `API_TOKEN` to your
Furbooru/Derpibooru username and API token respectively. Adding the username
associated with your bot to each request can help the booru staff when your bot
does unwanted things like violating rate limits.
*/

pub mod comment;
pub mod filter;
pub mod firehose;
pub mod forum;
pub mod image;
pub mod post;
pub mod profile;
pub mod tag;
pub mod topic;

pub use anyhow::Result;

pub use comment::Comment;
pub use filter::Filter;
pub use firehose::{FirehoseAdaptor, Message};
pub use forum::Forum;
pub use image::{Image, ImageMeta, Intensities, Representations};
pub use post::Post;
pub use profile::{Award, Link, User};
pub use tag::Tag;
pub use topic::Topic;

pub struct Client {
    pub(crate) cli: reqwest::Client,
    token: String,
    api_base: String,
}

static APP_USER_AGENT: &str = concat!(
    "library",
    "/",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " +https://github.com/Xe/furbooru",
);

impl Client {
    /// Create a new client targeting Furbooru.
    pub fn new<T: Into<String>>(user_agent: T, token: T) -> reqwest::Result<Self> {
        let cli = reqwest::Client::builder()
            .user_agent(&format!("{} {}", user_agent.into(), APP_USER_AGENT))
            .build()?;
        let cli = Client {
            cli: cli.into(),
            token: token.into(),
            api_base: "https://furbooru.org/".into(),
        };
        Ok(cli)
    }

    /// Create a new client targeting Derpibooru.
    pub fn derpi<T: Into<String>>(user_agent: T, token: T) -> reqwest::Result<Self> {
        let cli = reqwest::Client::builder()
            .user_agent(&format!("{} {}", user_agent.into(), APP_USER_AGENT))
            .build()?;
        let cli = Client {
            cli: cli.into(),
            token: token.into(),
            api_base: "https://derpibooru.org/".into(),
        };
        Ok(cli)
    }

    /// Create a new client targeting any server you want.
    pub fn with_baseurl<T: Into<String>>(
        user_agent: T,
        token: T,
        api_base: T,
    ) -> reqwest::Result<Self> {
        let cli = reqwest::Client::builder()
            .user_agent(&format!("{} {}", user_agent.into(), APP_USER_AGENT))
            .build()?;
        let cli = Client {
            cli: cli.into(),
            token: token.into(),
            api_base: api_base.into(),
        };
        Ok(cli)
    }

    fn request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let url = reqwest::Url::parse(&format!("{}{}", self.api_base, path)).unwrap();
        self.cli.request(method, url).query(&[("key", &self.token)])
    }
}

#[cfg(test)]
mod tests {
    use httptest::{matchers::*, responders::*, Expectation, Server};
    #[tokio::test]
    async fn basic() {
        let _ = pretty_env_logger::try_init();

        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/foo"))
                .respond_with(status_code(200)),
        );

        let cli =
            super::Client::with_baseurl("test", "42069", &format!("{}", server.url("/"))).unwrap();
        let resp = cli
            .request(reqwest::Method::GET, "foo")
            .send()
            .await
            .unwrap();

        assert_eq!(200, resp.status().as_u16());
    }
}
