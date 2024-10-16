use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Episode {
    title: i32,
    episode_slug: String,
    url: String,
}

impl Episode {
    pub fn new(title: i32, url: String, episode_slug: String) -> Self {
        Self {
            title,
            url,
            episode_slug,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Detail {
    title: String,
    img: String,
    details: HashMap<String, String>,
    genres: Vec<String>,
    other_titles: Vec<String>,
    episodes: Vec<Episode>,
}

impl Detail {
    pub fn new(
        title: String,
        img: String,
        details: HashMap<String, String>,
        genres: Vec<String>,
        other_titles: Vec<String>,
        episodes: Vec<Episode>,
    ) -> Self {
        Self {
            title,
            img,
            details,
            genres,
            other_titles,
            episodes,
        }
    }
}
