use crate::{
    http::{login_credentials, login_session, logout},
    database::DatabaseExecutor,
};
use r2d2::Pool;
use actix_cors::Cors;
use actix_files::Files;
use actix::{SystemRunner, Running, SyncArbiter};
use actix_web::{
    http::header::{CONTENT_TYPE, LOCATION},
    middleware,
    web::{get, post, resource},
    App, HttpServer, HttpResponse,
};
use std::{
    thread,
};
use diesel::{r2d2::ConnectionManager, prelude::*};
use anyhow::Result;
use webapp::{config::Config, API_URL_LOGIN_CREDENTIALS, API_URL_LOGIN_SESSION, API_URL_LOGOUT};
use url::Url;

pub struct Server {
    config: Config,
    runner: SystemRunner,
    url: Url,
}

impl Server {
    pub fn new(config: &Config) -> Result<()> {
        let runner = actix::System::new();

        let database = format!(
            "postgres://{}:{}@{}/{}",
            config.postgres.username,
            config.postgres.password,
            config.postgres.host,
            config.postgres.database,
        );

        let manager = ConnectionManager::<PgConnection>::new(database);
        let pool = Pool::builder().build(manager)?;
        let db_addr = SyncArbiter::start(num_cpus::get(), move || DatabaseExecutor(pool.clone()));

        let server = HttpServer::new(move || {
            App::new()
                .data(db_addr.clone())
                .wrap(
                    Cors::default()
                        .allowed_methods(vec!["GET", "POST"])
                        .allowed_header(CONTENT_TYPE)
                        .max_age(3600),
                )
                .wrap(middleware::Logger::default())
                // .service(resource(API_URL_LOGIN_CREDENTIALS).route(post().to(login_credentials)))
                // .service(resource(API_URL_LOGIN_SESSION).route(post().to(login_session)))
                // .service(resource(API_URL_LOGOUT).route(post().to(logout)))
                .service(Files::new("/", "./static/").index_file("index.html"))
        });

        runner.run();
        Ok(())
    }
}