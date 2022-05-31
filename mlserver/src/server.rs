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

impl MLServer {
    pub fn pt(self: &Self, x: usize, y: usize, z: usize) -> usize {
        match x {
            0 if z == 0 && y < 20 => y,
            1 if y == 0 && z < 20 => 20 + z,
            _ => panic!("Invalid location: {}, {}, {}", x, y, z),
        }
    }
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
            .write()
            .map_err(|e| e.to_string())
            .unwrap();
        let pos = updater_queue.iter().position(|x| x.clock == request.clock);
        return match pos {
            None => Err(Status::new(Code::Internal, "Not found in queue")),
            Some(pos) => {
                if updater_queue.len() > 4 {
                    while updater_queue.index(pos - 4).done == false {}
                }
                let mut ws_map = self.ws1.write().map_err(|e| e.to_string()).unwrap();
                let mut bs_map = self.bs1.write().map_err(|e| e.to_string()).unwrap();
                let mut ws1 = ws_map.get(&*request.model_name).unwrap().clone();
                let mut bs1 = bs_map.get(&*request.model_name).unwrap().clone();

                //Updating Weights - Backward Propagation
                // println!("UPDATING WEIGHTS");
                for i in 0..20 {
                    for pt in &[self.pt(1, 0, i), self.pt(0, i, 0)] {
                        ws1[*pt] -= request.ws1[*pt] * self.lr;
                        bs1[*pt] -= request.bs1[*pt] * self.lr;
                    }
                }

                ws_map.insert(request.model_name.clone(), ws1.to_vec());
                bs_map.insert(request.model_name.clone(), bs1.to_vec());
                let mut value = &mut updater_queue[pos];
                value.done = true;
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
