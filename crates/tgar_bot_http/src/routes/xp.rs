use actix_web::{
    HttpResponse, patch,
    web::{Data, Json},
};
use mongodb::Client;

use crate::models::player::ExpOperation;

#[patch("/set_xp")]
pub async fn set_xp(client: Data<Client>, operation: Json<ExpOperation>) -> HttpResponse {
    match tgar_bot_mongodb::set_xp(&client, operation.id, operation.xp).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[patch("/add_xp")]
pub async fn add_xp(client: Data<Client>, operation: Json<ExpOperation>) -> HttpResponse {
    match tgar_bot_mongodb::add_xp(&client, operation.id, operation.xp).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[patch("/remove_xp")]
pub async fn remove_xp(client: Data<Client>, operation: Json<ExpOperation>) -> HttpResponse {
    match tgar_bot_mongodb::remove_xp(&client, operation.id, operation.xp).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
