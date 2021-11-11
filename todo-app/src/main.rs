
use actix_web::{web, get, post, Error, App, HttpResponse, HttpServer, Responder};

async fn index() -> Result<Responder<()>, Error> {

    Ok(HttpResponse::Ok()
        .body("hello world")
        .expect("aaaaaaa")
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listning on port 8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}