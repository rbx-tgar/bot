use actix_web::{
    HttpResponse, get,
    http::StatusCode,
    post,
    web::{Json, Path},
};
use reqwest::Client;
use serde_json::Value;
use tgar_bot_mongodb::models::player::Player;

pub mod xp;

#[get("/player/{id}")]
pub async fn get_player(id: Path<i64>) -> HttpResponse {
    let player = Player::from_user_id(id.into_inner()).await;
    HttpResponse::Ok().json(player)
}

#[post("/webhook/{webhook_id}/{webhook_token}")]
pub async fn webhook(path: Path<(String, String)>, body: Json<Value>) -> HttpResponse {
    let (webhook_id, webhook_token) = path.into_inner();
    let webhook_url = format!("https://discord.com/api/webhooks/{}/{}", webhook_id, webhook_token);

    let client = Client::new();
    let result = client.post(&webhook_url).json(&body.into_inner()).send().await;

    match result {
        Ok(response) => {
            let status =
                StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::BAD_REQUEST);
            let text = response.text().await.unwrap();
            HttpResponse::build(status).body(text)
        }
        Err(_) => HttpResponse::InternalServerError().body("failed to proxy webhook"),
    }
}
