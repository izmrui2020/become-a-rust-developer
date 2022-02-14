//
use anyhow::Result;

pub mod message;
pub mod redis;

#[tokio::main]
async fn main() -> Result<()> {

    println!("Hello, world!");

    if let Err(error) = redis::subscribe(String::from("order")) {
        println!("{:?}", error);
    } else {
        println!("connected to queue");
    }

    println!("publish start");
    redis::publish_message(message::Message::new(
        message::Order::new("hoge".to_string(),
        3,
        24.4,
    )))?;

    println!("publish 1");

    redis::publish_message(message::Message::new(
        message::Order::new("fuga".to_string(),
        43,
        678.4379,
    )))?;

    redis::publish_message(message::Message::new(
        message::Order::new("forrbar".to_string(),
        10,
        47832.743829,
    )))?;
    
    Ok(())
}
