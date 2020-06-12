use reqwest;
use serde::{Deserialize, Serialize};

pub mod image;

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
    pub fn new<T: Into<String>>(user_agent: T, token: T) -> reqwest::Result<Self> {
        let cli = reqwest::Client::builder()
            .user_agent(&format!("{} {}", user_agent.into(), APP_USER_AGENT))
            .build()?;
        let cli = Client {
            cli: cli.into(),
            token: token.into(),
            api_base: "https://furbooru.org".into(),
        };
        Ok(cli)
    }

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
