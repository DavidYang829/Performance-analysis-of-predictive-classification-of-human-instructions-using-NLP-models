use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    let response = "Hello! Welcome to Yucheng Yang's Web APP~\n\
                    this is the local test\n\
                    Yucheng Yang yy341\n\
                    IDS 721";
    
    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
