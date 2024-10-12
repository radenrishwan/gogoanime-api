use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PopularOngoing {
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
    ) -> Self {
        PopularOngoing {
            title,
            img,
            episode,
            url,
            genre,
        }
    }
}
