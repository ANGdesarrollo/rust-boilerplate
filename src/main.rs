use actix_web::{App, HttpServer};
mod modules {
 pub mod item {
     pub mod presentation {
         pub mod routes {
             pub mod item_route;
         }
         pub mod controllers {
             pub mod item_controller;
         }
     }
     pub mod domain {
         pub mod use_cases {
             pub mod list_use_case;
             pub mod list_use_case_trait;
             pub mod create_use_case;
             pub mod create_use_case_trait;
         }
     }
 }
}

use item_route::list;
use item_route::create;
use crate::modules::item::presentation::routes::item_route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(list)
            .service(create)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
