use crate::home::model::{PopularOngoingUpdate, RecentRelease, RecentlyAddedSeries};
use scraper::{ElementRef, Selector};

pub fn parse_popular_ongoing_update(element: ElementRef) -> PopularOngoingUpdate {
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

    PopularOngoingUpdate::new(
        title.to_string(),
        img.to_string(),
        eps,
        url.to_string(),
        genres,
    )
}

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

    RecentRelease::new(title, img.to_string(), eps, url.to_string())
}

pub fn parse_recently_added_series(element: ElementRef) -> RecentlyAddedSeries {
    let title = element.value().attr("title").unwrap();
    let url = element.value().attr("href").unwrap();

    RecentlyAddedSeries::new(url.to_string(), title.to_string())
}
