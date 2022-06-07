use crate::err::TribResult;
use crate::ml::MLModel;
use crate::rpc::parameter_server_server::ParameterServer;
use crate::rpc::{Clock, DoubleList, EmptyRequest, ModelPull};
use tonic::{Request, Response, Status};

pub struct MyParameterServerBin {
    pub name: String,
    pub client: Box<dyn MLModel>,
    pub replica_client: Box<dyn MLModel>,
    pub no_replica: bool,
}

#[async_trait::async_trait]
impl MLModel for MyParameterServerBin {
    async fn pull(&self, model_pull: crate::ml::ModelPull) -> TribResult<crate::ml::DoubleList> {
        Ok(self.client.pull(model_pull).await?)
    }

    async fn push(&self, double_list: crate::ml::DoubleList) -> TribResult<bool> {
        Ok(self.client.push(double_list).await?)
    }

    async fn clock(&self, at_least: u64) -> TribResult<u64> {
        Ok(self.client.clock(at_least).await?)
    }
}
