use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AnimeList {
    title: String,
    url: String,
}

impl AnimeList {
    pub fn new(title: String, url: String) -> Self {
        Self { title, url }
    }
}
