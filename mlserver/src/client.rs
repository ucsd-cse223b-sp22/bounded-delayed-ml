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
impl ParameterServer for ParameterClient {
    async fn pull(&self, request: Request<ModelPull>) -> Result<Response<DoubleList>, Status> {
        let mut client = self.client.clone();
        let r = client.pull(request.into_inner()).await?;
        Ok(r)
    }

    async fn push(&self, request: Request<DoubleList>) -> Result<Response<EmptyRequest>, Status> {
        let mut client = self.client.clone();
        let r = client.push(request.into_inner()).await?;
        Ok(r)
    }

    async fn clock(&self, request: Request<Clock>) -> Result<Response<Clock>, Status> {
        let mut client = self.client.clone();
        let r = client.clock(request.into_inner()).await?;
        Ok(r)
    }
}
