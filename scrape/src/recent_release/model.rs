use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RecentRelease {
    episode_slug: String,
    title: String,
    episode: u32,
    url: String,
    img: String,
}

impl RecentRelease {
    pub fn new(
        title: String,
        url: String,
        img: String,
        episode_slug: String,
        episode: u32,
    ) -> Self {
        Self {
            title,
            url,
            img,
            episode_slug,
            episode,
        }
    }
}
