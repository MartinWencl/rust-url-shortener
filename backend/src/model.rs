use serde;

#[derive(serde::Deserialize)]
pub struct GetUrls {
    pub from_url: String,
}

#[derive(serde::Serialize)]
pub struct PostUrls {
    pub from_url: String,
    pub to_url: String,
}
