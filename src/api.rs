use crate::client::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    // #[serde(rename(deserialize = "login", serialize = "login"))]
    // pub username: String,
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
    // print_user_profile
    // get_users
}

pub async fn get_user(name: &str) {
    let user = Client::new().get::<User>(name).await;

    match user {
        Ok(success) => println!("{:?}", success),
        Err(e) => println!("Error: {}", e),
    }
}
