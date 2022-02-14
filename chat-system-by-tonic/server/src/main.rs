//
use anyhow::Result;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use futures::Stream;
use tokio::sync::{mpsc, RwLock};
use tonic::{transport::Server, Request, Response, Status, Streaming};

use chat::{Msg, Req, Empty};
use chat::chat_req_server::{ChatReq, ChatReqServer};

pub mod chat {
    tonic::include_proto!("chat");
}

#[derive(Debug)]
struct Shared {
    senders: HashMap<String, mpsc::Sender<Msg>>,
}

impl Shared {
    fn new() -> Self {
        Shared {
            senders: HashMap::new(),
        }
    }

    async fn broadcast(&self, msg: Msg) {
        for (name, tx) in &self.senders {
            match tx.send(msg.clone()).await {
                Ok(_) => {},
                Err(_) => {
                    println!("[Broadcast] SendError: to {}, {:?}", name, msg)
                }
            }
        }
    }
}

#[derive(Debug)]
struct ChatService {
    shared: Arc<RwLock<Shared>>,
}

impl ChatService {
    fn new(shared: Arc<RwLock<Shared>>) -> Self {
        ChatService { shared }
    }
}

#[tonic::async_trait]
impl ChatReq for ChatService {

    type ConnectServerStream = Pin<Box<dyn Stream<Item = Result<Msg, Status>> + Send + Sync + 'static>, >;

    async fn connect_server(
        &self,
        request: Request<Req>,
    ) -> Result<Response<Self::ConnectServerStream>, Status> {
        let name = request.into_inner().user_name;
        let (stream_tx, stream_rx) = mpsc::channel(1);

        let (tx, mut rx) = mpsc::channel(1);
        {
            self.shared.write().await.senders.insert(name.clone(), tx);
        }

        let shared_clone = self.shared.clone();
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match stream_tx.send(Ok(msg)).await {
                    Ok(_) => {}
                    Err(_) => {
                        println!(
                            "[Remote] stream tx sending error. Remote {}",
                            &name
                        );
                        shared_clone.write().await.senders.remove(&name);
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(stream_rx),
        )))
    }

    async fn send_msg(
        &self,
        request: Request<Msg>,
    ) -> Result<Response<Empty>, Status> {
        let req_data = request.into_inner();
        let user_name = req_data.user_name;
        let content = req_data.content;

        let msg = Msg { user_name, content };
        self.shared.read().await.broadcast(msg).await;

        Ok(Response::new(Empty {}))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:50000".parse().unwrap();
    println!("Send Msg Server Listening on: {}", addr);

    let instance = Shared::new();
    let shared = Arc::new(RwLock::new(instance));
    
    let chat_service = ChatService::new(shared.clone());

    Server::builder()
        .add_service(ChatReqServer::new(chat_service))
        .serve(addr)
        .await?;
    
    Ok(())
}