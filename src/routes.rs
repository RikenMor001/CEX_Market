use actix_web::{post, Responder, HttpResponse};
use actix_web::web::Json;

use crate::inputs::{
    trade_orders_input,
    order_deleted_input,
    market_depth_input,
};

use crate::outputs::{
    trade_order_response_output,
    order_deleted_output,
    market_depth_output,
};

#[post("/order")]
pub async fn create_order(body: Json<trade_orders_input>) -> impl Responder {

    let trade_orders_input {
        price,
        quantity,
        user_id,
        side,
    } = body.into_inner();

    HttpResponse::Ok().json(trade_order_response_output {
        order_id: 1,
        price,
        quantity,
        user_id,
        side,
    })
}

#[post("/delete")]
pub async fn delete_order(body: Json<order_deleted_input>) -> impl Responder {

    let order_deleted_input { order_id: _ } = body.into_inner();

    HttpResponse::Ok().json(order_deleted_output {
        filled_quantity: 50,
        average_price: 100,
    })
}

#[post("/depth")]
pub async fn market_depth(body: Json<market_depth_input>) -> impl Responder {

    let market_depth_input {
        symbol,
        quantity,
        bids,
        asks,
        price_spread: _,
    } = body.into_inner();

    let total_bids: u32 = bids.iter().sum();
    let total_asks: u32 = asks.iter().sum();
    let total_volume = total_bids + total_asks;

    let volume_flag = total_volume > quantity;

    HttpResponse::Ok().json(market_depth_output {
        symbol,
        market_price: 100,
        limit_price: 100,
        volume: volume_flag,
    })
}
