use async_trait::async_trait;
use tonic::Code;

use tribbler::err::TribResult;
use tribbler::rpc;
use tribbler::rpc::trib_storage_client::TribStorageClient;
use tribbler::rpc::{Clock, Key};
use tribbler::storage::{KeyList, KeyString, KeyValue, List, Pattern, Storage};

pub struct StorageClient {
    pub addr: String,
}

#[async_trait] // VERY IMPORTANT !!
impl KeyString for StorageClient {
    async fn get(&self, key: &str) -> TribResult<Option<String>> {
        let mut client = TribStorageClient::connect(self.addr.clone()).await?;
        let r = client
            .get(Key {
                key: key.to_string(),
            })
            .await?;
        let value = r.into_inner().value;
        let default_string = "".to_string();
        if value == default_string {
            Ok(None)
        } else {
            Ok(Some(value))
        }
    }

    async fn set(&self, kv: &KeyValue) -> TribResult<bool> {
        let mut client = TribStorageClient::connect(self.addr.clone()).await?;
        let r = client
            .set(rpc::KeyValue {
                key: kv.key.clone(),
                value: kv.value.clone(),
            })
            .await?;
        Ok(r.into_inner().value)
    }

    async fn keys(&self, p: &Pattern) -> TribResult<List> {
        let mut client = TribStorageClient::connect(self.addr.clone()).await?;
        let r = client
            .keys(rpc::Pattern {
                prefix: p.prefix.clone(),
                suffix: p.suffix.clone(),
            })
            .await?;
        let val = r.into_inner().list;
        return Ok(List(val));
    }
}

#[async_trait] // VERY IMPORTANT !!
impl KeyList for StorageClient {
    async fn list_get(&self, key: &str) -> TribResult<List> {
        let mut client = TribStorageClient::connect(self.addr.clone()).await?;
        let r = client
            .list_get(Key {
                key: key.to_string(),
            })
            .await?;
        match r.into_inner().list {
            value => Ok(List(value)),
        }
    }

    /// Append a string to the list. return true when no error.
    async fn list_append(&self, kv: &KeyValue) -> TribResult<bool> {
        let mut client = TribStorageClient::connect(self.addr.clone()).await?;
        let r = client
            .list_append(rpc::KeyValue {
                key: kv.key.clone(),
                value: kv.value.clone(),
            })
            .await?;
        return Ok(r.into_inner().value);
    }

    /// Removes all elements that are equal to `kv.value` in list `kv.key`
    /// returns the number of elements removed.
    async fn list_remove(&self, kv: &KeyValue) -> TribResult<u32> {
        let mut client = TribStorageClient::connect(self.addr.clone()).await?;
        let r = client
            .list_remove(rpc::KeyValue {
                key: kv.key.clone(),
                value: kv.value.clone(),
            })
            .await?;
        return Ok(r.into_inner().removed);
    }

    /// List all the keys of non-empty lists, where the key matches
    /// the given pattern.
    async fn list_keys(&self, p: &Pattern) -> TribResult<List> {
        let mut client = TribStorageClient::connect(self.addr.clone()).await?;
        let r = client
            .list_keys(rpc::Pattern {
                prefix: p.prefix.clone(),
                suffix: p.suffix.clone(),
            })
            .await?;
        return Ok(List(r.into_inner().list));
    }
}

#[async_trait] // VERY IMPORTANT !!
impl Storage for StorageClient {
    /// Returns an auto-incrementing clock. The returned value of each call will
    /// be unique, no smaller than `at_least`, and strictly larger than the
    /// value returned last time, unless it was [u64::MAX]
    async fn clock(&self, at_least: u64) -> TribResult<u64> {
        let mut client = TribStorageClient::connect(self.addr.clone()).await?;
        let r = client
            .clock(Clock {
                timestamp: at_least,
            })
            .await?;
        return Ok(r.into_inner().timestamp);
    }
}
