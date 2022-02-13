//
use postgres::{tls::NoTls, Client};
use dotenv::dotenv;
use std::env;

pub fn estabilish_connection() -> Client {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Client::connect(&database_url, NoTls).unwrap()
}