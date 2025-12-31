use actix_web::{post, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use actix_web::web::Json;

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

#[derive(Serialize, Deserialize)]
struct Rect {
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize)]
enum Side{
    Buy,
    Sell
}
#[derive(Serialize, Deserialize)]
struct trade_orders {
    price: u32,
    quantity: u32,
    user_id: u32,
    side: Side
}

#[post("/order")]
async fn create_order(body: Json<trade_orders>) -> impl Responder { // body: Json<trade_orders> is a way to parse the request body
    let r = Rect {
        width: 30,
        height: 20,
    };

    HttpResponse::Ok().json(r)
}

#[post("/delete")]
async fn delete_order() -> impl Responder {
    "Order Created"
}

#[post("/depth")]
async fn market_depth() -> impl Responder {
    "Order Created"
}
