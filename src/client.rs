const API_ACCEPT: &'static str = "application/vnd.github.v3+json";
const API_USER_AGENT: &'static str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

use serde::de::DeserializeOwned;

// use crate::auth::Auth;

pub struct Client {
    // headers: String,
    base_url: String,
    //auth: Auth,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            base_url: String::from("https://api.github.com/"),
            //  auth: Auth::default(),
        }
    }
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    // pub fn add_header(mut &self, header: &str) -> Self {}
    // pub fn add_auth(mut self, token: &str) -> Self {
    //     self.auth = Auth::PersonalToken;
    //     self
    // }

    pub async fn get<T>(&self, url: &str) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        let resp = reqwest::Client::new()
            .get(format!("{}{}", &self.base_url, url))
            .header("Accept", API_ACCEPT)
            .header("User-Agent", API_USER_AGENT)
            .send()
            .await;

        match resp {
            Ok(success) => {
                let resp_text = success.text().await?;
                let text_match = serde_json::from_str(&resp_text).unwrap();
                Ok(text_match)
            }
            Err(e) => Err(e),
        }
    }
}
