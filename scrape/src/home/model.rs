use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RecentRelease {
    title: String,
    img: String,
    eps: String,
    url: String,
}

impl RecentRelease {
    pub fn new(title: String, img: String, eps: String, url: String) -> Self {
        RecentRelease {
            title,
            img,
            eps,
            url,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RecentlyAddedSeries {
    url: String,
    title: String,
}

impl RecentlyAddedSeries {
    pub fn new(url: String, title: String) -> Self {
        RecentlyAddedSeries { url, title }
    }
}

#[derive(Debug, Serialize)]
pub struct PopularOngoingUpdate {
    title: String,
    img: String,
    episode: String,
    url: String,
    genre: Vec<String>,
}

impl PopularOngoingUpdate {
    pub fn new(
        title: String,
        img: String,
        episode: String,
        url: String,
        genre: Vec<String>,
    ) -> Self {
        PopularOngoingUpdate {
            title,
            img,
            episode,
            url,
            genre,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OngoingSeries {
    url: String,
    title: String,
}

impl OngoingSeries {
    pub fn new(url: String, title: String) -> Self {
        OngoingSeries { url, title }
    }
}

#[derive(Debug, Serialize)]
pub struct Home {
    recent_release: Vec<RecentRelease>,
    recently_added_series: Vec<RecentlyAddedSeries>,
    ongoing_series: Vec<OngoingSeries>,
    popular_ongoing_update: Vec<PopularOngoingUpdate>,
}

impl Home {
    pub fn new(
        recent_release: Vec<RecentRelease>,
        recently_added_series: Vec<RecentlyAddedSeries>,
        ongoing_series: Vec<OngoingSeries>,
        popular_ongoing_update: Vec<PopularOngoingUpdate>,
    ) -> Self {
        Home {
            recent_release,
            recently_added_series,
            ongoing_series,
            popular_ongoing_update,
        }
    }
}
