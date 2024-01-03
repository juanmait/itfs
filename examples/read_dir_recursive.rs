use itfs::read_dir_recursive;

/// ## Run this example
///
/// ```bash
/// cargo run --example read_dir_recursive
/// ```
fn main() {
    let iter_started = std::time::Instant::now();
    for (i, r) in read_dir_recursive(".").unwrap().enumerate() {
        match r {
            Ok(entry) => {
                println!("{} Found entry: '{:?}'", i, entry.path());
            }
            Err(e) => {
                eprintln!("{} Found ERROR: {}", i, e);
            }
        }
    }
    let elapsed = std::time::Instant::now().duration_since(iter_started);
    println!("Iteration took: {:?}", elapsed);
}
