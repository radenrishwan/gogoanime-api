use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RecentRelease {
    title: String,
    url: String,
    img: String,
}

impl RecentRelease {
    pub fn new(title: String, url: String, img: String) -> Self {
        Self { title, url, img }
    }
}
