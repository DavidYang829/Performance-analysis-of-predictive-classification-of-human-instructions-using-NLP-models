use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
struct PiResponse {
    number: f64,
    pi_result: f64,
}

#[get("/")]
async fn index() -> impl Responder {
    let response = "Hello! Welcome to Yucheng Yang's Web APP~\n\
                    this is the local test\n\
                    Individual Project 2\n\
                    Yucheng Yang yy341\n\
                    IDS 721";
    
    response
}

#[get("/pi")]
async fn pi(web::Query(info): web::Query<HashMap<String, String>>) -> impl Responder {
    match info.get("number") {
        Some(num_str) => {
            match num_str.parse::<f64>() {
                Ok(number) => {
                    let pi_result = number * std::f64::consts::PI;
                    let response = PiResponse { number, pi_result };
                    HttpResponse::Ok().json(response)
                }
                Err(_) => HttpResponse::BadRequest().body("Invalid number provided"),
            }
        }
        None => HttpResponse::BadRequest().body("Missing number query parameter"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(pi) 
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
