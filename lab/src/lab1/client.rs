pub struct StorageClient {
    pub addr: String,
}

use async_trait::async_trait;
use tribbler::err::TribResult;
use tribbler::rpc;
use tribbler::rpc::trib_storage_client::TribStorageClient;
use tribbler::rpc::Key;
use tribbler::storage::{KeyString, KeyValue, List, Pattern};

#[async_trait] // VERY IMPORTANT !!
impl KeyString for StorageClient {
    async fn get(&self, key: &str) -> TribResult<Option<String>> {
        let mut client = TribStorageClient::connect(self.addr.clone()).await?;
        let r = client
            .get(Key {
                key: key.to_string(),
            })
            .await?;
        match r.into_inner().value {
            value => Ok(Some(value)),
        }
    }

    async fn set(&self, kv: &KeyValue) -> TribResult<bool> {
        let mut client = TribStorageClient::connect(self.addr.clone()).await?;
        let r = client
            .set(rpc::KeyValue {
                key: kv.key,
                value: kv.value,
            })
            .await?;
        Ok(true)
    }

    async fn keys(&self, p: &Pattern) -> TribResult<List> {
        let mut client = TribStorageClient::connect(self.addr.clone()).await?;
        let r = client
            .keys(rpc::Pattern {
                prefix: p.prefix,
                suffix: p.suffix,
            })
            .await?;
        let val = r.into_inner().list;
        return Ok(List(val));
    }
}
