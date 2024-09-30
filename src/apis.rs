use crate::headers;
use crate::models;
use std::io::Write;

#[allow(dead_code)]
pub async fn search_mangas(search_query: &String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.comick.fun/v1.0/search?q={}&tachiyomi=true",
        search_query
    );
    let client = reqwest::Client::new();
    let headers = headers::headers().await;
    let res = client.get(&url).headers(headers).send().await?;

    let blob = match res.status() {
        reqwest::StatusCode::OK => res.text().await?,
        _ => panic!("did not get any mangas for that name"),
    };

    let vec_mangas: Vec<models::Manga> =
        serde_json::from_str(&blob).expect("not able to convert from json to vec<manga>");

    for manga in vec_mangas {
        println!("hid: {} title: {}", manga.hid, manga.title);
    }

    return Ok(());
}

#[allow(dead_code)]
pub async fn chapters(info_query: &String) -> Result<(), Box<dyn std::error::Error>> {
    //  NOTE: if they want to print all chapters then print so first ask them
    //  if they don't then ask the range and then donwload those chapters only

    let mut table = prettytable::Table::new();
    let url = format!(
        "https://api.comick.fun/comic/{}/chapters?lang=en&limit=99999&tachiyomi=true",
        info_query
    );
    let client = reqwest::Client::new();
    let headers = headers::headers().await;
    let res = client.get(&url).headers(headers).send().await?;

    let blob = match res.status() {
        reqwest::StatusCode::OK => res.text().await?,
        _ => panic!("did not get any chapters"),
    };

    let vec_chaps: models::ChaptersResponse =
        serde_json::from_str(&blob).expect("conv fail from json to chap res");
    table.add_row(prettytable::row!["ID", "Chapter", "Title", "Volume", "HID"]);

    for chapter in &vec_chaps.chapters {
        table.add_row(prettytable::row![
            chapter.id,
            chapter.chap,
            chapter.title.as_deref().unwrap_or("N/A"),
            chapter.volume.as_deref().unwrap_or("N/A"),
            chapter.hid
        ]);
    }

    let table_string = table.to_string();

    let mut less = std::process::Command::new("less")
        .stdin(std::process::Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = less.stdin.take() {
        stdin.write_all(table_string.as_bytes())?;
    }

    less.wait()?;

    return Ok(());
}

#[allow(dead_code)]
pub async fn download_manga(download_query: &String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.comick.fun/chapter/{}/get_images?tachiyomi=true",
        download_query
    );

    let client = reqwest::Client::new();
    let headers = headers::headers().await;
    let res = client.get(&url).headers(headers.clone()).send().await?;

    let blob = match res.status() {
        reqwest::StatusCode::OK => res.text().await?,
        _ => panic!("did not get any chapters"),
    };

    let vec_imgs: Vec<models::Images> =
        serde_json::from_str(&blob).expect("conv fail from json to chap res");

    for (_i, imgs) in vec_imgs.iter().enumerate() {
        let url = format!("https://meo3.comick.pictures/{}", imgs.b2key);
        // println!("{i}, {}", url);

        let res = client.get(&url).headers(headers.clone()).send().await?;
        let bytes = res.bytes().await.expect("dbg error");

        let filename = format!("{}", imgs.b2key); // Use filename based on b2key
        let mut file = std::fs::File::create(filename).expect("file fail");
        file.write_all(&bytes).expect("write failed");
        file.flush().expect("flush fail");

        // NOTE: someone help me in converting the jpg and png files that are created into one pdf
        // file please
    }

    return Ok(());
}
