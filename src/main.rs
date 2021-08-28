// use clap::App;
// use octocrab::{params::repos::forks::Sort, Octocrab};
mod api;
mod auth;
mod client;

#[tokio::main]
async fn main() {
    let test = client::Client::new("Accept: application/json")
        .get_user("/users/veritem")
        .await;

    match test {
        Err(e) => println!("Error: {:?}", e),
        _ => (),
    }
}
