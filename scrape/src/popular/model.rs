use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Popular {
    title: String,
    img: String,
    url: String,
    release_date: u32,
}

impl Popular {
    pub fn new(title: String, img: String, url: String, release_date: u32) -> Self {
        Self {
            title,
            img,
            url,
            release_date,
        }
    }
}
