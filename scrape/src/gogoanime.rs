use std::{error, sync::Arc};

use crate::*;

#[derive(Debug, Clone)]
pub struct GogoanimeOption {
    pub base_url: String,
}

impl GogoanimeOption {
    pub fn new(base_url: String) -> Self {
        GogoanimeOption { base_url }
    }
}

pub fn default_option(base_url: String) -> GogoanimeOption {
    GogoanimeOption::new(base_url)
}

#[derive(Debug, Clone)]
pub struct Gogoanime {
    option: Arc<GogoanimeOption>,
}

impl Gogoanime {
    pub fn new(base_url: String) -> Self {
        Gogoanime {
            option: Arc::new(default_option(base_url)),
        }
    }

    pub fn new_with_option(option: GogoanimeOption) -> Self {
        Gogoanime {
            option: Arc::new(option),
        }
    }

    pub async fn anime_list(
        &self,
        page: u32,
    ) -> Result<Vec<anime_list::model::AnimeList>, Box<dyn error::Error>> {
        anime_list::get(self.option.base_url.as_str(), page).await
    }

    pub async fn detail(
        &self,
        slug: String,
    ) -> Result<detail::model::Detail, Box<dyn error::Error>> {
        detail::get(self.option.base_url.as_str(), slug).await
    }

    pub async fn episode(
        &self,
        slug: String,
    ) -> Result<episode::model::Episode, Box<dyn error::Error>> {
        episode::get(self.option.base_url.as_str(), slug).await
    }

    pub async fn genre_list(
        &self,
        genre: String,
        page: u32,
    ) -> Result<Vec<genre_list::model::GenreList>, Box<dyn error::Error>> {
        genre_list::get(self.option.base_url.as_str(), genre, page).await
    }

    pub async fn genre(&self) -> Result<Vec<genre::model::Genre>, Box<dyn error::Error>> {
        genre::get(self.option.base_url.as_str()).await
    }

    pub async fn home(&self) -> Result<home::model::Home, Box<dyn error::Error>> {
        home::get(self.option.base_url.as_str()).await
    }

    pub async fn new_season(
        &self,
        page: u32,
    ) -> Result<Vec<new_season::model::NewSeason>, Box<dyn error::Error>> {
        new_season::get(self.option.base_url.as_str(), page).await
    }

    pub async fn popular(
        &self,
        page: u32,
    ) -> Result<Vec<popular::model::Popular>, Box<dyn error::Error>> {
        popular::get(self.option.base_url.as_str(), page).await
    }

    pub async fn popular_ongoing(
        &self,
        page: u32,
    ) -> Result<Vec<popular_ogoing::model::PopularOngoing>, Box<dyn error::Error>> {
        popular_ogoing::get(self.option.base_url.as_str(), page).await
    }

    pub async fn recent_release(
        &self,
        page: u32,
    ) -> Result<Vec<recent_release::model::RecentRelease>, Box<dyn error::Error>> {
        recent_release::get(self.option.base_url.as_str(), page).await
    }

    pub async fn search(
        &self,
        keyword: String,
    ) -> Result<Vec<search::model::Search>, Box<dyn error::Error>> {
        search::get(self.option.base_url.as_str(), keyword).await
    }
}
