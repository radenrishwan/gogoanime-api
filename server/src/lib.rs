use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[derive(Debug, serde::Serialize)]
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

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(details)
            .service(episode)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
