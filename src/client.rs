const API_ACCEPT: &'static str = "application/vnd.github.v3+json";
const API_USER_AGENT: &'static str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

use serde::de::DeserializeOwned;

pub struct Client {
    /// headers: String,
    base_url: String,
}

impl Client {
    pub fn new() -> Client {
        Client {
            base_url: String::from("https://api.github.com/"),
            // headers: headers.to_string(),
        }
    }

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
                let text_resp = serde_json::from_str(&resp_text).unwrap();
                Ok(text_resp)
            }
            Err(e) => Err(e),
        }
    }

    // pub async fn get_user(
    //     &self,
    //     url: &str,
    //     // opts: Option<Vec<(&str, &str)>>,
    // ) -> Result<(), reqwest::Error> {
    //     let resp = reqwest::Client::new()
    //         .get(format!("{}{}", &self.base_url, url))
    //         .header("Accept", API_ACCEPT)
    //         .header("User-Agent", API_USER_AGENT)
    //         .send()
    //         .await?;

    //     // if let Err(e) = resp {
    //     //     if e.is_connect() {
    //     //         println!("Failed to connect")
    //     //     } else if e.is_status() {
    //     //         println!("Failed to get the response")
    //     //     }
    //     // }

    //     let text_resp = resp.text().await?;

    //     let parsed_resp: User = serde_json::from_str(&text_resp).unwrap();

    //     Ok(())
    // }
}
