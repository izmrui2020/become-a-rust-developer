//
use anyhow::Result;
use structopt::{clap, StructOpt};
use std::path::PathBuf;

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
