#[macro_use]
extern crate diesel;

use actix_files::Files;
use actix_web::{web, get, post, Result, Error, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;
use std::thread;


use dotenv::dotenv;
use std::env;

use handlebars::Handlebars;

async fn index() -> Result<()> {

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    /////for .html
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    //for database
    let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);
    let pool =

    println!("Listning on port 8080");
    HttpServer::new(move|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}