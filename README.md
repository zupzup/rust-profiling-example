# rust-profiling-example

Example for using some basic profiling tools with a Rust web-app.

For creating load, we use [locust](https://locust.io/), for creating flamegraphs, we use [cargo-flamegraph](https://github.com/flamegraph-rs/flamegraph).

Run with `cargo build --release && ./target/release/rust-profiling-example` to start a webserver on http://localhost:8080 with the following routes:

* `GET /read` - reads from a shared hashmap protected by a Mutex, holding the lock during the whole request`
* `GET /fast` - reads from a shared hashmap protected by an RwLock, releasing the lock immediately after use`
* `GET /cpu` - does a resource-intensive loop, adding up numbers based on the shared hashmap data
* `GET /cpualloc` - does the same calculation, but clones the data every iteration, resulting in lots of time wasted during allocations 
