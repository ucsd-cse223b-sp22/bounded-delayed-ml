use tonic::{Code, Request, Response, Status};

use tribbler::rpc;
use tribbler::rpc::trib_storage_server::TribStorage;
use tribbler::rpc::{Bool, Clock, Key, KeyValue, ListRemoveResponse, Pattern, StringList, Value};
use tribbler::storage::Storage;

pub struct StorageServer {
    pub mem_storage: Box<dyn Storage>,
}

#[async_trait::async_trait]
impl TribStorage for StorageServer {
    async fn get(&self, request: Request<Key>) -> Result<Response<Value>, Status> {
        let get_result_match = self.mem_storage.get(&request.into_inner().key).await;
        let get_result = match get_result_match {
            Ok(inner) => match inner {
                Some(value) => value,
                None => return Err(Status::new(Code::NotFound, "Key not found")),
            },
            Err(error) => return Err(Status::new(Code::Internal, "Error occurred")),
        };
        Ok(Response::new(Value { value: get_result }))
    }

    async fn set(
        &self,
        request: tonic::Request<rpc::KeyValue>,
    ) -> Result<tonic::Response<rpc::Bool>, tonic::Status> {
        let value = request.into_inner();
        let set_result_match = self
            .mem_storage
            .set(&tribbler::storage::KeyValue {
                key: value.key,
                value: value.value,
            })
            .await;
        let set_result = match set_result_match {
            Ok(inner) => inner,
            Err(error) => false,
        };
        Ok(Response::new(Bool { value: set_result }))
    }

    async fn keys(&self, request: Request<Pattern>) -> Result<Response<StringList>, Status> {
        let pattern = request.into_inner();
        let keys_result_match = self
            .mem_storage
            .keys(&tribbler::storage::Pattern {
                prefix: pattern.prefix,
                suffix: pattern.suffix,
            })
            .await;
        let keys_result = match keys_result_match {
            Ok(inner) => inner,
            Err(error) => return Err(Status::new(Code::Internal, "Error occurred")),
        };
        Ok(Response::new(StringList {
            list: keys_result.0,
        }))
    }

    async fn list_get(&self, request: Request<Key>) -> Result<Response<StringList>, Status> {
        let list_get_result_match = self.mem_storage.list_get(&*request.into_inner().key).await;
        let list_get_result = match list_get_result_match {
            Ok(inner) => inner,
            Err(error) => return Err(Status::new(Code::Internal, "Error occurred")),
        };
        Ok(Response::new(StringList {
            list: list_get_result.0,
        }))
    }

    async fn list_append(&self, request: Request<KeyValue>) -> Result<Response<Bool>, Status> {
        let kv = request.into_inner();
        let list_append_result_match = self
            .mem_storage
            .list_append(&tribbler::storage::KeyValue {
                key: kv.key,
                value: kv.value,
            })
            .await;
        let list_append_result = match list_append_result_match {
            Ok(inner) => inner,
            Err(error) => return Err(Status::new(Code::Internal, "Error occurred")),
        };
        Ok(Response::new(Bool {
            value: list_append_result,
        }))
    }

    async fn list_remove(
        &self,
        request: Request<KeyValue>,
    ) -> Result<Response<ListRemoveResponse>, Status> {
        let kv = request.into_inner();
        let list_remove_result_match = self
            .mem_storage
            .list_remove(&tribbler::storage::KeyValue {
                key: kv.key,
                value: kv.value,
            })
            .await;
        let list_remove_result = match list_remove_result_match {
            Ok(inner) => inner,
            Err(error) => return Err(Status::new(Code::Internal, "Error occurred")),
        };
        Ok(Response::new(ListRemoveResponse {
            removed: list_remove_result,
        }))
    }

    async fn list_keys(&self, request: Request<Pattern>) -> Result<Response<StringList>, Status> {
        let pattern = request.into_inner();
        let list_keys_result_match = self
            .mem_storage
            .list_keys(&tribbler::storage::Pattern {
                suffix: pattern.suffix,
                prefix: pattern.prefix,
            })
            .await;
        let list_keys_result = match list_keys_result_match {
            Ok(inner) => inner,
            Err(error) => return Err(Status::new(Code::Internal, "Error occurred")),
        };
        Ok(Response::new(StringList {
            list: list_keys_result.0,
        }))
    }

    async fn clock(&self, request: Request<Clock>) -> Result<Response<Clock>, Status> {
        let clock_result_match = self.mem_storage.clock(request.into_inner().timestamp).await;
        let clock_result = match clock_result_match {
            Ok(inner) => inner,
            Err(error) => return Err(Status::new(Code::Internal, "Error occurred")),
        };
        Ok(Response::new(Clock {
            timestamp: clock_result,
        }))
    }
}
