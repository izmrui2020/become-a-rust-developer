#[macro_use]
extern crate diesel;

use actix_files::Files;
use actix_web::{web, get, post, Result, Error, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
//use r2d2_diesel::ConnectionManager;
use std::collections::HashMap;
use serde::Serialize;
use std::thread;
use handlebars::Handlebars;
use awmp::Parts;

use dotenv::dotenv;
use std::env;

use self::models::*;
mod models;
mod schema;
use self::schema::todos::dsl::*;

#[derive(Serialize)]
struct IndexTemplateData {
    project_name: String,
    todos: Vec<self::models::Todo>
}

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
async fn index(hb: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Can't get db connection from pool");
    
    let todo_data = web::block(move || {
        todos.limit(5).load::<Todo>(&connection)
    })
        .await
        .map_err(|_| {
            HttpResponse::InternalServerError().finish()
        })?;
    
    let data = IndexTemplateData {
        project_name: "Todo".to_string(),
        todos: todo_data,
    };
    let body = hb.render("index", &data).unwrap();
    
    Ok(HttpResponse::Ok().body(body))
}

async fn add_todo_form(pool: web::Data<DbPool>, mut parts: Parts) {
    let text_fields: HashMap<_, _> = parts.texts.as_pairs().into_iter().collect();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    /////for .html
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    //for database
    let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    ///for server
    println!("Listning on port 8080");
    HttpServer::new(move|| {
        App::new()
            .app_data(handlebars_ref.clone())
            .data(pool.clone())
            .service(
                Files::new("/static", "static")
                    .show_files_listing(),
            )
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}