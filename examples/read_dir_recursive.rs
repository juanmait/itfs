use itfs::rdr::read_dir_recursive;

/// ## Run this example
///
/// ```bash
/// cargo run --example read_dir_recursive
/// ```
fn main() {
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
}
