pub mod model;

use std::error::Error;

use scraper::{Html, Selector};

use crate::error_struct::ScrapeError;

use self::model::Popular;

pub async fn get(base_url: &str, page: u32) -> Result<Vec<Popular>, Box<dyn Error>> {
    let url = format!("{}/popular?page={}", base_url, page);

    let resp = reqwest::get(url).await?;
    if resp.status().is_client_error() {
        return Err(Box::new(ScrapeError::new(
            "Failed to get response".to_string(),
        )));
    }

    let document = Html::parse_document(resp.text().await.unwrap().as_str());
    let mut releases = vec![];
    document
        .select(&Selector::parse(".content_left > div > .last_episodes > ul > li").unwrap())
        .for_each(|f| {
            let url = f
                .select(&Selector::parse(".img > a").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap();

            let mut title = "";
            let mut img = "";
            f.select(&Selector::parse("div > a > img").unwrap())
                .for_each(|x| {
                    img = x.value().attr("src").unwrap();
                    title = x.value().attr("alt").unwrap();
                });

            let released = f
                .select(&Selector::parse("p.episode").unwrap())
                .next()
                .unwrap()
                .text()
                .collect::<String>()
                .to_lowercase()
                .split(":")
                .collect::<Vec<&str>>()[1]
                .trim()
                .parse::<u32>()
                .unwrap();

            let detail_slug = url.split("/").collect::<Vec<&str>>()[2].to_string();

            releases.push(Popular::new(
                title.to_string(),
                url.to_string(),
                img.to_string(),
                released,
                detail_slug,
            ));
        });

    Ok(releases)
}
