use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use scrape::gogoanime::{Gogoanime, GogoanimeOption};
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://ww8.gogoanimes.org";
const PORT: u16 = 8080;

#[derive(Debug, Serialize)]
struct DefaultResponse<T> {
    status: u16,
    message: &'static str,
    data: T,
}

impl<T> DefaultResponse<T> {
    fn new(status: u16, message: &'static str, data: T) -> Self {
        DefaultResponse {
            status,
            message,
            data,
        }
    }
}

#[derive(Deserialize)]
struct Page {
    page: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    println!("Route: /");
    HttpResponse::Ok().body("OK")
}

#[get("/api/home")]
async fn home(gogoanime: web::Data<Gogoanime>) -> impl Responder {
    println!("Route: /api/home");

    match gogoanime.home().await {
        Ok(data) => {
            HttpResponse::Ok().json(DefaultResponse::new(200, "Success getting data", data))
        }
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/details/{slug}")]
async fn details(gogoanime: web::Data<Gogoanime>, req: HttpRequest) -> impl Responder {
    println!("Route: /api/details/{{slug}}");
    let slug = req.match_info().get("slug").unwrap().to_string();

    match gogoanime.detail(slug).await {
        Ok(detail) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", detail)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/episode/{slug}")]
async fn episode(gogoanime: web::Data<Gogoanime>, req: HttpRequest) -> impl Responder {
    println!("Route: /api/episode/{{slug}}");
    let slug = req.match_info().get("slug").unwrap().to_string();

    match gogoanime.episode(slug).await {
        Ok(episode) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", episode)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/recent-release")]
async fn recent_release(gogoanime: web::Data<Gogoanime>, page: web::Query<Page>) -> impl Responder {
    println!("Route: /api/recent-release");
    match gogoanime
        .recent_release(page.page.parse::<u32>().unwrap_or(1))
        .await
    {
        Ok(releases) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", releases)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/popular-ongoing")]
async fn popular_ongoing(
    gogoanime: web::Data<Gogoanime>,
    page: web::Query<Page>,
) -> impl Responder {
    println!("Route: /api/popular-ongoing");
    match gogoanime
        .popular_ongoing(page.page.parse::<u32>().unwrap_or(1))
        .await
    {
        Ok(popular) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", popular)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/new-release")]
async fn new_release(gogoanime: web::Data<Gogoanime>) -> impl Responder {
    println!("Route: /api/new-release");
    match gogoanime.new_season(1).await {
        Ok(new_release) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", new_release)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/anime-list")]
async fn anime_list(gogoanime: web::Data<Gogoanime>, page: web::Query<Page>) -> impl Responder {
    println!("Route: /api/anime-list");

    match gogoanime
        .anime_list(page.page.parse::<u32>().unwrap_or(1))
        .await
    {
        Ok(anime_list) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", anime_list)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/popular")]
async fn popular_list(gogoanime: web::Data<Gogoanime>, page: web::Query<Page>) -> impl Responder {
    println!("Route: /api/popular");
    match gogoanime
        .popular(page.page.parse::<u32>().unwrap_or(1))
        .await
    {
        Ok(popular_list) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", popular_list)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[derive(Deserialize)]
struct Search {
    query: String,
}

#[get("/api/search")]
async fn search(gogoanime: web::Data<Gogoanime>, search: web::Query<Search>) -> impl Responder {
    println!("Route: /api/search");
    match gogoanime.search(search.query.to_string()).await {
        Ok(search) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", search)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/genres")]
async fn genres(gogoanime: web::Data<Gogoanime>) -> impl Responder {
    println!("Route: /api/genres");
    match gogoanime.genre().await {
        Ok(genres) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", genres)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/genre/{genre}")]
async fn genre(
    gogoanime: web::Data<Gogoanime>,
    query: web::Query<Page>,
    req: HttpRequest,
) -> impl Responder {
    println!("Route: /api/genre/{{genre}}");
    let genre = req.match_info().get("genre").unwrap().to_string();

    match gogoanime
        .genre_list(genre, query.page.parse::<u32>().unwrap_or(1))
        .await
    {
        Ok(genre_list) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", genre_list)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

pub async fn run() -> std::io::Result<()> {
    let client =
        redis::Client::open("rediss://default:AZPlAAIjcDFlOTAyZGRlN2VmZDc0ZjUzOWI0ZDE1NGQwY2QxZGVmNHAxMA@sweet-seagull-37861.upstash.io:6379").unwrap();

    let option = GogoanimeOption::new(BASE_URL.to_string(), true, 3600 * 12, Some(client));

    let gogoanime = web::Data::new(Gogoanime::new_with_option(option));

    HttpServer::new(move || {
        App::new()
            .app_data(gogoanime.clone())
            .service(hello)
            .service(home)
            .service(details)
            .service(episode)
            .service(recent_release)
            .service(popular_ongoing)
            .service(new_release)
            .service(anime_list)
            .service(popular_list)
            .service(search)
            .service(genres)
            .service(genre)
    })
    // when you running the project on local, you need to change the ip to 127.0.0.1
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
