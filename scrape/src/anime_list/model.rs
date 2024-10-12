use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AnimeList {
    detail_slug: String,
    title: String,
    url: String,
}

impl AnimeList {
    pub fn new(title: String, url: String, detail_slug: String) -> Self {
        Self {
            title,
            url,
            detail_slug,
        }
    }
}
