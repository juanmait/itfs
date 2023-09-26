use readdir_recursive::read_dir_recursive;
use std::time;

fn main() {
    // record the start time of the iteration
    let iter_started = time::Instant::now();
    println!("started..");
    for _ in read_dir_recursive(".").unwrap() {}
    let elapsed = time::Instant::now().duration_since(iter_started);
    println!("finished. Iteration took {elapsed:?} \n");
}
