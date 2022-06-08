use crate::client::ParameterClient;
use crate::err::{TribResult, TribblerError};
use crate::ml::MLModel;
use crate::ps_bin::MyParameterServerBin;
use crate::rpc::parameter_server_client::ParameterServerClient;
use crate::rpc::parameter_server_server::ParameterServer;
use crate::storage::BinStorage;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct BinStorageClient {
    pub addresses: Vec<String>,
    // pub bin_map: Arc<RwLock<HashMap<String, Bin>>>,
}

async fn get_successor(start_index: i32, addresses: Vec<String>) -> TribResult<i32> {
    let count = addresses.len() as i32;
    // println!("count: {}", count);
    // println!("start index: {}", start_index);
    if count == 1 {
        return Ok(0);
    }
    for i in 0..count {
        let index = (start_index + i) % count;
        let addr = addresses[index as usize].clone();
        let address = format!("http://{}", addr.clone());
        // let mut client_conn = ParameterServerClient::connect(address.clone()).await;
        // match client_conn {
        //     Ok(conn) => {
        //         let backend_client = ParameterClient { client: conn };
        return Ok(index as i32);
        // let mut ready = backend_client.get(key_constant::READY).await;
        // match ready {
        //     Ok(val) => {
        //         if val == None {
        //             continue;
        //         } else {
        //             if val.unwrap() == key_constant::FALSE {
        //                 ready = backend_client.get(key_constant::READY).await;
        //             }
        //             println!("Found Index {}", index);
        //             return Ok(index as i32);
        //         }
        //     }
        //     Err(_) => continue,
        // }
        // }
        // Err(_) => continue,
        // }
    }
    return Err(Box::new(TribblerError::RpcError(
        "Could not find successor".to_string(),
    )));
}

#[async_trait::async_trait]
impl BinStorage for BinStorageClient {
    async fn bin(&self, name: &str) -> TribResult<Box<dyn MLModel>> {
        let index = hash_into_bin(name.to_string()) % (self.addresses.len());
        let primary_index = get_successor(index as i32, self.addresses.clone()).await?;
        if primary_index == -1 {
            return Err(Box::new(TribblerError::RpcError(
                "Could not find primary".to_string(),
            )));
        }
        let secondary_index = get_successor(primary_index + 1, self.addresses.clone()).await?;
        if secondary_index == -1 {
            return Err(Box::new(TribblerError::RpcError(
                "Could not find secondary".to_string(),
            )));
        }
        let no_replica = primary_index == secondary_index;
        let primary_address = self.addresses.get(primary_index as usize);
        let secondary_address = self.addresses.get(secondary_index as usize);
        if primary_address != None && secondary_address != None {
            let unwrapped_primary = format!("http://{}", primary_address.unwrap());
            let unwrapped_secondary = format!("http://{}", secondary_address.unwrap());
            let mut prim_client_conn =
                ParameterServerClient::connect(unwrapped_primary.clone()).await?;
            let storage_client = ParameterClient {
                client: prim_client_conn,
            };
            let mut rep_client_conn =
                ParameterServerClient::connect(unwrapped_secondary.clone()).await?;
            let replica_client = ParameterClient {
                client: rep_client_conn,
            };
            Ok(Box::new(MyParameterServerBin {
                name: name.to_string(),
                client: Box::new(storage_client),
                replica_client: Box::new(replica_client),
                no_replica,
            }))
        } else {
            return Err(Box::new(TribblerError::Unknown(
                "failed to get address".to_string(),
            )));
        }
    }
}

fn hash_into_bin(name: String) -> usize {
    let mut s = DefaultHasher::new();
    name.hash(&mut s);
    s.finish() as usize
}
