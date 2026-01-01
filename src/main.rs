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

#[derive(Serialize, Deserialize, Debug)]
enum Side{
    Buy,
    Sell
}
#[derive(Serialize, Deserialize, Debug)]
struct trade_orders { // Create a struct to represent the request body
    price: u32,
    quantity: u32,
    user_id: u32,
    side: Side
}

#[derive(Serialize, Deserialize, Debug)]
struct trade_order_response {
    order_id: u32
}

// app.post("/order", (req, res) => {
//     const {price, quantity} = req.body;
// }) // This is the equivalent of the create_order function

#[post("/order")]
async fn create_order(body: Json<trade_orders>) -> impl Responder { // body: Json<trade_orders> is a way to parse the request body
    let price = body.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;
    // Returns a json response
    HttpRespone::ok().json(trade_order_response {
        order_id: String::from("Added to order book")
    })
}

#[post("/delete")]
async fn delete_order() -> impl Responder {
    "Order Created"
}

#[post("/depth")]
async fn market_depth() -> impl Responder {
    "Order Created"
}
