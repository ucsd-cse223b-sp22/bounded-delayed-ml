use log::{info, warn};
use std::net::ToSocketAddrs;
use tokio::join;
use cmd::backs::BACKS;

#[tokio::main]
pub async fn main() {

    let backs = BACKS.map(|x| x.to_string()).to_vec();
    let mut handles = vec![];
    println!("STARTING BACKENDS");
    for (i, srv) in backs.iter().enumerate() {
        handles.push(tokio::spawn(run_srv(srv.to_string())));
    }

    for h in handles {
        match join!(h) {
            (Ok(_), ) => (),
            (Err(e), ) => {
                warn!("failed to join: {}", e);
            }
        };
    }
}

#[allow(unused_must_use)]
async fn run_srv(adr: String) {
    // let (shut_tx, shut_rx) = tokio::sync::mpsc::channel(1);
    println!("starting backend on {}", adr);
    mlserver::serve::serve_back(adr).await;
}

