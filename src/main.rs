use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[derive(Debug, serde::Serialize)]
struct DefaultResponse<T> {
    status: u8,
    message: &'static str,
    data: T,
}

impl<T> DefaultResponse<T> {
    fn new(status: u8, message: &'static str, data: T) -> Self {
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
    match scrape::home::scrape().await {
        Ok(data) => {
            HttpResponse::Ok().json(DefaultResponse::new(200, "Success getting data", data))
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
