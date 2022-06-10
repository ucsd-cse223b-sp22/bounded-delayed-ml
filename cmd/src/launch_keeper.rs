use log::{info, warn};
use std::net::ToSocketAddrs;
use tokio::join;

#[tokio::main]
pub async fn main() {
    let backs = vec![
        "127.0.0.1:34151".to_string()
    ];
    let keeper = [
        "127.0.0.1:30013".to_string()
    ];
    println!("STARTING KEEPER");
    let mut handles = vec![];
    for (i, srv) in keeper.iter().enumerate() {
        handles.push(tokio::spawn(run_srv(backs, srv.to_string())));
    }

    for h in handles {
        match join!(h) {
            (Ok(_), ) => (),
            (Err(e), ) => {
                warn!("failed to launch keeper: {}", e);
            }
        };
    }
}

#[allow(unused_must_use)]
async fn run_srv(backs: Vec<String>, keeper: String) {
    println!("starting keeper on {}", keeper);
    mlserver::keeper::serve_keeper(backs, keeper).await;
}


