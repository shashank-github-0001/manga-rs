#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Manga {
    pub id: i32,
    pub hid: String,
    pub title: String,
    pub desc: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Chapter {
    pub id: i32,
    pub chap: String,
    pub title: Option<String>,
    pub volume: Option<String>,
    pub hid: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ChaptersResponse {
    pub chapters: Vec<Chapter>,
}
