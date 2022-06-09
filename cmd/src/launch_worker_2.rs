use std::cmp::min;
use mlserver::rpc::parameter_server_client::ParameterServerClient;
use std::time::Instant;
use cmd::backs::BACKS;
use cmd::ml_forward::Net;
use mlserver::err::TribResult;
use mlserver::serve::new_bin_client;

#[tokio::main]
async fn main() -> TribResult<()> {
    let backs = BACKS.map(|x| x.to_string()).to_vec();

    fn original_fn(x: f64) -> f64 {
        x * x * x + x * x + x
    }
    ;

    let training_data: Vec<(f64, f64)> = (1..=100)
        .step_by(7)
        .map(|x| (x as f64) / 100.0)
        .map(|x| (x, original_fn(x)))
        .collect();

    let validation_data: Vec<(f64, f64)> = (20..=60)
        .map(|x| (x as f64) / 100.0)
        .map(|x| (x, original_fn(x)))
        .collect();

    let epochs = 1000;
    let batch_size = 5;
    let start = Instant::now();

    //TODO:: Set point for each worker differently
    let mut point = 0;

    let log_interval = epochs / 100;
    //TODO:: Take this from queue and execute backprop for each model individually
    let mut net = Net::new("model2", 20, backs.clone()).await?;

    for epoch in 0..epochs {
        let limit = min(point + batch_size, training_data.len());
        net.backprop(&training_data[point..limit]).await?;
        if log_interval > 0 && epoch % log_interval == 0 {
            eprintln!("Epoch {}: {}", epoch, net.cost(&training_data.clone()));
        }
        net = Net::new("model2", 20, backs.clone()).await?;
    }


    eprintln!("Training duration: {}s", start.elapsed().as_secs());
    eprintln!("Validation error: {}", net.cost(&validation_data));
    for (x, y) in &training_data[point..batch_size] {
        println!("{}\t{}\t{}", x, original_fn(*x), net.eval(*x));
    }
    Ok(())
}