use std::error::Error;

use scraper::{Html, Selector};

use crate::{
    error_struct::ScrapeError,
    home::{
        model::{Home, OngoingSeries},
        parse::{parse_list, parse_recently_added_series},
    },
};

pub mod model;
pub mod parse;

pub async fn get() -> Result<Home, Box<dyn Error>> {
    let resp = reqwest::get("https://ww8.gogoanimes.org/").await?;

    if resp.status().is_client_error() {
        return Err(Box::new(ScrapeError::new(
            "Failed to get response".to_string(),
        )));
    }

    let document = Html::parse_document(resp.text().await.unwrap().as_str());
    let mut recent_release = vec![];
    for element in document.select(&Selector::parse(".items > li").unwrap()) {
        recent_release.push(parse_list(element));
    }

    let mut recently_added_series = vec![];
    for element in document.select(&Selector::parse(".added_series_body > ul > li > a").unwrap()) {
        recently_added_series.push(parse_recently_added_series(element));
    }

    let mut ongoing_series_list = vec![];
    for element in document
        .select(&Selector::parse("#scrollbar2 > div.viewport > div > nav > ul > li > a").unwrap())
    {
        ongoing_series_list.push(OngoingSeries::new(
            element.value().attr("href").unwrap().to_string(),
            element.value().attr("title").unwrap().to_string(),
        ));
    }

    let resp =
        reqwest::get("https://ww8.gogoanimes.org/ajax/page-recent-release-ongoing?page=1").await?;

    if resp.status().is_client_error() {
        return Err(Box::new(ScrapeError::new(
            "Failed to get response".to_string(),
        )));
    }

    let popular_list = crate::popular_ogoing::get(1).await.unwrap();

    Ok(Home::new(
        recent_release,
        recently_added_series,
        ongoing_series_list,
        popular_list,
    ))
}
