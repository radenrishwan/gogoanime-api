use std::error::Error;

use scraper::{Html, Selector};

use crate::error_struct::ScrapeError;

use self::model::RecentRelease;

mod model;

pub async fn get(page: u32) -> Result<Vec<RecentRelease>, Box<dyn Error>> {
    let url = format!(
        "https://ww8.gogoanimes.org/ajax/page-recent-release?page={}",
        page
    );

    let resp = reqwest::get(&url).await?;

    if resp.status().is_client_error() {
        return Err(Box::new(ScrapeError::new(
            "Failed to get response".to_string(),
        )));
    }

    let document = Html::parse_document(resp.text().await?.as_str());
    let mut releases = vec![];
    document
        .select(&Selector::parse("div.last_episodes.loaddub > ul > li > div > a").unwrap())
        .for_each(|x| {
            let href = x.value().attr("href").unwrap();

            releases.push(RecentRelease::new(
                x.value().attr("title").unwrap().to_string(),
                href.to_string(),
                x.select(&Selector::parse("img").unwrap())
                    .next()
                    .unwrap()
                    .value()
                    .attr("src")
                    .unwrap()
                    .to_string(),
                href.split("/").collect::<Vec<&str>>()[2].to_string(),
            ));
        });

    Ok(releases)
}
