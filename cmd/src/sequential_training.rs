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
        x*x*x + x*x + x
    }
    ;

    let training_data: Vec<(f64, f64)> = (1..=100)
        // .step_by(7)
        .map(|x| (x as f64) / 100.0)
        .map(|x| (x, original_fn(x)))
        .collect();

    let validation_data: Vec<(f64, f64)> = (100..=120)
        .map(|x| (x as f64) / 100.0)
        .map(|x| (x, original_fn(x)))
        .collect();

    let epochs = 100000;
    let batch_size = 5;
    let start = Instant::now();

    //TODO:: Set point for each worker differently
    let mut point = 0;
    let limit = min(point + batch_size, training_data.len());

    let log_interval = epochs / 100;
    //TODO:: Take this from queue and execute backprop for each model individually
    let mut net1 = Net::new("model1",20, backs.clone()).await?;
    net1.backprop(&training_data[point..limit]).await?;
    
    for epoch in 1..epochs {

        net1 = Net::new("model1",20, backs.clone()).await?;
        net1.backprop(&training_data[point..limit]).await?;
        if log_interval > 0 && epoch % log_interval == 0 {
            eprintln!("Net 1, Worker 1 - Epoch {}: {}", epoch, net1.cost(&training_data.clone()));
        }

    }

    let mut net2 = Net::new("model2",20, backs.clone()).await?;
    net2.backprop(&training_data[point..limit]).await?;

    for epoch in 1..epochs {

        net2 = Net::new("model2",20, backs.clone()).await?;
        net2.backprop(&training_data[point..limit]).await?;

        if log_interval > 0 && epoch % log_interval == 0 {
            eprintln!("Net 2, Worker 1 - Epoch {}: {}", epoch, net2.cost(&training_data.clone()));
        }

    }

    let mut net3 = Net::new("model3",20, backs.clone()).await?;
    net3.backprop(&training_data[point..limit]).await?;

    for epoch in 1..epochs {
        net3 = Net::new("model3",20, backs.clone()).await?;
        net3.backprop(&training_data[point..limit]).await?;

        if log_interval > 0 && epoch % log_interval == 0 {
            eprintln!("Net 3, Worker 1 - Epoch {}: {}", epoch, net3.cost(&training_data.clone()));
        }
    }

    let mut net4 = Net::new("model4",20, backs.clone()).await?;
    net4.backprop(&training_data[point..limit]).await?;

    for epoch in 1..epochs {

        net4 = Net::new("model4",20, backs.clone()).await?;
        net4.backprop(&training_data[point..limit]).await?;

        if log_interval > 0 && epoch % log_interval == 0 {
            eprintln!("Net 4, Worker 1 - Epoch {}: {}", epoch, net4.cost(&training_data.clone()));
        }

    }


    eprintln!("Training duration: {}s", start.elapsed().as_secs());
    eprintln!("Validation error Net 1 {}", net1.cost(&validation_data));
    eprintln!("Validation error Net 2 {}", net2.cost(&validation_data));
    eprintln!("Validation error Net 3 {}", net3.cost(&validation_data));
    eprintln!("Validation error Net 4 {}", net4.cost(&validation_data));

    for (x, y) in &training_data[point..batch_size] {
        println!("Net 1 {}\t{}\t{}", x, original_fn(*x), net1.eval(*x));
        println!("Net 2 {}\t{}\t{}", x, original_fn(*x), net2.eval(*x));
        println!("Net 3 {}\t{}\t{}", x, original_fn(*x), net3.eval(*x));
        println!("Net 4 {}\t{}\t{}", x, original_fn(*x), net4.eval(*x));
    }
    Ok(())
}