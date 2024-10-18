use std::error::Error;

use scraper::{Html, Selector};

use crate::error_struct::ScrapeError;

use self::model::Genre;

pub mod model;

pub async fn get(base_url: &str) -> Result<Vec<Genre>, Box<dyn Error>> {
    let resp = reqwest::get(base_url).await?;

    if resp.status().is_client_error() {
        return Err(Box::new(ScrapeError::new(
            "Failed to get response".to_string(),
        )));
    }

    let document = Html::parse_document(resp.text().await?.as_str());

    let genre = document
        .select(
            &Selector::parse(".content_right > div > div > div.recent > nav > ul > li > a")
                .unwrap(),
        )
        .map(|x| {
            Genre::new(
                x.text().collect::<String>(),
                x.value().attr("href").unwrap().to_string(),
            )
        })
        .collect::<Vec<Genre>>();

    Ok(genre)
}
