use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Response {
    tag: Tag,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ResponseList {
    tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub aliased_tag: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub category: String,
    pub description: Option<String>,
    pub dnp_entries: Vec<::serde_json::Value>, // TODO(Xe): update this when furbooru has a DNP entry
    pub id: i64,
    pub images: i64,
    pub implied_by_tags: Option<Vec<String>>,
    pub implied_tags: Vec<::serde_json::Value>,
    pub name: String,
    pub name_in_namespace: String,
    pub namespace: Option<String>,
    pub short_description: Option<String>,
    pub slug: String,
    pub spoiler_image_uri: Option<String>,
}

impl crate::Client {
    pub async fn tag<T: Into<String>>(&self, name: T) -> Result<Tag> {
        let name = name.into().replace(":", "-colon-");
        let resp: Response = self
            .request(reqwest::Method::GET, &format!("api/v1/json/tags/{}", name))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp.tag)
    }

    pub async fn tag_search<T: Into<String>>(&self, q: T, page: u64) -> Result<Vec<Tag>> {
        let mut req = self
            .request(reqwest::Method::GET, &format!("api/v1/json/search/tags"))
            .query(&[("q", q.into())]);

        if page != 0 {
            req = req.query(&[("page", format!("{}", page))]);
        }

        let resp: ResponseList = req.send().await?.error_for_status()?.json().await?;

        Ok(resp.tags)
    }
}

#[cfg(test)]
mod tests {
    use httptest::{matchers::*, responders::*, Expectation, Server};

    #[tokio::test]
    async fn tag_name() {
        let _ = pretty_env_logger::try_init();
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/tag_artist-colon-atryl.json"))
                .unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/api/v1/json/tags/artist-colon-atryl",
            ))
            .respond_with(json_encoded(data)),
        );

        let cli =
            crate::Client::with_baseurl("test", "42069", &format!("{}", server.url("/"))).unwrap();
        cli.tag("artist:atryl").await.unwrap();
    }

    #[tokio::test]
    async fn tag_search() {
        let _ = pretty_env_logger::try_init();
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/search_tags.json")).unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/api/v1/json/search/tags"))
                .respond_with(json_encoded(data)),
        );

        let cli =
            crate::Client::with_baseurl("test", "42069", &format!("{}", server.url("/"))).unwrap();
        cli.tag_search("orca", 0).await.unwrap();
    }
}
