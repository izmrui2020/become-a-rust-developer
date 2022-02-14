use std::fmt::Result;

//
use redis::{Client, Commands, PubSubCommands, ControlFlow};

use crate::message;

use super::message::Message;

pub fn publish_message(message: Message) -> anyhow::Result<()> {
    let client = redis::Client::open("redis://localhost:6666")?;

    let mut con = client.get_connection()?;

    let json = serde_json::to_string(&message)?;
    
    con.publish(message.channel, json)?;

    Ok(())
}

pub fn subscribe(channel: String) -> anyhow::Result<()> {

        let client = Client::open("redis://localhost:6666")?;

        let mut con = client.get_connection().unwrap();

        let _ = tokio::spawn(async move {
            let _: () = con.subscribe(&[channel], |msg| {
            let received: String = msg.get_payload().unwrap();
            let message_obj = serde_json::from_str::<Message>(&received).unwrap();

            print(message_obj);

            return ControlFlow::Continue;
        }).unwrap();
        });

    Ok(())
}

pub fn print(message: Message) {
    println!("{:?}", &message);
}