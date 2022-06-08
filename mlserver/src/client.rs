use crate::err::TribResult;
use crate::ml;
use crate::ml::MLModel;
use crate::rpc::parameter_server_client::ParameterServerClient;
use crate::rpc::parameter_server_server::ParameterServer;
use crate::rpc::{Clock, DoubleList, EmptyRequest, ModelPull};
use async_trait::async_trait;
use tonic::transport::Channel;
use tonic::{Request, Response, Status};

pub struct ParameterClient {
    pub client: ParameterServerClient<Channel>,
}

#[async_trait] // VERY IMPORTANT !!
impl MLModel for ParameterClient {
    async fn get_ready(&self, ready_value: ml::EmptyRequest) -> TribResult<ml::EmptyRequest> {
        let mut client = self.client.clone();
        let r = client
            .get_ready(EmptyRequest {
                empty: ready_value.empty,
            })
            .await?
            .into_inner();
        Ok(ml::EmptyRequest { empty: r.empty })
    }

    async fn set_ready(&self, ready_value: ml::EmptyRequest) -> TribResult<ml::EmptyRequest> {
        let mut client = self.client.clone();
        let r = client
            .set_ready(EmptyRequest {
                empty: ready_value.empty,
            })
            .await?
            .into_inner();
        Ok(ml::EmptyRequest { empty: true })
    }

    async fn pull(&self, model: ml::ModelPull) -> TribResult<ml::DoubleList> {
        let mut client = self.client.clone();
        let r = client
            .pull(ModelPull {
                name: model.name,
                clock: model.clock,
            })
            .await?
            .into_inner();
        Ok(ml::DoubleList {
            clock: r.clock,
            model_name: r.model_name,
            ws1: r.ws1,
            bs1: r.bs1,
        })
    }

    async fn push(&self, dll: ml::DoubleList) -> TribResult<bool> {
        let mut client = self.client.clone();
        let r = client
            .push(DoubleList {
                clock: dll.clock,
                model_name: dll.model_name,
                ws1: dll.ws1,
                bs1: dll.bs1,
            })
            .await?
            .into_inner();
        Ok(true)
    }

    async fn clock(&self, at_least: u64) -> TribResult<u64> {
        let mut client = self.client.clone();
        let r = client.clock(Clock { clock: at_least }).await?.into_inner();
        Ok(r.clock)
    }
}
