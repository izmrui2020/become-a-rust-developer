#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, web, http, get, post, Error, HttpResponse};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
use actix_web::middleware::Logger;
use log::{info, warn, error};
use validator::{Validate};
use validator_derive::{Validate};

use dotenv::dotenv;
use serde::Deserialize;
use std::env;

mod db;
use self::db::models::*;
use self::db::schema::todos::dsl::*;

mod error;
use self::error::error::UserError;

mod api;

async fn todos_endpoint(pool: web::Data<DbPool>) -> Result<HttpResponse<>, UserError> {
    let connection = pool.get()
        .map_err(|_| {
            error!("Failed to get DB connection from pool");
            UserError::InternalError
        })?;

    let todo_data = web::block(move ||{
        todos.limit(30).load::<Todo>(&connection)
    })
    .await
    .map_err(|_| {
        error!("Failed to get todo data");
        UserError::InternalError
    })?;
    
    return Ok(HttpResponse::Ok().json(todo_data));
}

#[derive(Deserialize, Validate)]
struct todo_endpoint_path {
    #[validate(range(min=0, max=150))]
    id: i32,
}
async fn todo_endpoint(pool: web::Data<DbPool>, todo_id: web::Path<todo_endpoint_path>) -> Result<HttpResponse, UserError> {
    todo_id.validate().map_err(|_| {
        warn!("Parameter validation failed");
        UserError::ValidationError
    })?;
    let connection = pool.get()
        .map_err(|_| {
            error!("faild at db connection pool");
            UserError::InternalError
        })?;

    let query_id = todo_id.id.clone();
    let todo_data = web::block(move ||
        todos.filter(id.eq(query_id)).first::<Todo>(&connection)
    )
    .await
    .map_err(|e|
        match e {
            error::BlockingError::Error(diesel::result::Error::NotFound) => {
                error!("todo id: {} not found in db", &todo_id.id);
                UserError::NotFoundError
            },
            _ => {
                error!("Unexpected error");
                UserError::InternalError
            },
        }
    )?;
    
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
        .app_data(
            web::PathConfig::default()
            .error_handler(|_, _| {
            UserError::ValidationError.into()
        }))
        .route("/todos", web::get().to(todos_endpoint))
        .route("/todo/{id}", web::get().to(todo_endpoint))
    );
}

#[actix_web::main]
async fn main() ->  std::io::Result<()> {
    env_logger::init();

    let pool = setup_database();

    println!("Listening on port 50000");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .configure(api_config)
    })
    .bind("127.0.0.1:50000")?
    .run()
    .await

}
