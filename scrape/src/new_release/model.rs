use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NewRelease {
    title: String,
    img: String,
    url: String,
    release_date: u32,
}

impl NewRelease {
    pub fn new(title: String, img: String, url: String, release_date: u32) -> Self {
        Self {
            title,
            img,
            url,
            release_date,
        }
    }
}
