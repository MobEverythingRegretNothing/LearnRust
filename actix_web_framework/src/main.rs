use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

#[get("/macro_style")]
async fn macro_style() -> impl Responder {
    HttpResponse::Ok().body("Hello macro service style")   
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}

struct AppState {
    counter: Mutex<i32>,
}

async fn index(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number {counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppState{
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .service(macro_style)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 9988))?
    .run()
    .await
}