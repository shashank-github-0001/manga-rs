#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Manga {
    pub id: i32,
    pub hid: String,
    pub title: Option<String>,
    pub desc: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Chapter {
    pub id: i32,
    pub chap: Option<String>,
    pub title: Option<String>,
    pub volume: Option<String>,
    pub hid: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ChaptersResponse {
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Images{
    pub h: u32,
    pub w: u32,
    pub name: Option<String>,
    pub b2key: String,
}
