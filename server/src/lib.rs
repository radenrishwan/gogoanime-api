use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

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
    HttpResponse::Ok().body("OK")
}

#[get("/api/home")]
async fn echo() -> impl Responder {
    match scrape::home::get().await {
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
async fn details(req: HttpRequest) -> impl Responder {
    let slug = req.match_info().get("slug").unwrap().to_string();

    match scrape::detail::get(slug).await {
        Ok(detail) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", detail)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/episode/{slug}")]
async fn episode(req: HttpRequest) -> impl Responder {
    let slug = req.match_info().get("slug").unwrap().to_string();

    match scrape::episode::get(slug).await {
        Ok(episode) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", episode)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/recent-release")]
async fn recent_release(page: web::Query<Page>) -> impl Responder {
    match scrape::recent_release::get(page.page.parse::<u32>().unwrap_or(1)).await {
        Ok(releases) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", releases)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/popular-ongoing")]
async fn popular_ongoing(page: web::Query<Page>) -> impl Responder {
    match scrape::popular_ogoing::get(page.page.parse::<u32>().unwrap_or(1)).await {
        Ok(popular) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", popular)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/new-release")]
async fn new_release() -> impl Responder {
    match scrape::new_season::get(1).await {
        Ok(new_release) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", new_release)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/anime-list")]
async fn anime_list(page: web::Query<Page>) -> impl Responder {
    match scrape::anime_list::get(page.page.parse::<u32>().unwrap_or(1)).await {
        Ok(anime_list) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", anime_list)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/popular")]
async fn popular_list(page: web::Query<Page>) -> impl Responder {
    match scrape::popular::get(page.page.parse::<u32>().unwrap_or(1)).await {
        Ok(popular_list) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", popular_list)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/search")]
async fn search() -> impl Responder {
    match scrape::search::get("naruto").await {
        Ok(search) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", search)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(details)
            .service(episode)
            .service(recent_release)
            .service(popular_ongoing)
            .service(new_release)
            .service(anime_list)
            .service(popular_list)
            .service(search)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
