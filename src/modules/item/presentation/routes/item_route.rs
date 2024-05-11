use actix_web::{get, post, Responder};

use crate::modules::item::presentation::controllers::item_controller::ItemController;

#[get("/")]
async fn list() -> impl Responder {
    ItemController::list().await
}

#[post("/")]
async fn create(req_body: String) -> impl Responder {
    ItemController::save(req_body).await
}
