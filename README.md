# Parameter Server

- Download Rust using [rustup](https://rustup.rs).


- Run cargo run --bin launch-server to start the servers
- Run cargo run --bin launch-keeper to start the keeper to manage the servers
- Change the names of the models you want to train in run-initializer
- Run cargo run --bin run-initializer to distribute the initial models to the servers
- Change the target functions, model architectures, models to train and hyperparameters in each of the worker scripts (launch_worker_i.rs)
- In seperate terminals, to launch worker i (1<=i<=4), run cargo run --bin launch-worker-i to start that worker.