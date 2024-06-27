mod adapters;
mod domain;
mod framework;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    framework::web::start_server().await
}
