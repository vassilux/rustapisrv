mod adapters;
mod domain;
mod framework;

use framework::web::start_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start_server().await
}
