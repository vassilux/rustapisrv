use actix_web::web;

use crate::adapters::api::v1::{create_user_endpoint, get_user_endpoint};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/user", web::post().to(create_user_endpoint))
            .route("/user/{id}", web::get().to(get_user_endpoint)),
    );
}
