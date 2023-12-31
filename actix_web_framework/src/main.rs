use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct CounterState {
    counter: Mutex<i32>,
}

async fn counter_handler(data: web::Data<CounterState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number {counter}")
}

fn counting_config(cfg: &mut web::ServiceConfig) {
    let counter = web::Data::new(CounterState{
        counter: Mutex::new(0)
    });

    cfg.service(
        web::resource("/")
        .app_data(counter.clone())
        .route(web::get().to(counter_handler))
        .route(web::head().to(|| async { HttpResponse::MethodNotAllowed() }))
        .route(web::post().to(|| async { HttpResponse::MethodNotAllowed() }))
        .route(web::put().to(|| async { HttpResponse::MethodNotAllowed() }))
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .configure(counting_config)
    })
    .bind(("127.0.0.1", 9988))?
    .run()
    .await
}