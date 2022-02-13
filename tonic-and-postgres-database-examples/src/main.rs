//
use anyhow::Result;
use dotenv;
use chrono;
use postgres;
use uuid;
use console::Style;
use tonic::{transport::Server};

use user::{
    crud_server::CrudServer
};

mod db_connection;
mod service;
use crate::service::User;

pub mod user {
    tonic::include_proto!("user");
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:20000".parse()?;
    let user = User::default();

    let blue = Style::new().blue();

    println!("\nRust gRPC Server ready at {}", blue.apply_to(addr));

    Server::builder()
        .add_service(CrudServer::new(user))
        .serve(addr)
        .await?;

    Ok(())
}
