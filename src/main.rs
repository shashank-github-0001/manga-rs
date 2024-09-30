mod apis;
mod headers;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::Command::new("manga")
        .version("0.0.1")
        .author("shashank")
        .about("tool to download mangas")
        .subcommand(
            clap::Command::new("manga")
                .about("search for top mangas matching the given args")
                .arg(
                    clap::Arg::new("manga")
                        .help("the search query")
                        .required(true),
                ),
        )
        .subcommand(
            clap::Command::new("chapters")
                .about("get chapters of a manga")
                .arg(
                    clap::Arg::new("chapters")
                        //  NOTE: this may take manga hid instead of just name
                        .help("manga name to get info about")
                        .required(true),
                ),
        )
        .subcommand(
            clap::Command::new("download")
                .about("give the name of the manga to download")
                .arg(
                    clap::Arg::new("download")
                        .help("the download query")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("manga", sub_matches)) => {
            let query = sub_matches.get_one::<String>("manga").expect("Required");
            println!("Searching for: {}", query);

            apis::search_mangas(&query)
                .await
                .expect("error for search_mangas function");
        }
        Some(("chapters", sub_matches)) => {
            let chapters = sub_matches.get_one::<String>("chapters").expect("Required");

            apis::chapters(&chapters)
                .await
                .expect("chapters method in main\n");
        }
        Some(("download", sub_matches)) => {
            let download = sub_matches.get_one::<String>("download").expect("Required");

            apis::download_manga(&download)
                .await
                .expect("download_manga method in main.rs\n");
        }
        _ => println!("Please use a valid subcommand. Use --help for more information."),
    }

    return Ok(());
}
