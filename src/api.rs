use crate::client::*;
use ansi_escapes;
use colorful::Color;
use colorful::Colorful;
use serde::{Deserialize, Serialize};
use spinners;
use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

#[derive(Deserialize, Serialize, Debug)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum UserType {
    User,
    Orgnization,
}

impl User {
    pub fn new(&self) -> Self {
        Self {
            username: String::from(""),
            name: String::from(""),
            user_type: UserType::User,
            location: String::from(""),
            followers: 0,
            following: 0,
        }
    }
}

// impl Default for User {
//     fn Default() -> Self {
//         User::Default()
//     };
// }

async fn get_user_by_name(name: &str) -> Result<User, reqwest::Error> {
    let url = format!("users{}", name);
    let user_match = Client::new().get::<User>(&url).await;

    match user_match {
        Ok(user) => Ok(user),
        Err(e) => Err(e),
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Repo {
    #[serde(skip_serializing, skip_deserializing)]
    user: User,
    private: bool,
    name: String,
    id: u32,
    fork: bool,
    full_name: String,
    language: String,
    stargazers_count: u32,
    disabled: bool,
    created_at: String,
    updated_at: String,
}

impl Repo {
    fn new(user: User) -> Self {
        Repo {
            user,
            name: String::from(""),
            private: false,
            id: 0,
            fork: false,
            full_name: String::from(""),
            language: String::from(""),
            stargazers_count: 0,
            disabled: false,
            created_at: String::from(""),
            updated_at: String::from(""),
        }
    }

    /// Get user's repostories
    async fn get_repos(&self) -> Result<Vec<Repo>, reqwest::Error> {
        let url = format!("users/{}/repos", self.user.username);
        let repos_match = Client::new().get::<Vec<Repo>>(&url).await;
        match repos_match {
            Ok(repos) => Ok(repos),
            Err(e) => Err(e),
        }
    }
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
                Ok(repo) => {
                    println!("{:?}", repo);
                }
                Err(e) => {
                    println!("Error fetching reps: {}", e);
                }
            }
        }
        Err(..) => {
            //println!("Error: {}", e)
            println!("Failed to get user");
        }
    }
}
