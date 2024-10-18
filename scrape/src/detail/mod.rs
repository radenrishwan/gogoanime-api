use std::{collections::HashMap, error::Error};

use scraper::{Html, Selector};

use crate::error_struct::ScrapeError;

use self::model::{Detail, Episode};

pub mod model;

pub async fn get(base_url: &str, detail_slug: String) -> Result<Detail, Box<dyn Error>> {
    // https://ww8.gogoanimes.org/category/kimetsu-no-yaiba-katanakaji-no-sato-hen
    let url = format!("{}/category/{}", base_url, detail_slug);

    let resp = reqwest::get(url).await?;

    if resp.status().is_client_error() {
        return Err(Box::new(ScrapeError::new(
            "Failed to get response".to_string(),
        )));
    }

    let document = Html::parse_document(resp.text().await.unwrap().as_str());

    let img = document
        .select(&Selector::parse("div.anime_info_body > div.anime_info_body_bg > img").unwrap())
        .next()
        .unwrap()
        .value()
        .attr("src")
        .unwrap();

    let title = document
        .select(&Selector::parse("div.anime_info_body > div.anime_info_body_bg > h1").unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<String>();

    let mut genres = vec![];
    let mut other_names = vec![];
    let mut details_list = HashMap::new();
    for details in document
        .select(&Selector::parse("div.anime_info_body > div.anime_info_body_bg > .type").unwrap())
    {
        let values = parse_text(details.text().collect::<String>());

        if values.0 == "genre" {
            details
                .select(&Selector::parse("a").unwrap())
                .for_each(|f| {
                    genres.push(f.text().collect::<String>());
                });
        } else if values.0 == "other_name" {
            values.1.split(";").for_each(|f| {
                other_names.push(f.trim().to_string());
            });
        } else {
            details_list.insert(values.0, values.1);
        }
    }

    let resp = reqwest::get("https://ww8.gogoanimes.org/ajaxajax/load-list-episode?ep_start=0&ep_end=&id=0&default_ep=&alias=/category/sabu-to-ichi-torimono-hikae").await?;

    if resp.status().is_client_error() {
        return Err(Box::new(ScrapeError::new(
            "Failed to get response".to_string(),
        )));
    }

    let document = Html::parse_document(resp.text().await.unwrap().as_str());
    let mut episodes = vec![];
    document
        .select(&Selector::parse("#episode_related > li > a").unwrap())
        .for_each(|f| {
            let url = f.value().attr("href").unwrap().to_string();
            let episode = f
                .select(&Selector::parse(".name").unwrap())
                .next()
                .unwrap()
                .text()
                .collect::<String>();

            let episode = episode.split(" ").collect::<Vec<&str>>()[1];

            // convert episode to number
            let episode = match episode.parse::<i32>() {
                Ok(v) => v,
                Err(_) => -1,
            };

            let episode_slug = url
                .split("/")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .to_string();

            episodes.push(Episode::new(episode, url, episode_slug));
        });

    Ok(Detail::new(
        title,
        img.to_string(),
        details_list,
        genres,
        other_names,
        episodes,
    ))
}

fn parse_text(text: String) -> (String, String) {
    let parts: Vec<&str> = text
        .split(':')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    return (
        parts[0].to_string().trim().to_lowercase().replace(" ", "_"),
        parts[1].to_string().replace("\n", "").replace("\t", " "),
    );
}
