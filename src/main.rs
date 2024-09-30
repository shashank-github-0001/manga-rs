mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::Command::new("manga")
        .version("0.0.1")
        .author("shashank")
        .about("tool to download mangas")
        .subcommand(
            clap::Command::new("search")
                .about("search for top mangas matching the given args")
                .arg(
                    clap::Arg::new("query")
                        .help("the search query")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            clap::Command::new("info")
                .about("get the info for manga")
                .arg(
                    clap::Arg::new("info")
                        //  NOTE: this may take manga hid instead of just name
                        .help("manga name to get info about")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            clap::Command::new("download")
                .about("give the name of the manga to download")
                .arg(
                    clap::Arg::new("download")
                        .help("the download query")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("search", sub_matches)) => {
            let query = sub_matches.get_one::<String>("search").expect("Required");
            println!("Searching for: {}", query);
        }
        Some(("info", sub_matches)) => {
            let info = sub_matches.get_one::<String>("info").expect("Required");
            println!("Getting info about: {}", item);
        }
        Some(("download", sub_matches)) => {
            let download = sub_matches.get_one::<String>("download").expect("Required");
            println!("Downloading from: {}", url);
        }
        _ => println!("Please use a valid subcommand. Use --help for more information."),
    }

    return Ok(());
}
