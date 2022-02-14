//
use anyhow::Result;


pub trait Server {
    fn run();
}

pub struct ApiServer {
    id: String,
}