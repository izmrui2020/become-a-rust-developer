#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, web, http, get, post, Error, HttpResponse};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use dotenv::dotenv;
use std::env;

mod db;
use self::db::models::*;
use self::db::schema::todos::dsl::*;

mod error;

mod api;

async fn todos_endpoint(pool: web::Data<DbPool>) -> Result<HttpResponse<>, Error> {
    let connection = pool.get().expect("can't get db connection from pool");

    let todo_data = web::block(move ||{
        todos.limit(30).load::<Todo>(&connection)
    })
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())?;
    
    return Ok(HttpResponse::Ok().json(todo_data));
}
async fn todo_endpoint() -> Result<HttpResponse<>, Error> {
    Ok(HttpResponse::Ok().body("Ok"))
}

fn setup_database() -> DbPool {
    dotenv().ok(); //call env

    let database_url = env::var("DATABASE_URL")
        .expect("Database url must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    //Q serch r2d2::Pool::builder(), .build()
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.")
}

//Q serch cfg: & mut web::ServiceConfig
fn api_config(cfg: & mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        .app_data(web::PathConfig::default().error_handler(|_, _| {
            UserError::ValidationError.into()
        }))
        .route("/todos", web::get().to(todos_endpoint))
        .route("/todo/{id}", web::get().to(todo_endpoint))
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = setup_database();

    println!("Listening on port 50000");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(api_config)
    })
    .bind("127.0.0.1:50000")?
    .run()
    .await

}
