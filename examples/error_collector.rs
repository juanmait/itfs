//! Run this example:
//!
//! ```no_rust
//! cargo run --release --example error_collector
//! ```
use itfs::ErrorCollector;

fn main() {
    let items = [
        Result::Ok(0),
        Result::Err(1),
        Result::Ok(2),
        Result::Ok(3),
        Result::Err(4),
        Result::Ok(5),
    ];

    let mut v = Vec::new();

    for item in ErrorCollector(items.into_iter(), &mut v) {
        println!("{:?}", item);
    }

    dbg!(v);
}
