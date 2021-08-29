use clap::{App, Arg};

mod api;
mod auth;
mod client;

#[tokio::main]
async fn main() {
    let matches = App::new("gts")
        .version("0.0.1")
        .about("Clean Github user stats")
        .author("Verite <mugaboverite@gmail.com>")
        .subcommand(
            App::new("user")
                .about("Get information about the user")
                .arg(
                    Arg::new("username")
                        .short('u')
                        .takes_value(true)
                        .about("prints user information")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("user") {
        if matches.is_present("username") {
            let username = format!("/{}", matches.value_of("username").unwrap());
            api::get_user(&username).await;
        }
    }
}
