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
            clap::Command::new("search")
                .about("fuzzy search for manga names")
                .arg(
                    clap::Arg::new("search")
                        .help("the search query")
                        .required(true),
                ),
        )
        .subcommand(
            clap::Command::new("chapters")
                .about("give manga hid")
                .arg(
                    clap::Arg::new("chapters")
                        .help("manga name to get info about")
                        .required(true),
                ),
        )
        .subcommand(
            clap::Command::new("download")
                .about("give chapter hid")
                .arg(
                    clap::Arg::new("download")
                        .help("the download query")
                        .required(true),
                ),
        )
        .subcommand(
            clap::Command::new("download_all")
                .about("give chapter hid")
                .arg(
                    clap::Arg::new("download_all")
                        .help("the download_all query to download all chapters of a manga")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("search", sub_matches)) => {
            let query = sub_matches.get_one::<String>("search").expect("Required");
            println!("Searching for: {}", query);

            apis::search_mangas(&query)
                .await
                .expect("error for search_mangas function\n");
        }
        Some(("chapters", sub_matches)) => {
            let chapters = sub_matches.get_one::<String>("chapters").expect("Required");

            apis::chapters(&chapters)
                .await
                .expect("chapters method in main\n");
        }
        Some(("download", sub_matches)) => {
            let download = sub_matches.get_one::<String>("download").expect("Required");

            apis::download_and_process_images(&download)
                .await
                .expect("download_manga method in main.rs\n");
        }
        Some(("download_all", sub_matches)) => {
            let download = sub_matches.get_one::<String>("download_all").expect("Required");

            apis::download_all_chapters(&download)
                .await
                .expect("download_all_manga method in main.rs\n");
        }
        _ => println!("Please use a valid subcommand. Use --help for more information."),
    }

    return Ok(());
}
