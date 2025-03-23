use mongodb::{Client, bson::doc};

use crate::{auto_ranking::try_rank, models::player::Player};

mod auto_ranking;
pub mod models;

pub async fn set_xp(client: &Client, user_id: i64, xp: i64) -> Result<(), String> {
    let collection = client.database("game").collection::<Player>("players");

    let filter = doc! { "id": user_id };
    let update = doc! { "$set": { "xp": xp } };

    match collection.update_one(filter, update).await {
        Ok(res) => {
            if res.matched_count == 1 {
                try_rank(user_id as u64).await;
                Ok(())
            } else {
                Err("player not found".to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn add_xp(client: &Client, user_id: i64, xp: i64) -> Result<(), String> {
    let collection = client.database("game").collection::<Player>("players");

    let filter = doc! { "id": user_id };
    let update = doc! { "$inc": { "xp": xp } };

    match collection.update_one(filter, update).await {
        Ok(res) => {
            if res.matched_count == 1 {
                try_rank(user_id as u64).await;
                Ok(())
            } else {
                Err("player not found".to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn remove_xp(client: &Client, user_id: i64, xp: i64) -> Result<(), String> {
    let collection = client.database("game").collection::<Player>("players");

    let filter = doc! { "id": user_id };
    let update = doc! { "$inc": { "xp": -xp } };

    match collection.update_one(filter, update).await {
        Ok(res) => {
            if res.matched_count == 1 {
                try_rank(user_id as u64).await;
                Ok(())
            } else {
                Err("player not found".to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
