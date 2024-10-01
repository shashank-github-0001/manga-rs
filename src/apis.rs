// use crate::headers;
// use crate::models;
// use std::io::Write;
//
// #[derive(Debug, thiserror::Error)]
// pub enum Box<dyn std::error::Error> {
//     #[error("IO error: {0}")]
//     IoError(#[from] std::io::Error),
// }
//
// pub async fn search_mangas(search_query: &String) -> Result<(), Box<dyn std::error::Error>> {
//     let url = format!(
//         "https://api.comick.fun/v1.0/search?q={}&tachiyomi=true",
//         search_query
//     );
//
//     let blob = make_api_request(&url).await?;
//     let vec_mangas: Vec<models::Manga> =
//         serde_json::from_str(&blob).expect("not able to convert from json to vec<manga>");
//
//     let mut table = prettytable::Table::new();
//     table.add_row(prettytable::row!["HID", "Title"]);
//
//     for manga in vec_mangas {
//         table.add_row(prettytable::row![manga.hid, manga.title.unwrap()]);
//     }
//
//     let table_string = table.to_string();
//
//     let mut less = std::process::Command::new("bat")
//         .stdin(std::process::Stdio::piped())
//         .spawn()?;
//
//     if let Some(mut stdin) = less.stdin.take() {
//         stdin.write_all(table_string.as_bytes())?;
//     }
//
//     less.wait()?;
//
//     Ok(())
// }
//
// async fn make_api_request(url: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let client = reqwest::Client::new();
//     let headers = headers::headers().await;
//     let res = client.get(url).headers(headers).send().await?;
//
//     match res.status() {
//         reqwest::StatusCode::OK => {
//             return Ok(res.text().await?);
//         }
//         _ => {
//             return Err("the statuscode was not ok".into());
//         }
//     };
// }
//
// pub async fn chapters(info_query: &String) -> Result<(), Box<dyn std::error::Error>> {
//     let mut table = prettytable::Table::new();
//     let url = format!(
//         "https://api.comick.fun/comic/{}/chapters?lang=en&limit=99999&tachiyomi=true",
//         info_query
//     );
//
//     let blob = make_api_request(&url).await?;
//
//     let vec_chaps: models::ChaptersResponse =
//         serde_json::from_str(&blob).expect("conv fail from json to chap res");
//     table.add_row(prettytable::row!["ID", "Chapter", "Title", "Volume", "HID"]);
//
//     for chapter in &vec_chaps.chapters {
//         table.add_row(prettytable::row![
//             chapter.id,
//             chapter.chap.to_owned().unwrap_or("N/A".to_string()),
//             chapter.title.as_deref().unwrap_or("N/A"),
//             chapter.volume.as_deref().unwrap_or("N/A"),
//             chapter.hid
//         ]);
//     }
//
//     let table_string = table.to_string();
//
//     let mut less = std::process::Command::new("bat")
//         .stdin(std::process::Stdio::piped())
//         .spawn()?;
//
//     if let Some(mut stdin) = less.stdin.take() {
//         stdin.write_all(table_string.as_bytes())?;
//     }
//
//     less.wait()?;
//
//     return Ok(());
// }
//
// // #[allow(dead_code)]
// // pub async fn download_manga(download_query: &String) -> Result<(), Box<dyn std::error::Error>> {
// //     let url = format!(
// //         "https://api.comick.fun/chapter/{}/get_images?tachiyomi=true",
// //         download_query
// //     );
// //
// //     let client = reqwest::Client::new();
// //     let headers = headers::headers().await;
// //     let res = client.get(&url).headers(headers.clone()).send().await?;
// //
// //     let blob = match res.status() {
// //         reqwest::StatusCode::OK => res.text().await?,
// //         _ => panic!("did not get any chapters"),
// //     };
// //
// //     let vec_imgs: Vec<models::Images> =
// //         serde_json::from_str(&blob).expect("conv fail from json to chap res");
// //
// //     for (_i, imgs) in vec_imgs.iter().enumerate() {
// //         let url = format!("https://meo3.comick.pictures/{}", imgs.b2key);
// //
// //         let res = client.get(&url).headers(headers.clone()).send().await?;
// //         let bytes = res.bytes().await.expect("dbg error");
// //
// //         let filename = format!("{}", imgs.b2key); // Use filename based on b2key
// //         let mut file = std::fs::File::create(filename).expect("file fail");
// //         file.write_all(&bytes).expect("write failed");
// //         file.flush().expect("flush fail");
// //
// //         // NOTE: someone help me in converting the jpg and png files that are created into one pdf
// //         // file please
// //     }
// //
// //     if !std::process::Command::new("magick")
// //         .args(["*.jpg", "*.png", "output.pdf"])
// //         .status()
// //         .expect("fail")
// //         .success()
// //     {
// //         eprintln!("process failed image");
// //     }
// //
// //     if !std::process::Command::new("rm")
// //         .args(["-rf", "*.jpg", "*.png"])
// //         .status()
// //         .expect("fail")
// //         .success()
// //     {
// //         eprintln!("process failed delete");
// //     }
// //
// //     return Ok(());
// // }
//
// use std::fs;
// use std::process::Command;
//
// async fn fetch_image(url: &str) -> Result<Vec<u8>, std::io::Error> {
//     let client = reqwest::Client::builder()
//         .build()
//         .expect("not able to get client");
//     let header = headers::headers().await;
//     let res = client
//         .get(url)
//         .headers(header)
//         .send()
//         .await
//         .expect("not able to do get request");
//     return Ok(res
//         .bytes()
//         .await
//         .expect("not able to get bytes form")
//         .to_vec());
// }
//
// fn convert_to_pdf(image_paths: &[String], name: &String) -> Result<(), Box<dyn std::error::Error>> {
//     let mut cmd = Command::new("magick");
//     cmd.args(image_paths.iter().cloned())
//         .args([format!("{name}.pdf")]);
//     cmd.status().expect("Failed to convert images to PDF");
//     Ok(())
// }
//
// fn delete_temporary_files(paths: &[String]) -> Result<(), Box<dyn std::error::Error>> {
//     for path in paths {
//         fs::remove_file(path).expect("removing file failed");
//     }
//     Ok(())
// }
//
// pub async fn download_and_process_images(
//     download_query: &String,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let url = format!(
//         "https://api.comick.fun/chapter/{}/get_images?tachiyomi=true",
//         download_query
//     );
//
//     let blob = make_api_request(&url).await?;
//
//     let vec_imgs: Vec<models::Images> =
//         serde_json::from_str(&blob).expect("conv fail from json to chap res");
//
//     let mut image_paths: Vec<String> = Vec::new();
//     let mut tasks: Vec<tokio::task::JoinHandle<Result<String, Box<dyn std::error::Error>>>> = Vec::new();
//
//     for imgs in vec_imgs.iter() {
//         let url = format!("https://meo3.comick.pictures/{}", &imgs.b2key);
//         let path = format!("tmp_{}", &imgs.b2key.clone());
//         let path_clone = path.clone();
//
//         tasks.push(tokio::task::spawn(async move {
//             let bytes = fetch_image(&url).await.expect("failed to fetch image");
//             let mut file = fs::File::create(&path)?;
//             file.write_all(&bytes)?;
//             file.flush()?;
//             Ok::<_, Box<dyn std::error::Error>>(path)
//         }));
//         image_paths.push(path_clone.clone());
//     }
//
//     let mut downloaded_images: Vec<String> = Vec::new();
//     for task in tasks {
//         let result = tokio::join!(task);
//         downloaded_images.push(result.0.unwrap().unwrap())
//     }
//
//     convert_to_pdf(&downloaded_images, download_query)?;
//     delete_temporary_files(&downloaded_images)?;
//
//     Ok(())
// }
//
// pub async fn download_all_chapters(info_query: &String) -> Result<(), Box<dyn std::error::Error>> {
//     let url = format!(
//         "https://api.comick.fun/comic/{}/chapters?lang=en&limit=99999&tachiyomi=true",
//         info_query
//     );
//     let client = reqwest::Client::new();
//     let headers = headers::headers().await;
//     let res = client.get(&url).headers(headers).send().await?;
//
//     let blob = match res.status() {
//         reqwest::StatusCode::OK => res.text().await?,
//         _ => panic!("did not get any chapters"),
//     };
//
//     let vec_chaps: models::ChaptersResponse =
//         serde_json::from_str(&blob).expect("conv fail from json to chap res");
//
//     for chapter in &vec_chaps.chapters {
//         download_and_process_images(&chapter.hid).await?;
//     }
//
//     return Ok(());
// }

