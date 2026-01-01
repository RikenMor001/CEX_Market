use actix_web::{App, HttpServer};
use crate::routes::{create_order, delete_order, market_depth};

pub mod routes;
pub mod inputs;
pub mod outputs;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
HttpServer::new(|| {
        App::new()
            .service(create_order)
            .service(delete_order)
            .service(market_depth)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

