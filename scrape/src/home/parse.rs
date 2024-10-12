use crate::home::model::{RecentRelease, RecentlyAddedSeries};
use scraper::{ElementRef, Selector};

pub fn parse_list(element: ElementRef) -> RecentRelease {
    let title = element
        .select(&Selector::parse(".name > a").unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<String>();
    let img = element
        .select(&Selector::parse(".img > a > img").unwrap())
        .next()
        .unwrap()
        .value()
        .attr("src")
        .unwrap();
    let eps = element
        .select(&Selector::parse(".episode").unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<String>();
    let url = element
        .select(&Selector::parse(".img > a").unwrap())
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap();

    let episode_slug = url.split("/").collect::<Vec<&str>>()[2];

    RecentRelease::new(
        title,
        img.to_string(),
        eps,
        url.to_string(),
        episode_slug.to_string(),
    )
}

pub fn parse_recently_added_series(element: ElementRef) -> RecentlyAddedSeries {
    let title = element.value().attr("title").unwrap();
    let url = element.value().attr("href").unwrap();

    let detail_slug = url.split("/").collect::<Vec<&str>>()[2];

    RecentlyAddedSeries::new(url.to_string(), title.to_string(), detail_slug.to_string())
}
