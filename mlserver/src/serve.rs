use crate::bin_client::BinStorageClient;
use crate::err::TribResult;
use crate::ml::{MLModel, MLStorage};
use crate::rpc::parameter_server_server;
use crate::server::MLServer;
use crate::storage::BinStorage;
use rand::distributions::Standard;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::net::ToSocketAddrs;
use std::sync::RwLock;
use tonic::transport::Server;

pub async fn serve_back(addr: String) -> TribResult<()> {
    let addr = addr.to_socket_addrs()?.as_slice()[0];

    let server = MLServer {
        addr: addr.to_string(),
        ml_model: new_ml_model().await?,
    };

    Server::builder()
        .add_service(parameter_server_server::ParameterServerServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}

pub async fn new_bin_client(backs: Vec<String>) -> TribResult<Box<dyn BinStorage>> {
    Ok(Box::new(BinStorageClient { addresses: backs }))
}

pub async fn new_ml_model() -> TribResult<Box<dyn MLModel>> {
    let ml_model = MLStorage {
        updater_queue: Default::default(),
        ws1: RwLock::new(HashMap::from([(
            "model1".to_string(),
            thread_rng().sample_iter(Standard).take(40).collect(),
        )])),
        bs1: RwLock::new(HashMap::from([(
            "model1".to_string(),
            thread_rng().sample_iter(Standard).take(40).collect(),
        )])),
        ready: RwLock::new(true),
        lr: 0.000001,
        clock: Default::default(),
    };
    Ok(Box::new(ml_model))
}
//TODO:: Add new worker creation
