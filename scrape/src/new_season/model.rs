use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NewSeason {
    detail_slug: String,
    title: String,
    img: String,
    url: String,
    release_date: u32,
}

impl NewSeason {
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
