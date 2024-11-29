//! Run this example:
//! ```no_rust
//! cargo run --example find_dirs_with_component
//! ```
use itfs::FindDirsWithComponent;

fn main() {
    let rdr = FindDirsWithComponent::new(".", "examples").unwrap();

    for entry in rdr {
        let found = entry.unwrap().path();
        dbg!(found);
    }
}
