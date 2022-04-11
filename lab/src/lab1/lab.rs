use std::net::ToSocketAddrs;
use tonic::transport::Server;
use tribbler::rpc::trib_storage_server;
use tribbler::{config::BackConfig, err::TribResult, storage::Storage};

/// This function should create a new client which implements the [Storage]
/// trait. It should communicate with the backend that is started in the
/// [serve_back] function.
use crate::lab1::client::StorageClient;
/// an async function which blocks indefinitely until interrupted serving on
/// the host and port specified in the [BackConfig] parameter.
use crate::lab1::server::StorageServer;

pub async fn serve_back(config: BackConfig) -> TribResult<()> {
    let addr = config.addr.to_socket_addrs()?.as_slice()[0];

    let server = StorageServer {
        mem_storage: config.storage,
    };

    match config.ready {
        Some(inner) => {
            inner.send(true);
            ()
        }
        None => {}
    }

    match config.shutdown {
        None => {
            Server::builder()
                .add_service(trib_storage_server::TribStorageServer::new(server))
                .serve(addr)
                .await?;
        }
        Some(mut sd) => {
            Server::builder()
                .add_service(trib_storage_server::TribStorageServer::new(server))
                .serve_with_shutdown(addr, async {
                    sd.recv().await;
                    ()
                })
                .await?
        }
    };

    Ok(())
}

pub async fn new_client(addr: &str) -> TribResult<Box<dyn Storage>> {
    Ok(Box::new(StorageClient {
        addr: addr.to_string(),
    }))
}
