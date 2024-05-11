use actix_web::{HttpResponse, Responder};

use crate::modules::item::domain::use_cases::create_use_case_trait::CreateUseCaseTrait;
use crate::modules::item::domain::use_cases::create_use_case::CreateUseCase;
use crate::modules::item::domain::use_cases::list_use_case_trait::ListUseCaseTrait;
use crate::modules::item::domain::use_cases::list_use_case::ListUseCase;

pub struct ItemController;

impl ItemController {
    pub async fn list() -> impl Responder {
        let result: Vec<String> = ListUseCase::handle().await;
        return HttpResponse::Ok().json(result);
    }

    pub async fn save(req_body: String) -> impl Responder {
        let result = CreateUseCase::handle(req_body).await;
        return HttpResponse::Ok().body(result);
    }

    // pub async fn get_one() {}
    //
    // pub async fn delete() {}
}