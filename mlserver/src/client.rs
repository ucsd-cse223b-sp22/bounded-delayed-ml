use crate::err::TribResult;
use crate::ml::{MLModel, WorkerStatus};
use crate::rpc::parameter_server_client::ParameterServerClient;
use crate::rpc::parameter_server_server::ParameterServer;
use crate::rpc::{Clock, DoubleList, EmptyRequest, ModelPull, WeightsPair};
use crate::{ml, rpc};
use async_trait::async_trait;
use std::collections::HashMap;
use tonic::transport::Channel;
use tonic::{Request, Response, Status};

pub struct ParameterClient {
    pub client: ParameterServerClient<Channel>,
}

#[async_trait] // VERY IMPORTANT !!
impl MLModel for ParameterClient {
    async fn initialize(&self, dll: ml::DoubleList) -> TribResult<ml::EmptyRequest> {
        let mut client = self.client.clone();
        let r = client
            .initialize(DoubleList {
                clock: dll.clock,
                model_name: dll.model_name,
                ws1: dll.ws1,
                bs1: dll.bs1,
            })
            .await?
            .into_inner();
        Ok(ml::EmptyRequest { empty: r.empty })
    }

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

    async fn get_model_dump(&self) -> TribResult<ml::ModelDump> {
        let mut client = self.client.clone();
        let storage_dump = client.get_model_dump(()).await?.into_inner();
        // Deserialize
        let vec_pair_ws1 = storage_dump.ws1;
        let vec_pair_bs1 = storage_dump.bs1;
        let vec_pair_queue = storage_dump.updater_queue;
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
            hashmap_queue.push(WorkerStatus {
                clock: val.clock,
                done: val.done,
            });
        }
        Ok(ml::ModelDump {
            updater_queue: hashmap_queue,
            ws1: hashmap_ws1,
            bs1: hashmap_bs1,
            lr: storage_dump.lr, // FIXME: Change
        })
    }

    async fn merge_model_dump(&self, model_dump: ml::ModelDump) -> TribResult<()> {
        let mut client = self.client.clone();
        // Serialize
        let hashmap_ws1 = model_dump.ws1;
        let hashmap_bs1 = model_dump.bs1;
        let hashmap_queue = model_dump.updater_queue;
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
            vec_pair_queue.push(rpc::WorkerStatus {
                clock: val.clock,
                done: val.done,
            })
        }
        let model_dump = rpc::ModelDump {
            updater_queue: vec_pair_queue,
            ws1: vec_pair_ws1,
            bs1: vec_pair_bs1,
            lr: model_dump.lr,
        };
        let _ = client.merge_model_dump(model_dump).await;
        Ok(())
    }
}
