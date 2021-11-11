use actix_files::{NamedFile};
use actix_web::{web, get, post, Result, Error, App, HttpResponse, HttpServer, Responder};

async fn index() -> Result<NamedFile> {

    Ok(NamedFile::open("./static/index.html")?
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