use crate::err::TribResult;
use crate::ml::MLModel;
use crate::rpc::parameter_server_server::ParameterServer;
use crate::rpc::{Clock, DoubleList, EmptyRequest, ModelPull, WeightsPair, WorkerStatus};
use crate::{ml, rpc};
use async_trait::async_trait;
use std::cmp::min;
use std::collections::{HashMap, LinkedList};
use std::ops::Index;
use std::ptr::hash;
use std::sync::{Arc, RwLock};
use tonic::{Code, Request, Response, Status};

pub struct MLServer {
    pub addr: String,
    pub ml_model: Box<dyn MLModel>,
}

#[async_trait]
impl ParameterServer for MLServer {
    async fn initialize(
        &self,
        request: Request<DoubleList>,
    ) -> Result<Response<EmptyRequest>, Status> {
        let request_val = request.into_inner();
        let result = self
            .ml_model
            .initialize(ml::DoubleList {
                clock: 0,
                model_name: request_val.model_name.to_string(),
                ws1: request_val.ws1.clone(),
                bs1: request_val.bs1.clone(),
            })
            .await;
        Ok(Response::new(EmptyRequest { empty: true }))
    }

    async fn set_ready(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<EmptyRequest>, Status> {
        let request = request.into_inner();
        let ready_val = self
            .ml_model
            .set_ready(ml::EmptyRequest {
                empty: request.empty,
            })
            .await;
        match ready_val {
            Ok(_) => Ok(Response::new(EmptyRequest { empty: true })),
            Err(_) => return Err(Status::new(Code::Internal, "Error occurred")),
        }
    }

    async fn get_ready(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<EmptyRequest>, Status> {
        let request = request.into_inner();
        let ready_val = self
            .ml_model
            .get_ready(ml::EmptyRequest { empty: true })
            .await;
        match ready_val {
            Ok(result) => Ok(Response::new(EmptyRequest {
                empty: result.empty,
            })),
            Err(_) => return Err(Status::new(Code::Internal, "Error occurred")),
        }
    }

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

    async fn get_model_dump(
        &self,
        request: Request<()>,
    ) -> Result<Response<rpc::ModelDump>, Status> {
        let model_dump = self.ml_model.get_model_dump().await;
        match model_dump {
            Ok(model) => {
                // Serialize ModelDump
                let hashmap_ws1 = model.ws1;
                let hashmap_bs1 = model.bs1;
                let hashmap_queue = model.updater_queue;
                let mut vec_pair_ws1 = Vec::new();
                let mut vec_pair_bs1 = Vec::new();
                let mut vec_pair_queue = Vec::new();
                for (k, val) in hashmap_ws1.into_iter() {
                    vec_pair_ws1.push(WeightsPair { key: k, value: val })
                }
                for (k, val) in hashmap_bs1.into_iter() {
                    vec_pair_bs1.push(WeightsPair { key: k, value: val })
                }
                for val in hashmap_queue.into_iter() {
                    vec_pair_queue.push(WorkerStatus {
                        clock: val.clock,
                        done: val.done,
                    })
                }
                Ok(Response::new(rpc::ModelDump {
                    updater_queue: vec_pair_queue,
                    ws1: vec_pair_ws1,
                    bs1: vec_pair_bs1,
                    lr: model.lr,
                }))
            }
            Err(_) => return Err(Status::new(Code::Internal, "Error occurred")),
        }
    }

    async fn merge_model_dump(
        &self,
        request: Request<rpc::ModelDump>,
    ) -> Result<Response<EmptyRequest>, Status> {
        let request = request.into_inner();

        // Deserialize
        let vec_pair_ws1 = request.ws1;
        let vec_pair_bs1 = request.bs1;
        let vec_pair_queue = request.updater_queue;
        let mut hashmap_ws1 = HashMap::new();
        let mut hashmap_bs1 = HashMap::new();
        let mut hashmap_queue = Vec::new(); // FIXME: Change
        for val in vec_pair_ws1.into_iter() {
            hashmap_ws1.insert(val.key, val.value);
        }
        for val in vec_pair_bs1.into_iter() {
            hashmap_bs1.insert(val.key, val.value);
        }
        for val in vec_pair_queue.into_iter() {
            hashmap_queue.push(ml::WorkerStatus {
                clock: val.clock,
                done: val.done,
            });
        }
        let _ = self
            .ml_model
            .merge_model_dump(ml::ModelDump {
                updater_queue: hashmap_queue,
                ws1: hashmap_ws1,
                bs1: hashmap_bs1,
                lr: request.lr,
            })
            .await;
        Ok(Response::new(EmptyRequest { empty: true }))
    }
}
