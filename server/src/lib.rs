use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

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

#[get("/api/details")]
async fn details() -> impl Responder {
    match scrape::detail::get().await {
        Ok(detail) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", detail)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/episode")]
async fn episode() -> impl Responder {
    match scrape::episode::get().await {
        Ok(episode) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", episode)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/recent-release")]
async fn recent_release() -> impl Responder {
    match scrape::recent_release::get(1).await {
        Ok(releases) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", releases)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/popular-ongoing")]
async fn popular_ongoing() -> impl Responder {
    match scrape::popular_ogoing::get(1).await {
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
async fn anime_list() -> impl Responder {
    match scrape::anime_list::get(1).await {
        Ok(anime_list) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", anime_list)),
        Err(e) => HttpResponse::InternalServerError().json(DefaultResponse::new(
            500,
            "Error while getting data",
            e.to_string(),
        )),
    }
}

#[get("/api/popular")]
async fn popular_list() -> impl Responder {
    match scrape::popular::get(1).await {
        Ok(popular_list) => HttpResponse::Ok().json(DefaultResponse::new(200, "OK", popular_list)),
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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
