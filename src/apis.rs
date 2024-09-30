use crate::headers;
use crate::models;
use std::io::Write;

#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

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

    let mut table = prettytable::Table::new();
    table.add_row(prettytable::row!["HID", "Title"]);

    for manga in vec_mangas {
        table.add_row(prettytable::row![manga.hid, manga.title.unwrap()]);
    }

    let table_string = table.to_string();

    let mut less = std::process::Command::new("less")
        .stdin(std::process::Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = less.stdin.take() {
        stdin.write_all(table_string.as_bytes())?;
    }

    less.wait()?;

    Ok(())
}

#[allow(dead_code)]
pub async fn chapters(info_query: &String) -> Result<(), Box<dyn std::error::Error>> {
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
            chapter.chap.to_owned().unwrap_or("N/A".to_string()),
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

// #[allow(dead_code)]
// pub async fn download_manga(download_query: &String) -> Result<(), Box<dyn std::error::Error>> {
//     let url = format!(
//         "https://api.comick.fun/chapter/{}/get_images?tachiyomi=true",
//         download_query
//     );
//
//     let client = reqwest::Client::new();
//     let headers = headers::headers().await;
//     let res = client.get(&url).headers(headers.clone()).send().await?;
//
//     let blob = match res.status() {
//         reqwest::StatusCode::OK => res.text().await?,
//         _ => panic!("did not get any chapters"),
//     };
//
//     let vec_imgs: Vec<models::Images> =
//         serde_json::from_str(&blob).expect("conv fail from json to chap res");
//
//     for (_i, imgs) in vec_imgs.iter().enumerate() {
//         let url = format!("https://meo3.comick.pictures/{}", imgs.b2key);
//
//         let res = client.get(&url).headers(headers.clone()).send().await?;
//         let bytes = res.bytes().await.expect("dbg error");
//
//         let filename = format!("{}", imgs.b2key); // Use filename based on b2key
//         let mut file = std::fs::File::create(filename).expect("file fail");
//         file.write_all(&bytes).expect("write failed");
//         file.flush().expect("flush fail");
//
//         // NOTE: someone help me in converting the jpg and png files that are created into one pdf
//         // file please
//     }
//
//     if !std::process::Command::new("magick")
//         .args(["*.jpg", "*.png", "output.pdf"])
//         .status()
//         .expect("fail")
//         .success()
//     {
//         eprintln!("process failed image");
//     }
//
//     if !std::process::Command::new("rm")
//         .args(["-rf", "*.jpg", "*.png"])
//         .status()
//         .expect("fail")
//         .success()
//     {
//         eprintln!("process failed delete");
//     }
//
//     return Ok(());
// }

use std::fs;
use std::process::Command;

async fn fetch_image(url: &str) -> Result<Vec<u8>, std::io::Error> {
    let client = reqwest::Client::builder()
        .build()
        .expect("not able to get client");
    let header = headers::headers().await;
    let res = client
        .get(url)
        .headers(header)
        .send()
        .await
        .expect("not able to do get request");
    return Ok(res
        .bytes()
        .await
        .expect("not able to get bytes form")
        .to_vec());
}

fn convert_to_pdf(image_paths: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("magick");
    let mut name = String::new();
    print!("give a name to store manga: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut name).expect("give a proper name");
    let name = name.trim();
    cmd.args(image_paths.iter().cloned()).args([format!("{name}.pdf")]);
    cmd.status().expect("Failed to convert images to PDF");
    Ok(())
}

fn delete_temporary_files(paths: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    for path in paths {
        fs::remove_file(path).expect("removing file failed");
    }
    Ok(())
}

pub async fn download_and_process_images(
    download_query: &String,
) -> Result<(), Box<dyn std::error::Error>> {
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

    let mut image_paths: Vec<String> = Vec::new();
    let mut tasks: Vec<tokio::task::JoinHandle<Result<String, MyError>>> = Vec::new();

    for imgs in vec_imgs.iter() {
        let url = format!("https://meo3.comick.pictures/{}", &imgs.b2key);
        let path = format!("tmp_{}", &imgs.b2key.clone());
        let path_clone = path.clone();

        tasks.push(tokio::task::spawn(async move {
            let bytes = fetch_image(&url).await.expect("failed to fetch image");
            let mut file = fs::File::create(&path)?;
            file.write_all(&bytes)?;
            file.flush()?;
            Ok::<_, MyError>(path)
        }));
        image_paths.push(path_clone.clone());
    }

    let mut downloaded_images: Vec<String> = Vec::new();
    for task in tasks {
        let result = tokio::join!(task);
        downloaded_images.push(result.0.unwrap().unwrap())
    }

    convert_to_pdf(&downloaded_images)?;
    delete_temporary_files(&downloaded_images)?;

    Ok(())
}
