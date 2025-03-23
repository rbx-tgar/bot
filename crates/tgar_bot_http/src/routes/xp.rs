use actix_web::{
    HttpResponse, patch,
    web::{Data, Json},
};
use futures::future::try_join_all;
use mongodb::Client;

use crate::models::player::ExpOperation;

#[patch("/set_xp")]
pub async fn set_xp(client: Data<Client>, batch: Json<Vec<ExpOperation>>) -> HttpResponse {
    let futures: Vec<_> = batch
        .into_inner()
        .into_iter()
        .map(|op| tgar_bot_mongodb::set_xp(&client, op.id, op.xp))
        .collect();

    let res = try_join_all(futures).await;

    if res.is_err() {
        HttpResponse::InternalServerError().finish()
    } else {
        HttpResponse::Ok().finish()
    }
}

#[patch("/add_xp")]
pub async fn add_xp(client: Data<Client>, batch: Json<Vec<ExpOperation>>) -> HttpResponse {
    let futures: Vec<_> = batch
        .into_inner()
        .into_iter()
        .map(|op| tgar_bot_mongodb::add_xp(&client, op.id, op.xp))
        .collect();

    let res = try_join_all(futures).await;

    if res.is_err() {
        HttpResponse::InternalServerError().finish()
    } else {
        HttpResponse::Ok().finish()
    }
}

#[patch("/remove_xp")]
pub async fn remove_xp(client: Data<Client>, batch: Json<Vec<ExpOperation>>) -> HttpResponse {
    let futures: Vec<_> = batch
        .into_inner()
        .into_iter()
        .map(|op| tgar_bot_mongodb::remove_xp(&client, op.id, op.xp))
        .collect();

    let res = try_join_all(futures).await;

    if res.is_err() {
        HttpResponse::InternalServerError().finish()
    } else {
        HttpResponse::Ok().finish()
    }
}
