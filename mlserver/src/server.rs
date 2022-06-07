use crate::err::TribResult;
use crate::ml;
use crate::ml::MLModel;
use crate::rpc::parameter_server_server::ParameterServer;
use crate::rpc::{Clock, DoubleList, EmptyRequest, ModelPull};
use async_trait::async_trait;
use std::cmp::min;
use std::collections::{HashMap, LinkedList};
use std::ops::Index;
use std::sync::{Arc, RwLock};
use tonic::{Code, Request, Response, Status};

pub struct MLServer {
    pub addr: String,
    pub ml_model: Box<dyn MLModel>,
}

#[async_trait]
impl ParameterServer for MLServer {
    async fn pull(&self, request: Request<ModelPull>) -> Result<Response<DoubleList>, Status> {
        let request = request.into_inner();
        let model_name = request.name;
        let clock = request.clock;
        let pulled_model = self
            .ml_model
            .pull(ml::ModelPull {
                name: model_name.to_string(),
                clock: clock.clone(),
            })
            .await;
        match pulled_model {
            Ok(result) => Ok(Response::new(DoubleList {
                clock: clock.clone(),
                model_name: model_name.clone(),
                ws1: result.ws1.clone(),
                bs1: result.bs1.clone(),
            })),
            Err(_) => return Err(Status::new(Code::Internal, "Error occurred")),
        }
    }

    async fn push(&self, request: Request<DoubleList>) -> Result<Response<EmptyRequest>, Status> {
        let request = request.into_inner();

        let push_result = self
            .ml_model
            .push(ml::DoubleList {
                clock: request.clock,
                model_name: request.model_name,
                ws1: request.ws1,
                bs1: request.bs1,
            })
            .await;
        Ok(Response::new(EmptyRequest { empty: true }))
    }

    async fn clock(&self, request: Request<Clock>) -> Result<Response<Clock>, Status> {
        let clock_result_match = self.ml_model.clock(request.into_inner().clock).await;
        let clock_result = match clock_result_match {
            Ok(inner) => inner,
            Err(error) => return Err(Status::new(Code::Internal, "Error occurred")),
        };
        Ok(Response::new(Clock {
            clock: clock_result,
        }))
    }
}
