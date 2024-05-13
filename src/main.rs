use actix_web::{App, HttpServer};
extern crate dotenv;
use dotenv::dotenv;
mod modules;
mod shared;

use crate::modules::item::presentation::routes::init_item_router;
use crate::shared::infrastructure::create_sea_orm_connection::init;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _db = init().await;
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .configure(init_item_router)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
