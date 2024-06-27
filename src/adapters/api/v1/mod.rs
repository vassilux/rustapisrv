use crate::adapters::repositories::user_repository::{insert_user, select_user};
use crate::domain::use_cases::{create_user, get_user};
use crate::framework::database::establish_connection;
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
    match establish_connection() {
        Ok(conn) => {
            let user = create_user(1, req.name.clone());
            match insert_user(&conn, user.id, &user.name) {
                Ok(_) => HttpResponse::Ok().json(UserResponse {
                    id: user.id,
                    name: user.name,
                }),
                Err(err) => {
                    eprintln!("Failed to insert user: {}", err);
                    HttpResponse::InternalServerError().body("Failed to insert user")
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to connect to the database: {}", err);
            HttpResponse::InternalServerError().body("Failed to connect to the database")
        }
    }
}

#[derive(Deserialize)]
pub struct GetUserRequest {
    pub id: u32,
}

pub async fn get_user_endpoint(path: web::Path<GetUserRequest>) -> impl Responder {
    match establish_connection() {
        Ok(conn) => match select_user(&conn, path.id) {
            Ok(Some(user)) => HttpResponse::Ok().json(UserResponse {
                id: user.id,
                name: user.name,
            }),
            Ok(None) => HttpResponse::NotFound().finish(),
            Err(err) => {
                eprintln!("Failed to query user: {}", err);
                HttpResponse::InternalServerError().body("Failed to query user")
            }
        },
        Err(err) => {
            eprintln!("Failed to connect to the database: {}", err);
            HttpResponse::InternalServerError().body("Failed to connect to the database")
        }
    }
}
