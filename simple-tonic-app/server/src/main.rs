//
use anyhow::Result;
use structopt::{clap, StructOpt};
use std::path::PathBuf;
use tonic::{transport::Server, Request, Response, Status};

pub mod demo {
    tonic::include_proto!("user");
}

#[derive(Debug, StructOpt)]
#[structopt(name = "tonic_and_postgres")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Opt {
    #[structopt(short = "p", long = "postgres")]
    pub postgres: String
}

fn main() {
    let opt = Opt::from_args();
    
    println!("Hello, world!");
}
