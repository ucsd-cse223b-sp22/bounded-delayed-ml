use rand::thread_rng;
use rand::{distributions::Standard, Rng};
use std::cmp::min;
use tonic::transport::Channel;
use mlserver::client::ParameterClient;
use mlserver::ml::MLModel;
use mlserver::rpc::{Clock, DoubleList, ModelPull};
use mlserver::rpc::parameter_server_client::ParameterServerClient;
use mlserver::storage::BinStorage;

/// Data structure to hold the net

pub struct MLWorker {
    pub client: Box<dyn MLModel>,
}

pub struct Net {
    ws: Vec<f64>,
    bs: Vec<f64>,
    ns: usize,
    clock_id: u64,
}

impl Net {
    /// Create a fully-connected net with hidden layer size
    pub async fn new(ns: usize, addr: String) -> Net {
        let mut worker_conn = ParameterServerClient::connect(addr).await.unwrap();
        let worker_client = ParameterClient {
            client: worker_conn
        };
        //TODO:: Get model name from queue recursively for # of epochs
        let clock_response = worker_client.clock(0).await.unwrap();
        let response = worker_client.pull(mlserver::ml::ModelPull {
            name: "model1".to_string(),
            clock: clock_response,
        }).await.unwrap();
        let ws: Vec<f64> = response.ws1;
        let bs: Vec<f64> = response.bs1;
        Net { ws, bs, ns, clock_id: clock_response }
    }

    /// Calculates an index into the weights/biases vector
    /// for a given net
    pub fn pt(self: &Self, x: usize, y: usize, z: usize) -> usize {
        match x {
            0 if z == 0 && y < self.ns => y,
            1 if y == 0 && z < self.ns => self.ns + z,
            _ => panic!("Invalid location: {}, {}, {}", x, y, z),
        }
    }

    pub fn cost(self: &Self, data: &[(f64, f64)]) -> f64 {
        let mut loss = 0.0;
        for (x, y) in data {
            let val = self.eval(*x);
            loss += (y - val).powi(2);
        }
        loss / self.ns as f64
    }

    pub async fn backprop(self: &mut Self, data: &[(f64, f64)], addr: String) {
        let mut worker_conn = ParameterServerClient::connect(addr).await.unwrap();
        let worker_client = ParameterClient {
            client: worker_conn
        };
        let mut dws: Vec<f64> = vec![0.0; self.ns * 2];
        let mut dbs: Vec<f64> = vec![0.0; self.ns * 2];

        for i in 0..self.ns {
            let pt1 = self.pt(0, i, 0);
            let pt2 = self.pt(1, 0, i);

            for (x, y) in data {
                let yy = self.eval(*x);

                dws[pt2] += -2.0 * (y - yy) * relu_ish(self.rwxb(*x, 0, i, 0), yy);
                dbs[pt2] += -2.0 * (y - yy) * relu_ish(1.0, yy);

                dws[pt1] += -2.0
                    * (y - yy)
                    * relu_ish(self.ws[pt2] * relu_ish(*x, self.wxb(*x, 0, i, 0)), yy);
                dbs[pt1] += -2.0
                    * (y - yy)
                    * relu_ish(self.ws[pt2] * relu_ish(1.0, self.wxb(*x, 0, i, 0)), yy);
            }
        }

        // println!("DWS {:?}", dws);
        // println!("DBS {:?}", dbs);
        let push_result = worker_client.push(mlserver::ml::DoubleList {
            clock: self.clock_id,
            model_name: "model1".to_string(),
            ws1: dws,
            bs1: dbs,
        }).await;
        let x = 5;
    }

    pub fn eval(self: &Self, val: f64) -> f64 {
        relu(
            (0..self.ns)
                .map(|i| self.rwxb(self.rwxb(val, 0, i, 0), 1, 0, i))
                .sum(),
        )
    }

    /// Relu(wx + b) for coordinates x, y, z with input val
    fn rwxb(self: &Self, val: f64, x: usize, y: usize, z: usize) -> f64 {
        relu(self.wxb(val, x, y, z))
    }

    /// wx + b for coordinates x, y, z with input val
    fn wxb(self: &Self, val: f64, x: usize, y: usize, z: usize) -> f64 {
        self.w(x, y, z) * val + self.b(x, y, z)
    }

    fn w(self: &Self, x: usize, y: usize, z: usize) -> f64 {
        self.ws[self.pt(x, y, z)]
    }

    fn b(self: &Self, x: usize, y: usize, z: usize) -> f64 {
        self.bs[self.pt(x, y, z)]
    }
}

/// Leaky relu
fn relu(v: f64) -> f64 {
    relu_ish(v, v)
}

/// Leaky relu based on another variable, useful for derivatives
fn relu_ish(v: f64, point: f64) -> f64 {
    if point >= 0.0 {
        v
    } else {
        0.01 * v
    }
}



