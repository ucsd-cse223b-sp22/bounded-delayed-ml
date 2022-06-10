use crate::err::TribResult;
use crate::ml::{MLModel, ModelDump};
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
    async fn initialize(
        &self,
        double_list: crate::ml::DoubleList,
    ) -> TribResult<crate::ml::EmptyRequest> {
        Ok(self.client.initialize(double_list).await?)
    }

    async fn get_ready(
        &self,
        ready_value: crate::ml::EmptyRequest,
    ) -> TribResult<crate::ml::EmptyRequest> {
        Ok(self.client.get_ready(ready_value).await?)
    }

    async fn set_ready(
        &self,
        ready_value: crate::ml::EmptyRequest,
    ) -> TribResult<crate::ml::EmptyRequest> {
        Ok(self.client.set_ready(ready_value).await?)
    }

    async fn pull(&self, model_pull: crate::ml::ModelPull) -> TribResult<crate::ml::DoubleList> {
        Ok(self.client.pull(model_pull).await?)
    }

    async fn push(&self, double_list: crate::ml::DoubleList) -> TribResult<bool> {
        Ok(self.client.push(double_list).await?)
    }

    async fn clock(&self, at_least: u64) -> TribResult<u64> {
        Ok(self.client.clock(at_least).await?)
    }

    async fn get_model_dump(&self) -> TribResult<ModelDump> {
        Ok(self.client.get_model_dump().await?)
    }

    async fn merge_model_dump(&self, model_dump: ModelDump) -> TribResult<()> {
        Ok(self.client.merge_model_dump(model_dump).await?)
    }
}
