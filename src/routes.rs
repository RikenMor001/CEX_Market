use actix_web::{post, Responder, HttpResponse};
use actix_web::web::Json;
use crate::inputs::trade_orders_input;
use crate::outputs::trade_order_response_output;

#[post("/order")]
pub async fn create_order(body: Json<trade_orders_input>) -> impl Responder { // body: Json<trade_orders> is a way to parse the request body
    let price = body.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;
    // Returns a json response
    HttpResponse::Ok().json(trade_order_response_output {
        order_id: 1,
        price: price,
        quantity: quantity,
        user_id: user_id,
        side: side
    })
}

#[post("/delete")]
pub async fn delete_order() -> impl Responder {
    "Order Created"
}

#[post("/depth")]
pub async fn market_depth() -> impl Responder {
    "Order Created"
}
