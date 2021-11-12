
use actix_web::{App, HttpServer, web, get, post, Error, HttpResponse};

async fn index() -> Result<HttpResponse<>, Error> {
    Ok(HttpResponse::Ok().body("Ok"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    println!("Listening on port 50000");
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:50000")?
    .run()
    .await

}
