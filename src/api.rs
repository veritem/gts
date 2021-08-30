use crate::client::*;
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

// impl User {
//     // print_user_profile
//     // get_users
// }

pub async fn get_user(name: &str) {
    let url = format!("users{}", name);
    let user = Client::new().get::<User>(&url).await;

    let sp = Spinner::new(&Spinners::Dots9, "Getting user info".into());
    sleep(Duration::from_secs(3));
    sp.stop();

    println!();

    match user {
        Ok(success) => {
            println!("{:?}", success)
        }
        Err(..) => {
            //println!("Error: {}", e)
            println!("Failed to get user");
        }
    }
}
