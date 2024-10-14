#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");
    server::run().await
}
