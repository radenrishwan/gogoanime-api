use std::error::Error;

use scraper::{Html, Selector};

use crate::error_struct::ScrapeError;

use self::model::RecentRelease;

pub mod model;

pub async fn get(base_url: &str, page: u32) -> Result<Vec<RecentRelease>, Box<dyn Error>> {
    let url = format!("{}/ajax/page-recent-release?page={}", base_url, page);

    let resp = reqwest::get(&url).await?;

    if resp.status().is_client_error() {
        return Err(Box::new(ScrapeError::new(
            "Failed to get response".to_string(),
        )));
    }

    // body > div.last_episodes.loaddub > ul > li:nth-child(4) > p.episode
    let document = Html::parse_document(resp.text().await?.as_str());
    let mut releases = vec![];
    document
        .select(&Selector::parse("div.last_episodes.loaddub > ul > li").unwrap())
        .for_each(|x| {
            // div.last_episodes.loaddub > ul > li > div > a
            let title = x
                .select(&Selector::parse("div > a").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("title")
                .unwrap();

            let href = x
                .select(&Selector::parse("div > a").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap();

            let img = x
                .select(&Selector::parse("div > a > img").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("src")
                .unwrap();

            let eps = x
                .select(&Selector::parse("p.episode").unwrap())
                .next()
                .unwrap()
                .text()
                .collect::<String>();

            releases.push(RecentRelease::new(
                title.to_string(),
                href.to_string(),
                img.to_string(),
                href.split("/").collect::<Vec<&str>>()[2].to_string(),
                eps.split(" ").collect::<Vec<&str>>()[1]
                    .parse::<u32>()
                    .unwrap(),
            ));
        });

    Ok(releases)
}
