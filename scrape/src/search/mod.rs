pub mod model;

use std::error::Error;

use scraper::{Html, Selector};

use crate::error_struct::ScrapeError;

use self::model::Search;

pub async fn get(keyword: String) -> Result<Vec<Search>, Box<dyn Error>> {
    let url = format!("https://ww8.gogoanimes.org/search?keyword={}", keyword);

    let resp = reqwest::get(url).await?;
    if resp.status().is_client_error() {
        return Err(Box::new(ScrapeError::new(
            "Failed to get response".to_string(),
        )));
    }

    let document = Html::parse_document(resp.text().await.unwrap().as_str());
    let mut result = vec![];
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

            let search = f
                .select(&Selector::parse("p.released").unwrap())
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

            result.push(Search::new(
                title.to_string(),
                url.to_string(),
                img.to_string(),
                search,
            ));
        });

    Ok(result)
}
