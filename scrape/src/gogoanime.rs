use std::{error, sync::Arc};

use crate::*;

#[derive(Debug, Clone)]
pub struct GogoanimeOption {
    pub base_url: String,
    pub cache: bool,
    pub cache_duration: u64,
    pub redis_client: redis::Client,
}

impl GogoanimeOption {
    pub fn new(
        base_url: String,
        mut cache: bool,
        mut cache_duration: u64,
        mut redis_client: Option<redis::Client>,
    ) -> Self {
        if cache {
            if redis_client.is_none() {
                panic!("Redis client is required when cache is enabled");
            }
        }

        GogoanimeOption {
            base_url,
            cache,
            cache_duration,
            redis_client: redis_client.unwrap(),
        }
    }
}

pub fn default_option(base_url: String) -> GogoanimeOption {
    GogoanimeOption::new(base_url, false, 0, None)
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
        anime_list::get(page).await
    }

    pub async fn detail(
        &self,
        slug: String,
    ) -> Result<detail::model::Detail, Box<dyn error::Error>> {
        detail::get(slug).await
    }

    pub async fn episode(
        &self,
        slug: String,
    ) -> Result<episode::model::Episode, Box<dyn error::Error>> {
        episode::get(slug).await
    }

    pub async fn genre_list(
        &self,
        genre: String,
        page: u32,
    ) -> Result<Vec<genre_list::model::GenreList>, Box<dyn error::Error>> {
        genre_list::get(genre, page).await
    }

    pub async fn genre(&self) -> Result<Vec<genre::model::Genre>, Box<dyn error::Error>> {
        genre::get().await
    }

    pub async fn home(&self) -> Result<home::model::Home, Box<dyn error::Error>> {
        home::get().await
    }

    pub async fn new_season(
        &self,
        page: u32,
    ) -> Result<Vec<new_season::model::NewSeason>, Box<dyn error::Error>> {
        new_season::get(page).await
    }

    pub async fn popular(
        &self,
        page: u32,
    ) -> Result<Vec<popular::model::Popular>, Box<dyn error::Error>> {
        popular::get(page).await
    }

    pub async fn popular_ongoing(
        &self,
        page: u32,
    ) -> Result<Vec<popular_ogoing::model::PopularOngoing>, Box<dyn error::Error>> {
        popular_ogoing::get(page).await
    }

    pub async fn recent_release(
        &self,
        page: u32,
    ) -> Result<Vec<recent_release::model::RecentRelease>, Box<dyn error::Error>> {
        recent_release::get(page).await
    }

    pub async fn search(
        &self,
        keyword: String,
    ) -> Result<Vec<search::model::Search>, Box<dyn error::Error>> {
        search::get(keyword).await
    }
}
