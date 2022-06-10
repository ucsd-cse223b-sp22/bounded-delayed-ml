use crate::client::ParameterClient;
use crate::err::TribResult;
use crate::ml::{EmptyRequest, MLModel};
use crate::rpc::parameter_server_client::ParameterServerClient;
use tokio::time;

// Store the backend addresses as a struct with predecessor and successor for efficient lookup
#[derive(PartialEq, Debug, Clone)]
pub struct ChordBacks {
    pub this_addr: String,
    pub successor: String,
    pub predecessor: String,
}

pub struct SyncClockResp {
    pub largest_clock: u64,
    pub alive_backends: Vec<String>,
}

fn vec_equals(first: Vec<String>, second: Vec<String>) -> bool {
    if first.len() == second.len() {
        for ele in second.iter() {
            if !first.contains(ele) {
                return false;
            }
        }
        for ele in first.iter() {
            if !second.contains(ele) {
                return false;
            }
        }
    } else {
        return false;
    }
    return true;
}

pub async fn serve_keeper(backs: Vec<String>, keeper: String) -> TribResult<()> {
    return synchronize_clocks(backs).await;
}

#[allow(unused_must_use)]
async fn synchronize_clocks(backs: Vec<String>) -> TribResult<()> {
    let mut interval = time::interval(time::Duration::from_secs(2));
    let mut clock_val = 0;
    let mut sync_clock_resp: SyncClockResp;
    let mut backs_alive_last_checked: Vec<String> = vec![];
    let mut backs_alive_recent_check: Vec<String>;
    loop {
        interval.tick().await;
        sync_clock_resp = do_synchronize_clock(clock_val, backs.clone()).await?;
        clock_val = sync_clock_resp.largest_clock;
        backs_alive_recent_check = sync_clock_resp.alive_backends.clone();
        if !vec_equals(
            backs_alive_recent_check.clone(),
            backs_alive_last_checked.clone(),
        ) {
            // Spawn a new thread for data migration
            tokio::spawn(migrate_data(
                backs_alive_last_checked,
                backs_alive_recent_check,
            ));
        }
        backs_alive_last_checked = sync_clock_resp.alive_backends.clone();
    }
}

async fn migrate_data(
    backs_alive_last_checked: Vec<String>,
    backs_alive_recent_check: Vec<String>,
) {
    let old_chord_ring = build_chord_ring(backs_alive_last_checked.clone()).await;
    match old_chord_ring {
        Ok(old_ring) => {
            let new_chord_ring = build_chord_ring(backs_alive_recent_check.clone()).await;
            match new_chord_ring {
                Ok(new_ring) => {
                    let _ = do_data_migration(old_ring, new_ring).await;
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }
}

async fn do_synchronize_clock(
    mut largest_clock: u64,
    backs: Vec<String>,
) -> TribResult<SyncClockResp> {
    let mut max_clock = u64::MIN;
    let mut alive_backs: Vec<String> = Vec::new();
    for addr in backs.iter() {
        let mut keeper_conn = ParameterServerClient::connect(addr.to_string()).await;
        match keeper_conn {
            Ok(conn) => {
                alive_backs.push(addr.to_string());
                let keeper_client = ParameterClient { client: conn };
                // TODO: Need to check on READY flag here
                let clock_response = keeper_client.clock(largest_clock).await;
                match clock_response {
                    Ok(val) => {
                        largest_clock = val;
                        max_clock = val.max(max_clock);
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }
    Ok(SyncClockResp {
        largest_clock,
        alive_backends: alive_backs,
    })
}

async fn build_chord_ring(backs: Vec<String>) -> TribResult<Vec<ChordBacks>> {
    let mut chordbacks: Vec<ChordBacks> = Vec::new();
    for index in 0..backs.len() {
        let mut successor_backend_set = "".to_string();
        let mut predecessor_backend_set = "".to_string();
        if index == 0 {
            let mut predecessor_backend = backs[backs.len() - 1].clone();
            predecessor_backend_set = predecessor_backend;
            successor_backend_set = backs[(index + 1) % backs.len()].clone();
        } else if index == backs.len() - 1 {
            successor_backend_set = backs[(index + 1) % backs.len()].clone();
            if index > 0 {
                predecessor_backend_set = backs[index - 1].clone();
            }
        } else {
            predecessor_backend_set = backs[index - 1].clone();
            successor_backend_set = backs[(index + 1) % backs.len()].clone();
        }

        chordbacks.push(ChordBacks {
            this_addr: backs[index].clone(),
            predecessor: predecessor_backend_set.clone(),
            successor: successor_backend_set.clone(),
        });
    }
    return Ok(chordbacks);
}

async fn do_data_migration(
    backs_old: Vec<ChordBacks>,
    backs_new: Vec<ChordBacks>,
) -> TribResult<()> {
    if backs_new.len() == 1 {
        return Ok(());
    }

    // Handle nodes that failed
    for (idx_back, back) in backs_old.iter().enumerate() {
        let mut found = false;
        for back_check in backs_new.iter() {
            if back_check.this_addr.clone() == back.this_addr.clone() {
                found = true;
                break;
            }
        }
        if !found {
            let predAddress = back.predecessor.clone();
            let succAddress = back.successor.clone();

            let predAddressFormatted = format!("http://{}", predAddress.clone());
            let succAddressFormatted = format!("http://{}", succAddress.clone());

            let mut pred_conn =
                ParameterServerClient::connect(predAddressFormatted.clone()).await?;
            let pred_client = ParameterClient { client: pred_conn };

            let succ_conn = ParameterServerClient::connect(succAddressFormatted.clone()).await?;
            let succ_client = ParameterClient { client: succ_conn };

            succ_client.set_ready(EmptyRequest{
                empty: false
            }).await?;
            let data_on_predecessor = pred_client.get_model_dump().await?;
            let data_on_successor = succ_client.get_model_dump().await?;

            pred_client.merge_model_dump(data_on_successor).await?;
            succ_client.merge_model_dump(data_on_predecessor).await?;
            succ_client.set_ready(EmptyRequest{
                empty: true
            }).await?;
            // TODO: Handle SuccessorSuccessor data
        }
    }

    // Handle nodes that joined
    for back in backs_new.iter() {
        let mut found = false;
        for back_check in backs_old.iter() {
            if back_check.this_addr.clone() == back.this_addr.clone() {
                found = true;
                break;
            }
        }
        if !found {
            let predAddress = back.predecessor.clone();
            let succAddress = back.successor.clone();
            let thisAddress = back.this_addr.clone();

            let thisAddressFormatted = format!("http://{}", thisAddress.clone());
            let succAddressFormatted = format!("http://{}", succAddress.clone());
            let thisAddressFormatted = format!("http://{}", thisAddress.clone());

            let mut this_conn =
                ParameterServerClient::connect(thisAddressFormatted.clone()).await?;
            let this_conn = ParameterClient { client: this_conn };

            let succ_conn = ParameterServerClient::connect(succAddressFormatted.clone()).await?;
            let succ_client = ParameterClient { client: succ_conn };

            let mut this_conn = ParameterServerClient::connect(thisAddressFormatted.clone()).await?;
            let this_client = ParameterClient {client: this_conn};

            // TODO: Set ready flag to false
            let data_on_successor = succ_client.get_model_dump().await?;
            this_conn.merge_model_dump(data_on_successor).await?;
            // TODO: Set ready flag to true
        }
    }
    Ok(())
}
