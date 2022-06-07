use crate::err::{TribResult, TribblerError};
use async_trait::async_trait;
use std::collections::HashMap;
use std::ops::Index;
use std::sync::RwLock;
use tonic::{Code, Response, Status};

pub struct WorkerStatus {
    pub clock: u64,
    pub done: bool,
}

pub struct ModelPull {
    pub name: String,
    pub clock: u64,
}

pub struct DoubleList {
    pub clock: u64,
    pub model_name: String,
    pub ws1: Vec<f64>,
    pub bs1: Vec<f64>,
}

#[async_trait]
pub trait MLModel: Send + Sync {
    /// Returns an auto-incrementing clock. The returned value of each call will
    /// be unique, no smaller than `at_least`, and strictly larger than the
    /// value returned last time, unless it was [u64::MAX]

    async fn pull(&self, model_pull: ModelPull) -> TribResult<DoubleList>;
    async fn push(&self, double_list: DoubleList) -> TribResult<bool>;
    async fn clock(&self, at_least: u64) -> TribResult<u64>;
}

pub struct MLStorage {
    pub updater_queue: RwLock<Vec<WorkerStatus>>,
    pub ws1: RwLock<HashMap<String, Vec<f64>>>,
    pub bs1: RwLock<HashMap<String, Vec<f64>>>,
    pub lr: f64,
    pub clock: RwLock<u64>,
}

impl MLStorage {
    pub fn pt(self: &Self, x: usize, y: usize, z: usize) -> usize {
        match x {
            0 if z == 0 && y < 20 => y,
            1 if y == 0 && z < 20 => 20 + z,
            _ => panic!("Invalid location: {}, {}, {}", x, y, z),
        }
    }
}

#[async_trait]
impl MLModel for MLStorage {
    async fn pull(&self, model_pull: ModelPull) -> TribResult<DoubleList> {
        let ws_map = self.ws1.read().map_err(|e| e.to_string()).unwrap();
        let ws1 = ws_map.get(&*model_pull.name).unwrap();

        let bs_map = self.bs1.read().map_err(|e| e.to_string()).unwrap();
        let bs1 = bs_map.get(&*model_pull.name).unwrap();
        let mut updater_queue = self
            .updater_queue
            .write()
            .map_err(|e| e.to_string())
            .unwrap();
        updater_queue.push(WorkerStatus {
            clock: model_pull.clock,
            done: false,
        });
        Ok(DoubleList {
            clock: model_pull.clock.clone(),
            model_name: model_pull.name.clone(),
            ws1: ws1.clone(),
            bs1: bs1.clone(),
        })
    }

    async fn push(&self, double_list: DoubleList) -> TribResult<bool> {
        let mut updater_queue = self
            .updater_queue
            .write()
            .map_err(|e| e.to_string())
            .unwrap();
        let pos = updater_queue
            .iter()
            .position(|x| x.clock == double_list.clock);
        return match pos {
            None => Err(Box::new(TribblerError::RpcError("Error".to_string()))),
            Some(pos) => {
                if updater_queue.len() > 4 {
                    while updater_queue.index(pos - 4).done == false {}
                }
                let mut ws_map = self.ws1.write().map_err(|e| e.to_string()).unwrap();
                let mut bs_map = self.bs1.write().map_err(|e| e.to_string()).unwrap();
                let model_name = double_list.model_name;
                let mut ws1 = ws_map.get(&*model_name).unwrap().clone();
                let mut bs1 = bs_map.get(&*model_name).unwrap().clone();

                //Updating Weights - Backward Propagation
                // println!("UPDATING WEIGHTS");
                for i in 0..20 {
                    for pt in &[self.pt(1, 0, i), self.pt(0, i, 0)] {
                        ws1[*pt] -= double_list.ws1[*pt] * self.lr;
                        bs1[*pt] -= double_list.bs1[*pt] * self.lr;
                    }
                }

                ws_map.insert(model_name.clone(), ws1.to_vec());
                bs_map.insert(model_name.clone(), bs1.to_vec());
                let mut value = &mut updater_queue[pos];
                value.done = true;
                updater_queue.insert(
                    pos,
                    WorkerStatus {
                        clock: double_list.clock,
                        done: true,
                    },
                );
                Ok(true)
            }
        };
    }

    async fn clock(&self, at_least: u64) -> TribResult<u64> {
        let mut clk = self.clock.write().map_err(|e| e.to_string())?;
        if *clk < at_least {
            *clk = at_least
        }

        let ret = *clk;

        if *clk < u64::MAX {
            *clk += 1;
        }
        Ok(ret)
    }
}
