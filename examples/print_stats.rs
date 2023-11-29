use itfs::rdr::read_dir_recursive;
use std::time;

/// ## Run this example
///
/// ```bash
/// cargo run --example print_stats
/// ```
///
/// ## List of Seen Errors
///
/// - Bad file descriptor (os error 9)
/// - Operation not permitted (os error 1)
/// - Permission denied (os error 13)
/// - Too many open files (os error 24)
fn main() {
    let path = std::path::PathBuf::from(".");
    let mut iterator = read_dir_recursive(&path).unwrap();
    let iter_started = time::Instant::now();

    println!("Started at root {:?}..", path.as_os_str());

    for (i, r) in iterator.by_ref().enumerate() {
        match r {
            Ok(_) => (),
            Err(e) => eprintln!("{i} {e}"),
        }
    }

    let elapsed = time::Instant::now().duration_since(iter_started);
    println!("Finished. Iteration took {elapsed:?} \n");
    println!("Stats: {:?}", iterator.stats);
    println!("");
    println!("Meta Errors: {:?}", iterator.meta_errors);
    println!("");
    println!("RD Errors: {:?}", iterator.rd_errors);
}
