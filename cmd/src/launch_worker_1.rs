use std::cmp::min;
use mlserver::rpc::parameter_server_client::ParameterServerClient;
use std::time::Instant;
use cmd::backs::BACKS;
use cmd::ml_forward::Net;
use mlserver::err::TribResult;
use mlserver::serve::new_bin_client;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> TribResult<()> {

    let backs = BACKS.map(|x| x.to_string()).to_vec();
    fn original_fn(x: f64) -> f64 {
        x * x * x + x * x + x
    }
    ;

    let training_data: Vec<(f64, f64)> = (50..=100)
        .step_by(7)
        .map(|x| (x as f64) / 100.0)
        .map(|x| (x, original_fn(x)))
        .collect();

    let validation_data: Vec<(f64, f64)> = (110..=120)
        .map(|x| (x as f64) / 100.0)
        .map(|x| (x, original_fn(x)))
        .collect();

    let epochs = 1000;
    let batch_size = 5;
    let start = Instant::now();

    //TODO:: Set point for each worker differently
    let mut point = 0;

    let log_interval = epochs / 100;
    let limit = min(point + batch_size, training_data.len());
    //TODO:: Take this from queue and execute backprop for each model individually
    let mut net1 = Net::new("model1",20, backs.clone()).await?;
    net1.backprop(&training_data[point..limit]).await?;

    let mut net2 = Net::new("model2",20, backs.clone()).await?;
    net2.backprop(&training_data[point..limit]).await?;

    for epoch in 1..epochs {

        net1 = Net::new("model1",20, backs.clone()).await?;
        net1.backprop(&training_data[point..limit]).await?;

        net2 = Net::new("model2",20, backs.clone()).await?;
        net2.backprop(&training_data[point..limit]).await?;

        if log_interval > 0 && epoch % log_interval == 0 {
            eprintln!("Net 1, Worker 1 - Epoch {}: {}", epoch, net1.cost(&training_data.clone()));
            eprintln!("Net 2, Worker 1 - Epoch {}: {}", epoch, net2.cost(&training_data.clone()));
        }

    }


    eprintln!("Training duration: {}s", start.elapsed().as_secs());
    eprintln!("Validation error Net 1 Worker 1: {}", net1.cost(&validation_data));
    eprintln!("Validation error Net 2 Worker 1: {}", net2.cost(&validation_data));
    for (x, y) in &training_data[point..batch_size] {
        println!("Net 1 Worker 1: {}\t{}\t{}", x, original_fn(*x), net1.eval(*x));
        println!("Net 2 Worker 1: {}\t{}\t{}", x, original_fn(*x), net2.eval(*x));
    }
    Ok(())
}