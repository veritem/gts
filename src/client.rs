const API_ACCEPT: &'static str = "application/vnd.github.v3+json";
const API_USER_AGENT: &'static str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
const GITHUB_BASE_URL: &'static str = "https://api.github.com";

use crate::auth::Auth;
use reqwest::Url;

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    pub base_url: String,
    auth: Auth,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            base_url: Url::parse(GITHUB_BASE_URL).unwrap().to_string(),
            client: reqwest::ClientBuilder::new()
                .user_agent(API_USER_AGENT)
                .build()
                .unwrap(),
            auth: Auth::default(),
        }
    }
}

impl Client {
    pub fn add_auth(&mut self, token: String) -> &Self {
        self.auth = Auth::PersonalToken(token);
        self
    }

    pub fn build(self) -> Result<reqwest::Client, reqwest::Error> {
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

        Ok(client)
    }
}
