use clap::App;
use octocrab::{params::repos::forks::Sort, Octocrab};

mod api;
mod client;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let _matches = App::new("gts")
        .version("0.0.1")
        .author("Makuza Mugabo Verite")
        .about("PR manager for opensource developers")
        .get_matches();

    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let forks = octocrab
        .repos("rust-lang", "rust")
        .list_forks()
        .sort(Sort::Oldest)
        .page(2u32)
        .per_page(35)
        .send()
        .await?;

    for f in forks {
        println!("fork: {}", f.owner.login);
    }

    Ok(())
}
