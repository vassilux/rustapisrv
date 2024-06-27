mod v1;

use crate::framework::web::v1::init_routes as init_v1_routes;
use actix_web::{App, HttpServer};

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(init_v1_routes))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
}
