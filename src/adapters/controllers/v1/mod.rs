use crate::domain::use_cases::{create_user, get_user};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: u32,
    pub name: String,
}

pub async fn create_user_endpoint(req: web::Json<CreateUserRequest>) -> impl Responder {
    let user = create_user(1, req.name.clone());
    HttpResponse::Ok().json(UserResponse {
        id: user.id,
        name: user.name,
    })
}

#[derive(Deserialize)]
pub struct GetUserRequest {
    pub id: u32,
}

pub async fn get_user_endpoint(path: web::Path<GetUserRequest>) -> impl Responder {
    match get_user(path.id) {
        Some(user) => HttpResponse::Ok().json(UserResponse {
            id: user.id,
            name: user.name,
        }),
        None => HttpResponse::NotFound().finish(),
    }
}
