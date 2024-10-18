use std::error;

use scraper::{Html, Selector};

use crate::error_struct::ScrapeError;

use self::model::AnimeList;

pub mod model;

pub async fn get(base_url: &str, page: u32) -> Result<Vec<AnimeList>, Box<dyn error::Error>> {
    let url = format!("{}/anime-list?page={}", base_url, page);
    let resp = reqwest::get(&url).await?;
    if resp.status().is_client_error() {
        return Err(Box::new(ScrapeError::new(
            "Failed to get response".to_string(),
        )));
    }

    let document = Html::parse_document(resp.text().await?.as_str());
    let mut anime_list = vec![];
    document
        .select(&Selector::parse(".content_left > div > .anime_list_body > ul > li > a").unwrap())
        .for_each(|x| {
            let title = x.text().collect::<String>();
            let url = x.value().attr("href").unwrap();

            let detail_slug = url.split("/").collect::<Vec<&str>>()[2].to_string();

            anime_list.push(AnimeList::new(
                title.trim().to_string(),
                url.to_string(),
                detail_slug,
            ));
        });

    Ok(anime_list)
}
