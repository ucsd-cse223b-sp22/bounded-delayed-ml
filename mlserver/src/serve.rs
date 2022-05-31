use crate::err::TribResult;
use crate::rpc::parameter_server_server;
use crate::server::MLServer;
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
        ws1: RwLock::new(HashMap::from([(
            "model1".to_string(),
            thread_rng().sample_iter(Standard).take(40).collect(),
        )])),
        bs1: RwLock::new(HashMap::from([(
            "model1".to_string(),
            thread_rng().sample_iter(Standard).take(40).collect(),
        )])),
        updater_queue: Default::default(),
        lr: 0.000001,
        clock: Default::default(),
    };

    Server::builder()
        .add_service(parameter_server_server::ParameterServerServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
