use readdir_recursive::read_dir_recursive;
use std::time;

/// ## Run this example
///
/// ```bash
/// cargo run --example print_stats
/// ```
///
/// ## Run in --release mode (prevents stack overflows)
///
/// ```bash
/// cargo run --example print_stats --release
/// ```
///
/// ## List of Seen Errors
///
/// - Bad file descriptor (os error 9)
/// - Operation not permitted (os error 1)
/// - Permission denied (os error 13)
/// - Too many open files (os error 24)
fn main() {
    let now = time::SystemTime::now();
    // record the start time of the iteration
    let iter_started = time::Instant::now();
    let mut rdr = read_dir_recursive(".").unwrap();

    println!("Started at {:?}", now);
    for (i, r) in rdr.by_ref().enumerate() {
        match r {
            Ok(_) => (),
            Err(e) => eprintln!("{i} {e}"),
        }
    }

    let elapsed = time::Instant::now().duration_since(iter_started);
    println!("Finished. Iteration took {elapsed:?} \n");
    println!("Stats: {:?}", rdr.stats);
    println!("");
    println!("Meta Errors: {:?}", rdr.meta_errors);
    println!("");
    println!("RD Errors: {:?}", rdr.rd_errors);
}
