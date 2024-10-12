use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PopularOngoing {
    detail_slug: String,
    title: String,
    img: String,
    episode: String,
    url: String,
    genre: Vec<String>,
}

impl PopularOngoing {
    pub fn new(
        title: String,
        img: String,
        episode: String,
        url: String,
        genre: Vec<String>,
        detail_slug: String,
    ) -> Self {
        PopularOngoing {
            title,
            img,
            episode,
            url,
            genre,
            detail_slug,
        }
    }
}
