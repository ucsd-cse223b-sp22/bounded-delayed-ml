use log::{info, warn};
use std::net::ToSocketAddrs;
use rand::{Rng, thread_rng};
use rand::distributions::Standard;
use tokio::join;
use cmd::backs::BACKS;
use mlserver::err::TribResult;
use mlserver::ml::DoubleList;
use mlserver::serve::new_bin_client;

#[tokio::main]
pub async fn main() -> TribResult<()> {
    let model_names = [
        "model1".to_string(),
        "model2".to_string()
    ];
    let backs = BACKS.map(|x| x.to_string()).to_vec();
    let bin_client = new_bin_client(backs).await?;
    for m_name in model_names {
        bin_client.bin(&*m_name).await?.initialize(DoubleList{
            clock: 0,
            model_name: m_name.to_string(),
            ws1: thread_rng().sample_iter(Standard).take(40).collect(),
            bs1: thread_rng().sample_iter(Standard).take(40).collect()
        }).await?;
    }
    Ok(())
}

