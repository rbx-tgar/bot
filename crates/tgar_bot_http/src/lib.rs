use actix_web::{App, HttpServer, web::Data};
use tgar_bot_utils::mongo_client;
use tracing_actix_web::TracingLogger;

use crate::routes::{
    get_player, webhook,
    xp::{add_xp, remove_xp, set_xp},
};

mod models;
mod routes;

pub async fn create_server() -> std::io::Result<()> {
    let client = mongo_client().await;
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(Data::new(client.clone()))
            .service(get_player)
            .service(set_xp)
            .service(add_xp)
            .service(remove_xp)
            .service(webhook)
    })
    .bind("127.0.0.1:5750")?
    .run()
    .await
}
