use crate::client::*;
// use ansi_escapes;
// use colorful::Color;
// use colorful::Colorful;
use serde::{Deserialize, Serialize};
// use spinners;
// use spinners::{Spinner, Spinners};
// use std::cmp::Reverse;
use std::fmt;
// use std::thread::sleep;
// use std::time::Duration;
use tabled::Tabled;

use crate::client;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct User {
    #[serde(rename(deserialize = "login", serialize = "login"))]
    pub username: String,
    pub name: String,
    #[serde(rename(deserialize = "type"))]
    pub user_type: UserType,
    pub location: String,
    pub followers: u32,
    pub following: u32,
    pub public_repos: u32,
    pub public_gists: u32,
    pub blog: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub enum UserType {
    User,
    Orgnization,
}

impl Default for UserType {
    fn default() -> Self {
        UserType::User
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.username)
    }
}

fn diplay_lang_option(opt: &Option<String>) -> String {
    match opt {
        Some(s) => format!("{}", s),
        None => format!(""),
    }
}

pub struct UserHolder {
    pub client: reqwest::Client,
    pub username: String,
}

impl UserHolder {
    pub async fn get_user(self) -> Result<User, reqwest::Error> {
        let base_url = "https://api.github.com";
        let res = self
            .client
            .get(format!("{}/users{}", base_url, self.username))
            .send()
            .await?;

        let body = res.text().await?;
        let user: User = serde_json::from_str(&body).unwrap();

        Ok(user)
    }
}

#[derive(Deserialize, Serialize, Debug, Tabled)]
pub struct Repo {
    #[header(hidden = true)]
    id: u32,
    name: String,
    #[serde(skip_serializing, skip_deserializing)]
    #[header(hidden = true)]
    owner: User,
    private: bool,
    fork: bool,
    full_name: String,
    #[field(display_with = "diplay_lang_option")]
    language: Option<String>,
    #[header("stars")]
    stargazers_count: u32,
    // disabled: bool,
    // created_at: String,
    // updated_at: String,
}

// impl Repo {
//     fn new(user: User) -> Self {
//         Repo {
//             owner: user,
//             name: String::from(""),
//             private: false,
//             id: 0,
//             fork: false,
//             full_name: String::from(""),
//             language: None,
//             stargazers_count: 0,
//             // disabled: false,
//             // created_at: String::from(""),
//             // updated_at: String::from(""),
//         }
//     }
// }

// pub struct RepoHandler {
//     client: client::Client,
//     owner: String,
//     repo: String,
// }

// impl RepoHandler {
//     fn new(client: Client, owner: String, repo: String) -> Self {
//         Self {
//             client,
//             owner,
//             repo,
//         }
//     }

//     //    async fn get_repos() {
//     //    }
// }

// async fn get_repos(&self) -> Result<Vec<Repo>, reqwest::Error> {
//     let url = format!("users/{}/repos", self.owner.username);
//     let repos_match = Client::new().get::<Vec<Repo>>(&url).await;
//     match repos_match {
//         Ok(repos) => Ok(repos),
//         Err(e) => Err(e),
//     }
// }

// pub async fn get_user(name: &str) {
//     // let url = format!("users{}", name);
//     let user_match = get_user_by_name(name).await;

//     let sp = Spinner::new(&Spinners::Dots9, "Getting user Info...".into());
//     sleep(Duration::from_secs(1));
//     sp.stop();

//     print!("{}", ansi_escapes::EraseLines(1));

//     match user_match {
//         Ok(user) => {
//             println!("@{}", user.username.color(Color::Green));
//             print!("{}   ", user.name.color(Color::Blue));
//             println!("{}", user.location);
//             println!("{} Followers  {} Following", user.followers, user.following)
//         }
//         Err(..) => {
//             //println!("Error: {}", e)
//             println!("Failed to get user");
//         }
//     }
// }

// pub async fn get_repos(name: &str) {
//     let user_match = get_user_by_name(name).await;

//     let sp = Spinner::new(&Spinners::Dots9, "Getting user Info...".into());
//     sleep(Duration::from_secs(1));
//     sp.stop();

//     print!("{}", ansi_escapes::EraseLines(1));

//     match user_match {
//         Ok(user) => {
//             let reps_matches = Repo::new(user).get_repos().await;
//             match reps_matches {
//                 Ok(mut repo) => {
//                     // sort by number of starts
//                     repo.sort_by_key(|r| Reverse(r.stargazers_count));
//                     println!(
//                         "{}",
//                         Table::new(&repo)
//                             .with(Header(format!(
//                                 "{} has {} repositories",
//                                 name.replace("/", ""),
//                                 repo.len()
//                             )))
//                             .with(Style::default())
//                             .with(
//                                 Modify::new(Row(..1)) // .with(FormatWithIndex(|_, _, colum| colum.to_string()))
//                             )
//                             .with(
//                                 Modify::new(Full)
//                                     .with(MaxWidth(28, "..."))
//                                     .with(Alignment::Horizontal(AlignmentHorizontal::Center))
//                             )
//                     );
//                 }
//                 Err(e) => {
//                     println!("Error fetching reps: {}", e);
//                 }
//             }
//         }
//         Err(..) => {
//             println!("Failed to get user");
//         }
//     }
// }
