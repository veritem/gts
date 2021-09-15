use crate::client::*;
use ansi_escapes;
use colorful::Color;
use colorful::Colorful;
use serde::{Deserialize, Serialize};
use spinners;
use spinners::{Spinner, Spinners};
use std::cmp::Reverse;
use std::fmt;
use std::thread::sleep;
use std::time::Duration;
use tabled::{
    papergrid::AlignmentHorizontal, Alignment, Full, Header, MaxWidth, Modify, Row, Style, Table,
    Tabled,
};

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

async fn get_user_by_name(name: &str) -> Result<User, reqwest::Error> {
    let url = format!("users{}", name);
    let user_match = Client::new().get::<User>(&url).await;

    match user_match {
        Ok(user) => Ok(user),
        Err(e) => Err(e),
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

impl Repo {
    fn new(user: User) -> Self {
        Repo {
            owner: user,
            name: String::from(""),
            private: false,
            id: 0,
            fork: false,
            full_name: String::from(""),
            language: None,
            stargazers_count: 0,
            // disabled: false,
            // created_at: String::from(""),
            // updated_at: String::from(""),
        }
    }

    /// Get user's repostories
    // async fn get_repos(&self) -> Result<Vec<Repo>, reqwest::Error> {
    //     let url = format!("users/{}/repos", self.owner.username);
    //     let repos_match = Client::new().get::<Vec<Repo>>(&url).await;
    //     match repos_match {
    //         Ok(repos) => Ok(repos),
    //         Err(e) => Err(e),
    //     }
    // }
}

pub async fn get_user(name: &str) {
    // let url = format!("users{}", name);
    let user_match = get_user_by_name(name).await;

    let sp = Spinner::new(&Spinners::Dots9, "Getting user Info...".into());
    sleep(Duration::from_secs(1));
    sp.stop();

    print!("{}", ansi_escapes::EraseLines(1));

    match user_match {
        Ok(user) => {
            println!("@{}", user.username.color(Color::Green));
            print!("{}   ", user.name.color(Color::Blue));
            println!("{}", user.location);
            println!("{} Followers  {} Following", user.followers, user.following)
        }
        Err(..) => {
            //println!("Error: {}", e)
            println!("Failed to get user");
        }
    }
}

pub async fn get_repos(name: &str) {
    let user_match = get_user_by_name(name).await;

    let sp = Spinner::new(&Spinners::Dots9, "Getting user Info...".into());
    sleep(Duration::from_secs(1));
    sp.stop();

    print!("{}", ansi_escapes::EraseLines(1));

    match user_match {
        Ok(user) => {
            let reps_matches = Repo::new(user).get_repos().await;
            match reps_matches {
                Ok(mut repo) => {
                    // sort by number of starts
                    repo.sort_by_key(|r| Reverse(r.stargazers_count));
                    println!(
                        "{}",
                        Table::new(&repo)
                            .with(Header(format!(
                                "{} has {} repositories",
                                name.replace("/", ""),
                                repo.len()
                            )))
                            .with(Style::default())
                            .with(
                                Modify::new(Row(..1)) // .with(FormatWithIndex(|_, _, colum| colum.to_string()))
                            )
                            .with(
                                Modify::new(Full)
                                    .with(MaxWidth(28, "..."))
                                    .with(Alignment::Horizontal(AlignmentHorizontal::Center))
                            )
                    );
                }
                Err(e) => {
                    println!("Error fetching reps: {}", e);
                }
            }
        }
        Err(..) => {
            println!("Failed to get user");
        }
    }
}
