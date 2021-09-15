const API_ACCEPT: &'static str = "application/vnd.github.v3+json";
const API_USER_AGENT: &'static str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
const GITHUB_BASE_URL: &'static str = "https://api.github.com";

use crate::auth::Auth;
use reqwest::Url;
// use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    pub base_url: String,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            base_url: Url::parse(GITHUB_BASE_URL).unwrap().to_string(),
            client: reqwest::ClientBuilder::new()
                .user_agent(API_USER_AGENT)
                .build()
                .unwrap(),
        }
    }
}

impl Client {
    fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }
}

pub struct ClientBuilder {
    base_url: String,
    auth: Auth,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            base_url: format!("{}", GITHUB_BASE_URL),
            auth: Auth::default(),
        }
    }
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    // pub fn add_header(mut &self, header: &str) -> Self {}
    pub fn add_auth(mut self, token: String) -> Self {
        self.auth = Auth::PersonalToken(token);
        self
    }

    // access the current loged in user information

    pub fn build(self) -> Result<Client, reqwest::Error> {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.append(
            reqwest::header::ACCEPT,
            format!("{}", API_ACCEPT).parse().unwrap(),
        );

        if let Auth::PersonalToken(token) = self.auth {
            headers.append(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", token).parse().unwrap(),
            );
        }
        let client = reqwest::Client::builder()
            .user_agent("gts")
            .default_headers(headers)
            .build()?;

        Ok(Client {
            client,
            base_url: self.base_url,
        })
    }

    // pub async fn get<T>(&self, url: &str) -> Result<T, reqwest::Error>
    // where
    //     T: DeserializeOwned,
    // {
    //     let mut headers = reqwest::header::HeaderMap::new();

    //     headers.append(
    //         reqwest::header::ACCEPT,
    //         format!("{}", API_ACCEPT).parse().unwrap(),
    //     );
    //     headers.append(
    //         reqwest::header::USER_AGENT,
    //         format!("{}", API_USER_AGENT).parse().unwrap(),
    //     );

    //     if let Auth::PersonalToken(token) = self.auth {
    //         headers.append(
    //             reqwest::header::AUTHORIZATION,
    //             format!("Bearer {}", token).parse().unwrap(),
    //         );
    //     }

    //     let resp = reqwest::Client::new()
    //         .get(format!("{}{}", &self.base_url, url))
    //         .header("Accept", API_ACCEPT)
    //         .default_headers(headers)
    //         .send()
    //         .await;

    //     match resp {
    //         Ok(success) => {
    //             let resp_text = success.text().await?;
    //             let text_match = serde_json::from_str(&resp_text).unwrap();
    //             Ok(text_match)
    //         }
    //         Err(e) => Err(e),
    //     }
    // }
}
