use actix_web::{App, post, HttpServer, Responder};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(move || {
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

struct Rect{
    width: u32,
    height: u32
}

#[post("/order")]
async fn create_order(r: Request) -> impl Responder {
    let r: Rect = Rect{
        width: 30,
        height: 20
    }
    return HttpResponse::Ok().json(r);
}

#[post("/delete")]
async fn delete_order() -> impl Responder {
    "Order Created"
}

#[post("/depth")]
async fn market_depth() -> impl Responder {
    "Order Created"
}