use std::io::Write;

async fn make_api_request(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let headers = crate::headers::headers().await;
    let res = client.get(url).headers(headers).send().await?;

    match res.status() {
        reqwest::StatusCode::OK => Ok(res.text().await?),
        _ => Err("status code not ok".into()),
    }
}

fn create_table(headers: Vec<&str>) -> prettytable::Table {
    let mut table = prettytable::Table::new();
    table.add_row(prettytable::row![headers.join(" ")]);
    table
}

async fn display_table(table: prettytable::Table) -> Result<(), Box<dyn std::error::Error>> {
    let table_string = table.to_string();
    let mut less = std::process::Command::new("less")
        .stdin(std::process::Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = less.stdin.take() {
        if let Err(e) = stdin.write_all(table_string.as_bytes()) {
            if e.kind() != std::io::ErrorKind::BrokenPipe {
                return Err(Box::new(e));
            }
        }
    }

    less.wait()?;
    Ok(())
}

pub async fn search_mangas(search_query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.comick.fun/v1.0/search?q={}&tachiyomi=true",
        search_query
    );
    let blob = make_api_request(&url).await?;
    let vec_mangas: Vec<crate::models::Manga> = serde_json::from_str(&blob)?;

    let mut table = create_table(vec!["HID", "Title"]);
    for manga in vec_mangas {
        table.add_row(prettytable::row![
            manga.hid,
            manga.title.unwrap_or_default()
        ]);
    }

    display_table(table).await
}

