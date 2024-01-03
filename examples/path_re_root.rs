use std::fs::read_dir;

use itfs::{PathReRoot, ResultFilter};

fn main() {
    // build an iterator over the [PathBuf] items on both folders "./examples" and "./src"
    // respectively
    let fs_iter = ResultFilter(
        read_dir("./examples")
            .unwrap()
            .chain(read_dir("./src").unwrap()),
    )
    .map(|e| e.path());

    // build an iterator that replaces the root of the [PathBuf] items above
    let fs_iter = PathReRoot {
        inner_iter: fs_iter,
        strip_prefix: "./examples",
        replace_by: "./x/y/z",
    };

    for (original, result) in fs_iter {
        match result {
            Ok(re_rooted) => println!("{:?} => {:?}", original, re_rooted),
            Err(e) => println!("{:?} => {}", original, e.to_string()),
        }
    }
}
