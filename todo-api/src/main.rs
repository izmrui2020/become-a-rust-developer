#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, web, http, get, post, Error, HttpResponse};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

mod db;
use self::db::models::*;
use self::db::schema::todos::dsl::*;

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
