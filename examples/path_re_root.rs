use std::fs::read_dir;

use itfs::{path_reroot::PathReRoot, result_filter::ResultFilter};

fn main() {
    let paths = ResultFilter(
        read_dir("./examples")
            .unwrap()
            .chain(read_dir("./src").unwrap()),
    )
    .map(|e| e.path());

    let re_rooted = PathReRoot {
        inner_iter: paths,
        strip_prefix: "./examples",
        replace_by: "",
    };

    for path in re_rooted {
        match path {
            Ok(p) => println!("{:?}", p),
            Err(e) => println!("{}", e.to_string()),
        }
    }
}
