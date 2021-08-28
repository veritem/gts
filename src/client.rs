static API_ACCEPT: &'static str = "application/vnd.github.v3+json";
static API_USER_AGENT: &'static str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    #[serde(rename(deserialize = "login"))]
    pub username: String,
    pub name: String,
    #[serde(rename(deserialize = "type"))]
    pub user_type: UserType,
    pub location: String,
    pub followers: u32,
    pub follows: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserType {
    User,
    Orgnization,
}

pub struct Client {
    pub headers: String,
    pub base_url: String,
}

impl Client {
    pub fn new(headers: &str) -> Client {
        Client {
            base_url: String::from("https://api.github.com"),
            headers: headers.to_string(),
        }
    }

    pub async fn get_user(
        &self,
        url: &str,
        // opts: Option<Vec<(&str, &str)>>,
    ) -> Result<(), reqwest::Error> {
        let resp = reqwest::Client::new()
            .get(format!("{}{}", &self.base_url, url))
            .header("Accept", API_ACCEPT)
            .header("User-Agent", API_USER_AGENT)
            .send()
            .await?;

        let text_resp = resp.text().await?;

        let parsed_resp: User = serde_json::from_str(&text_resp).unwrap();

        println!("{:?}", parsed_resp);

        Ok(())
    }
}
