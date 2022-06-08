use log::{info, warn};
use std::net::ToSocketAddrs;
use tokio::join;

#[tokio::main]
pub async fn main() {
    let backs = [
        "127.0.0.1:34151".to_string(),
        "127.0.0.1:34152".to_string()
    ];
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

