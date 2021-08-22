use clap::App;

fn main() {
    let _matches = App::new("gts")
        .version("0.0.1")
        .author("Makuza Mugabo Verite")
        .about("PR manager for opensource developers")
        .get_matches();
}
