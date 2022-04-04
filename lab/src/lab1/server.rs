#[derive(Default)]
pub struct StorageServer {}

use tonic::{Request, Response, Status};
use tribbler::rpc;
use tribbler::rpc::{Bool, Clock, Key, KeyValue, ListRemoveResponse, Pattern, StringList, Value};

#[async_trait::async_trait]
impl rpc::trib_storage_server::TribStorage for StorageServer {
    async fn get(&self, request: Request<Key>) -> Result<Response<Value>, Status> {
        todo!()
    }

    async fn set(
        &self,
        request: tonic::Request<rpc::KeyValue>,
    ) -> Result<tonic::Response<rpc::Bool>, tonic::Status> {
        todo!();
    }

    async fn keys(&self, request: Request<Pattern>) -> Result<Response<StringList>, Status> {
        todo!()
    }

    async fn list_get(&self, request: Request<Key>) -> Result<Response<StringList>, Status> {
        todo!()
    }

    async fn list_append(&self, request: Request<KeyValue>) -> Result<Response<Bool>, Status> {
        todo!()
    }

    async fn list_remove(
        &self,
        request: Request<KeyValue>,
    ) -> Result<Response<ListRemoveResponse>, Status> {
        todo!()
    }

    async fn list_keys(&self, request: Request<Pattern>) -> Result<Response<StringList>, Status> {
        todo!()
    }

    async fn clock(&self, request: Request<Clock>) -> Result<Response<Clock>, Status> {
        todo!()
    }
}
