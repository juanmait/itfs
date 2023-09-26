use readdir_recursive::read_dir_recursive;
use std::time;

/// ## Run this example
///
/// ```bash
/// cargo run --example iteration
/// ```
///
/// ## Run in --release mode (prevents stack overflows)
///
/// ```bash
/// cargo run --example iteration --release
/// ```
fn main() {
    // record the start time of the iteration
    let iter_started = time::Instant::now();
    println!("started..");
    for (i, r) in read_dir_recursive(".").unwrap().enumerate() {
        println!("{} Found file: '{:?}'", i, r.unwrap().path());
    }

    let elapsed = time::Instant::now().duration_since(iter_started);
    println!("Finished. Iteration took {elapsed:?} \n");
}
