use itfs::read_dir_recursive;
use std::time;

/// ## Run this example
///
/// ```bash
/// cargo run --example iteration
/// ```
fn main() {
    let path = std::path::PathBuf::from(".");

    let iterator = read_dir_recursive(&path).unwrap();
    let iter_started = time::Instant::now();

    println!("Started at root {:?}..", path.as_os_str());

    for (i, r) in iterator.enumerate() {
        match r {
            Ok(entry) => {
                println!("{} Found file: '{:?}'", i, entry.path());
            }
            Err(e) => {
                eprintln!("{} Found ERROR: {}", i, e);
            }
        }
    }

    let elapsed = time::Instant::now().duration_since(iter_started);
    println!("Finished. Iteration took {elapsed:?} \n");
}
