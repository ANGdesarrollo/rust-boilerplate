use actix_web::{App, HttpServer, web};
mod modules;

use crate::modules::item::presentation::routes::init_item_router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(init_item_router)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
