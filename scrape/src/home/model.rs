use serde::Serialize;

use crate::popular_ogoing::model::PopularOngoing;
#[derive(Debug, Serialize)]
pub struct RecentRelease {
    episode_slug: String,
    title: String,
    img: String,
    eps: String,
    url: String,
}

impl RecentRelease {
    pub fn new(title: String, img: String, eps: String, url: String, episode_slug: String) -> Self {
        RecentRelease {
            episode_slug,
            title,
            img,
            eps,
            url,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RecentlyAddedSeries {
    detail_slug: String,
    url: String,
    title: String,
}

impl RecentlyAddedSeries {
    pub fn new(url: String, title: String, detail_slug: String) -> Self {
        RecentlyAddedSeries {
            url,
            title,
            detail_slug,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OngoingSeries {
    detail_slug: String,
    url: String,
    title: String,
}

impl OngoingSeries {
    pub fn new(url: String, title: String, detail_slug: String) -> Self {
        OngoingSeries {
            url,
            title,
            detail_slug,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Home {
    recent_release: Vec<RecentRelease>,
    recently_added_series: Vec<RecentlyAddedSeries>,
    ongoing_series: Vec<OngoingSeries>,
    popular_ongoing_update: Vec<PopularOngoing>,
}

impl Home {
    pub fn new(
        recent_release: Vec<RecentRelease>,
        recently_added_series: Vec<RecentlyAddedSeries>,
        ongoing_series: Vec<OngoingSeries>,
        popular_ongoing_update: Vec<PopularOngoing>,
    ) -> Self {
        Home {
            recent_release,
            recently_added_series,
            ongoing_series,
            popular_ongoing_update,
        }
    }
}
