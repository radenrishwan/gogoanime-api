use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GenreList {
    detail_slug: String,
    title: String,
    img: String,
    url: String,
    release_date: u32,
}

impl GenreList {
    pub fn new(
        title: String,
        img: String,
        url: String,
        release_date: u32,
        detail_slug: String,
    ) -> Self {
        Self {
            title,
            img,
            url,
            release_date,
            detail_slug,
        }
    }
}
