use std::ffi::OsStr;

use itfs::{read_dir_recursive, ComponentFilter, EntryToPath, ResultFilter};

/// This example shows that it works for an inner iterator that yields
/// items of type DirEntry. Also shows how one can initialize ComponentFilter directly
/// without using the `new` method by giving a [OsStr] as second argument.
fn works_for_dir_entry() {
    // iterator over items of type `Result<DirEntry>`
    let rdr = read_dir_recursive(".").unwrap();

    // iterator over items of type `DirEntry`
    let dir_entry_iter = ResultFilter(rdr);

    // without using the method `ComponentFilter::new` we have to pass a `OsStr`
    // as the second parameter
    let iter = ComponentFilter(
        dir_entry_iter,
        OsStr::new("target"),
        itfs::ComponentFilterOperationType::Exclude,
    );

    let iter_started = std::time::Instant::now();
    for entry in iter {
        let path = entry.path();

        println!("{path:?}")
    }

    let elapsed = std::time::Instant::now().duration_since(iter_started);
    println!("Iteration took: {:?}", elapsed);
}

/// This example shows that is works for an inner iterator that yields items of type
/// [PathBuf][std::path::PathBuf]. It also shows the usage of the `new` method which can
/// receive a regular `&str` (as well as a [OsStr]).
fn works_for_pathbuf() {
    // iterator over items of type `Result<DirEntry>`
    let rdr = read_dir_recursive(".").unwrap();

    // iterator over items of type `DirEntry`
    let dir_entry_iter = ResultFilter(rdr);

    // iterator over items of type `PathBuf`
    let path_buf_iter = EntryToPath(dir_entry_iter);

    // using the `new` method we can pass a plain &str directly as second parameter
    let iter = ComponentFilter::new(
        path_buf_iter,
        "target",
        itfs::ComponentFilterOperationType::Exclude,
    );

    let iter_started = std::time::Instant::now();
    for path in iter {
        println!("{path:?}")
    }

    let elapsed = std::time::Instant::now().duration_since(iter_started);
    println!("Iteration took: {:?}", elapsed);
}

fn main() {
    works_for_dir_entry();
    works_for_pathbuf();
}
