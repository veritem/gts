use clap::{App, AppSettings, Arg};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;

mod api;
mod auth;
mod client;

//TODO:
// - login command
// - authenticated stastics
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    access_token: String,
}

#[tokio::main]
async fn main() {
    let home = home_dir().unwrap();

    let path = format!("{}/.gts.yml", home.into_os_string().into_string().unwrap());

    let config_match = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .ok();

    let configs: Result<Config, _> = serde_yaml::from_reader(config_match.unwrap());

    if let Ok(config) = configs {
        println!("{:?}", config);
    }

    let matches = App::new("gts")
        .version("0.0.1")
        .about("Clean Github user stats")
        .author("Verite <mugaboverite@gmail.com>")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            App::new("user")
                .about("Get information about the user")
                .arg(
                    Arg::new("username")
                        .short('u')
                        .long("username")
                        .takes_value(true)
                        .about("prints user information")
                        .required(true),
                ),
        )
        .subcommand(
            App::new("pr").about("Starts about PR").arg(
                Arg::new("username")
                    .long("username")
                    .takes_value(true)
                    .required(true),
            ),
        )
        .subcommand(
            App::new("login")
                .about("Connect gts with your gh account")
                .arg(Arg::new("username").takes_value(true).required(true)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("user") {
        if matches.is_present("username") {
            let username = format!("/{}", matches.value_of("username").unwrap());
            api::get_user(&username).await;
        }
    }

    if let Some(matches) = matches.subcommand_matches("pr") {
        if matches.is_present("username") {
            let username = format!("/{}", matches.value_of("username").unwrap());
            api::get_repos(&username).await;
        }
    }
}