pub async fn chapters(info_query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.comick.fun/comic/{}/chapters?lang=en&limit=99999&tachiyomi=true",
        info_query
    );
    let blob = make_api_request(&url).await?;
    let vec_chaps: crate::models::ChaptersResponse = serde_json::from_str(&blob)?;

    let mut table = create_table(vec!["ID", "Chapter", "Title", "Volume", "HID"]);
    for chapter in &vec_chaps.chapters {
        table.add_row(prettytable::row![
            chapter.id,
            chapter.chap.to_owned().unwrap_or_else(|| "N/A".to_string()),
            chapter.title.as_deref().unwrap_or("N/A"),
            chapter.volume.as_deref().unwrap_or("N/A"),
            chapter.hid
        ]);
    }

    display_table(table).await
}

async fn fetch_image(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let header = crate::headers::headers().await;
    let res = client.get(url).headers(header).send().await?;
    Ok(res.bytes().await?.to_vec())
}

fn convert_to_pdf(image_paths: &[String], name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = std::process::Command::new("magick");
    cmd.args(image_paths).arg(format!("{name}.pdf"));
    match cmd.status() {
        Ok(_) => {
            return Ok(());
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    };
}

fn delete_temporary_files(paths: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    for path in paths {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

async fn process_single_image(
    url: String,
    path: String,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let bytes = fetch_image(&url).await.expect("not able to fetch images");
    let mut file = std::fs::File::create(path.clone())?;
    file.write_all(&bytes)?;
    file.flush()?;
    Ok(path.to_string())
}

pub async fn download_and_process_images(
    download_query: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.comick.fun/chapter/{}/get_images?tachiyomi=true",
        download_query
    );
    let blob = make_api_request(&url).await?;
    let vec_imgs: Vec<crate::models::Images> = serde_json::from_str(&blob)?;

    let mut tasks = Vec::new();
    for imgs in &vec_imgs {
        let url = format!("https://meo3.comick.pictures/{}", &imgs.b2key);
        let path = format!("tmp_{}", &imgs.b2key);
        tasks.push(tokio::spawn(process_single_image(
            url.clone(),
            path.clone(),
        )));
    }

    let mut downloaded_images = Vec::new();
    for task in tasks {
        downloaded_images.push(task.await?.expect("error shit"));
    }

    convert_to_pdf(&downloaded_images, download_query)?;
    delete_temporary_files(&downloaded_images)?;

    Ok(())
}

pub async fn download_all_chapters(info_query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.comick.fun/comic/{}/chapters?lang=en&limit=99999&tachiyomi=true",
        info_query
    );
    let blob = make_api_request(&url).await?;
    let vec_chaps: crate::models::ChaptersResponse = serde_json::from_str(&blob)?;

    for chapter in vec_chaps.chapters.iter().rev() {
        download_and_process_images(&chapter.hid).await?;
    }

    Ok(())
}
