use chrono::{DateTime, Datelike, Utc};
use clap::{App, AppSettings, Arg};
use dialoguer::{theme::ColorfulTheme, Password};
use dirs::home_dir;
use num_format::{Locale, ToFormattedString};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;

mod api;
mod auth;
mod client;
mod log;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    access_token: String,
}

#[tokio::main]
async fn main() {
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
            App::new("repo").about("Starts about users repo").arg(
                Arg::new("username")
                    .long("username")
                    .takes_value(true)
                    .required(true),
            ),
        )
        .subcommand(App::new("login").about("Connect gts with your gh account"))
        .get_matches();

    let config: Option<Config> = get_env();

    let mut api_builder = client::Client::default();

    if let Some(app_config) = config {
        api_builder.add_auth(app_config.access_token);
    }

    let api_client = api_builder.build().unwrap();

    if let Some(matches) = matches.subcommand_matches("user") {
        if matches.is_present("username") {
            let username = format!("/{}", matches.value_of("username").unwrap());

            let user_holder = api::UserHolder {
                client: api_client,
                username: String::from(username),
            };

            let user_req = user_holder.get_user().await;

            if let Ok(user) = user_req {
                let years_joined = DateTime::parse_from_rfc3339(&user.created_at)
                    .unwrap()
                    .year();

                let current_year = Utc::now().year();

                let years_elapsed = current_year - years_joined;

                log::clear_screen();

                // Also print activities
                // https://api.github.com/users/veritem/events/public
                println!("\n");

                if let Some(name) = user.name {
                    println!("\tNames: {}", name);
                }
                println!("\tUsername: {}", user.username);
                println!(
                    "\tFollowers: {}",
                    user.followers.to_formatted_string(&Locale::en)
                );
                println!(
                    "\tFollows: {}",
                    user.following.to_formatted_string(&Locale::en)
                );
                println!(
                    "\tRepositories: {}",
                    user.public_repos.to_formatted_string(&Locale::en)
                );
                println!(
                    "\tGists: {}",
                    user.public_gists.to_formatted_string(&Locale::en)
                );
                if let Some(location) = user.location {
                    println!("\tLocation: {}", location);
                }
                if years_elapsed > 1 {
                    println!("\tJoined: {}years ago", years_elapsed);
                } else if years_elapsed == 1 {
                    println!("\tJoined: a year ago");
                } else {
                    print!("\tJoined: This year");
                }
                println!("\n");
            }
        } else {
            //TODO: handle for not found
            println!("{:^-30}", format!("User not found!"));
        }
    }

    if let Some(matches) = matches.subcommand_matches("pr") {
        if matches.is_present("username") {
            // let username = format!("/{}", matches.value_of("username").unwrap());
            // api::get_repos(&username).await;
        }
    }

    if let Some(..) = matches.subcommand_matches("login") {
        if let Some(..) = get_env() {
            log::success("Already logged in!");
            return;
        }

        let access_token = Password::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter your github api token")
            .interact()
            .unwrap();

        let config = Config { access_token };
        set_env(&config);
    }
}

fn get_env() -> Option<Config> {
    let home = home_dir().unwrap();

    // TODO:
    // Fix and use this package
    // https://crates.io/crates/directories

    let path = format!("{}/.gts.yml", home.into_os_string().into_string().unwrap());

    let config_match = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .ok();

    let configs: Result<Config, _> = serde_yaml::from_reader(config_match.unwrap());

    if let Ok(config) = configs {
        return Some(config);
    }
    None
}

fn set_env(config: &Config) {
    let home = home_dir().unwrap();
    let path = format!("{}/.gts.yml", home.into_os_string().into_string().unwrap());
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();

    let env_setting_result = file.write_all(serde_yaml::to_string(&config).unwrap().as_bytes());
    match env_setting_result {
        Ok(_) => log::success("logged in successfully"),
        Err(_) => log::error("Error writing"),
    }
}

#[cfg(test)]
mod tests {
    use super::{get_env, set_env, Config};

    #[test]
    fn setgetenv() {
        let config = Config {
            access_token: String::from("123212"),
        };
        set_env(&config);

        let got_config: Config = get_env().unwrap();
        assert_eq!(got_config.access_token, config.access_token);
    }

    #[test]
    fn test_home() {
        assert_eq!(2, 2)
    }
}
