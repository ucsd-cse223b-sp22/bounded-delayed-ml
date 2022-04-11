use std::borrow::Borrow;
use std::future::Future;
use tokio::sync::mpsc::Receiver;
use tonic::transport::Server;
use tribbler::rpc::trib_storage_server;
use tribbler::{config::BackConfig, err::TribResult, storage::Storage};

/// an async function which blocks indefinitely until interrupted serving on
/// the host and port specified in the [BackConfig] parameter.
use crate::lab1::server::StorageServer;

pub async fn serve_back(config: BackConfig) -> TribResult<()> {
    println!("I AM HERE 1");
    let addr = config.addr.parse().unwrap();
    let server = StorageServer {
        mem_storage: Default::default(),
    };
    println!("I AM HERE 2");
    Server::builder()
        .add_service(trib_storage_server::TribStorageServer::new(server))
        .serve(addr)
        .await;
    Ok(())
}

/// This function should create a new client which implements the [Storage]
/// trait. It should communicate with the backend that is started in the
/// [serve_back] function.
use crate::lab1::client::StorageClient;

pub async fn new_client(addr: &str) -> TribResult<Box<dyn Storage>> {
    Ok(Box::new(StorageClient {
        addr: addr.to_string(),
    }))
}
