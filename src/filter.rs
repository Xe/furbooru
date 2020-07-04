use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Response {
    pub filter: Filter,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct ResponseList {
    pub filters: Vec<Filter>,
}

/// An image filter.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Filter {
    pub description: String,
    pub hidden_complex: Option<String>,
    pub hidden_tag_ids: Vec<i64>,
    pub id: i64,
    pub name: String,
    pub public: bool,
    pub spoilered_complex: ::serde_json::Value,
    pub spoilered_tag_ids: Vec<i64>,
    pub system: bool,
    pub user_count: i64,
    pub user_id: ::serde_json::Value,
}

impl crate::Client {
    /// Fetch a filter by its ID.
    pub async fn filter(&self, id: u64) -> Result<Filter> {
        let resp: Response = self
            .request(reqwest::Method::GET, &format!("api/v1/json/filters/{}", id))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp.filter)
    }

    /// Fetch the list of system filters.
    pub async fn system_filters(&self) -> Result<Vec<Filter>> {
        let resp: ResponseList = self
            .request(reqwest::Method::GET, "api/v1/json/filters/system")
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(resp.filters)
    }

    /// Fetch the list of user-level filters.
    pub async fn user_filters(&self, page: u64) -> Result<Vec<Filter>> {
        let mut req = self.request(reqwest::Method::GET, "api/v1/json/filters/user");

        if page != 0 {
            req = req.query(&[("page", format!("{}", page))])
        }

        let resp: ResponseList = req.send().await?.error_for_status()?.json().await?;

        Ok(resp.filters)
    }
}

#[cfg(test)]
mod tests {
    use httptest::{matchers::*, responders::*, Expectation, Server};

    #[tokio::test]
    async fn filter() {
        let _ = pretty_env_logger::try_init();
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/filter_1.json")).unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/api/v1/json/filters/1"))
                .respond_with(json_encoded(data)),
        );

        let cli =
            crate::Client::with_baseurl("test", "42069", &format!("{}", server.url("/"))).unwrap();
        cli.filter(1).await.unwrap();
    }

    #[tokio::test]
    async fn system_filters() {
        let _ = pretty_env_logger::try_init();
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/filters_system.json")).unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/api/v1/json/filters/system"))
                .respond_with(json_encoded(data)),
        );

        let cli =
            crate::Client::with_baseurl("test", "42069", &format!("{}", server.url("/"))).unwrap();
        cli.system_filters().await.unwrap();
    }

    #[tokio::test]
    async fn user_filters() {
        let _ = pretty_env_logger::try_init();
        let data: serde_json::Value =
            serde_json::from_slice(include_bytes!("../testdata/filters_system.json")).unwrap();
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/api/v1/json/filters/user"))
                .respond_with(json_encoded(data)),
        );

        let cli =
            crate::Client::with_baseurl("test", "42069", &format!("{}", server.url("/"))).unwrap();
        cli.user_filters(0).await.unwrap();
    }
}
