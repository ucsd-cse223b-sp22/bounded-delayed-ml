use crate::rpc::parameter_server_server::ParameterServer;
use crate::rpc::{Clock, DoubleList, EmptyRequest, ModelPull};
use async_trait::async_trait;
use std::cmp::min;
use std::collections::{HashMap, LinkedList};
use std::ops::Index;
use std::sync::{Arc, RwLock};
use tonic::{Code, Request, Response, Status};

pub struct WorkerStatus {
    pub clock: u64,
    pub done: bool,
}

pub struct MLServer {
    pub addr: String,
    pub ws1: RwLock<HashMap<String, Vec<f64>>>,
    pub bs1: RwLock<HashMap<String, Vec<f64>>>,
    pub updater_queue: RwLock<Vec<WorkerStatus>>,
    pub lr: f64,
    pub clock: RwLock<u64>,
}

#[async_trait]
impl ParameterServer for MLServer {
    async fn pull(&self, request: Request<ModelPull>) -> Result<Response<DoubleList>, Status> {
        let request = request.into_inner();
        let model_name = request.name;
        let clock = request.clock;
        let ws_map = self.ws1.read().map_err(|e| e.to_string()).unwrap();
        let ws1 = ws_map.get(&*model_name).unwrap();

        let bs_map = self.bs1.read().map_err(|e| e.to_string()).unwrap();
        let bs1 = bs_map.get(&*model_name).unwrap();
        let mut updater_queue = self
            .updater_queue
            .write()
            .map_err(|e| e.to_string())
            .unwrap();
        updater_queue.push(WorkerStatus { clock, done: false });
        Ok(Response::new(DoubleList {
            clock,
            model_name,
            ws1: ws1.clone(),
            bs1: bs1.clone(),
        }))
    }

    async fn push(&self, request: Request<DoubleList>) -> Result<Response<EmptyRequest>, Status> {
        let request = request.into_inner();
        let mut updater_queue = self
            .updater_queue
            .read()
            .map_err(|e| e.to_string())
            .unwrap();
        let pos = updater_queue.iter().position(|x| x.clock == request.clock);
        return match pos {
            None => Err(Status::new(Code::Internal, "Not found in queue")),
            Some(pos) => {
                while updater_queue.index(min(pos - 4, 0)).done == false {}
                let mut ws_map = self.ws1.write().map_err(|e| e.to_string()).unwrap();
                let mut bs_map = self.bs1.write().map_err(|e| e.to_string()).unwrap();
                let mut ws1 = ws_map.get(&*request.model_name).unwrap();
                let bs1 = bs_map.get(&*request.model_name).unwrap();

                //Updating Weights - Backward Propagation
                let ws1_updated: Vec<f64> = (0..ws1.len())
                    .map(|i| ws1[i] - self.lr * request.ws1[i])
                    .collect();
                let bs1_updated: Vec<f64> = (0..bs1.len())
                    .map(|i| bs1[i] - self.lr * request.bs1[i])
                    .collect();
                ws_map.insert(request.model_name.clone(), ws1_updated);
                bs_map.insert(request.model_name.clone(), bs1_updated);
                Ok(Response::new(EmptyRequest { empty: true }))
            }
        };
    }

    async fn clock(&self, request: Request<Clock>) -> Result<Response<Clock>, Status> {
        let mut clk = self.clock.write().map_err(|e| e.to_string()).unwrap();
        let at_least = request.into_inner().clock;
        if *clk < at_least {
            *clk = at_least
        }
        let ret = *clk;
        if *clk < u64::MAX {
            *clk += 1;
        }
        Ok(Response::new(Clock { clock: ret }))
    }
}
