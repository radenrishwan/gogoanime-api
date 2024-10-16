pub mod model;

use std::error::Error;

use scraper::{Html, Selector};

use crate::error_struct::ScrapeError;

use self::model::Episode;

pub async fn get(episode_slug: String) -> Result<Episode, Box<dyn Error>> {
    let url = format!("https://ww8.gogoanimes.org/watch/{}", episode_slug);

    let resp = reqwest::get(url).await?;
    if resp.status().is_client_error() {
        return Err(Box::new(ScrapeError::new(
            "Failed to get response".to_string(),
        )));
    }

    let document = Html::parse_document(resp.text().await.unwrap().as_str());

    let iframe_urls = document
        .select(&Selector::parse("#load_anime > div > div > iframe").unwrap())
        .next()
        .unwrap()
        .value()
        .attr("src")
        .unwrap();

    println!("urls : {}", iframe_urls);

    let mut urls = vec![];
    urls.push(iframe_urls.to_string());

    if iframe_urls.contains("s3taku.com") {
        let resp = reqwest::get(iframe_urls).await?;
        if resp.status().is_client_error() {
            return Err(Box::new(ScrapeError::new(
                "Failed to get response".to_string(),
            )));
        }

        let document = Html::parse_document(resp.text().await.unwrap().as_str());
        document
            .select(&Selector::parse("#list-server-more > ul > li").unwrap())
            .for_each(|x| {
                if x.value().attr("data-video").unwrap() != "" {
                    urls.push(x.attr("data-video").unwrap().to_string());
                }
            });
    }

    Ok(Episode::new(urls))
}
