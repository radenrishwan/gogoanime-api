use std::error::Error;

use scraper::{ElementRef, Html, Selector};

use crate::error_struct::ScrapeError;

use self::model::PopularOngoing;

pub mod model;

pub async fn get(base_url: &str, page: u32) -> Result<Vec<PopularOngoing>, Box<dyn Error>> {
    let url = format!(
        "{}/ajax/page-recent-release-ongoing?page={}",
        base_url, page
    );
    let resp = reqwest::get(url).await?;

    if resp.status().is_client_error() {
        return Err(Box::new(ScrapeError::new(
            "Failed to get response".to_string(),
        )));
    }

    let document = Html::parse_document(resp.text().await.unwrap().as_str());
    let mut popular_list = vec![];
    for element in document.select(&Selector::parse(".added_series_body > ul > li").unwrap()) {
        popular_list.push(parse_popular_ongoing_update(element));
    }

    Ok(popular_list)
}

pub fn parse_popular_ongoing_update(element: ElementRef) -> PopularOngoing {
    let title = element
        .select(&Selector::parse("a").unwrap())
        .next()
        .unwrap()
        .value()
        .attr("title")
        .unwrap();
    let img = element
        .select(&Selector::parse("a > div").unwrap())
        .next()
        .unwrap()
        .value()
        .attr("style")
        .unwrap();
    let eps = element
        .select(&Selector::parse("p > a").unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<String>();
    let url = element
        .select(&Selector::parse("a").unwrap())
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap();

    let mut genres = Vec::new();
    for genre in element.select(&Selector::parse(".genres > a").unwrap()) {
        genres.push(genre.text().collect::<String>());
    }

    // background: url('https://gogocdn.net/images/anime/One-piece.jpg');
    // get the url from the style attribute
    let img = img.split("'").collect::<Vec<&str>>()[1];

    let detail_slug = url.split("/").collect::<Vec<&str>>()[2];

    PopularOngoing::new(
        title.to_string(),
        img.to_string(),
        eps,
        url.to_string(),
        genres,
        detail_slug.to_string(),
    )
}
