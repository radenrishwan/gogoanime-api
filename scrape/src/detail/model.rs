use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Episode {
    title: i32,
    url: String,
}

impl Episode {
    pub fn new(title: i32, url: String) -> Self {
        Self { title, url }
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
