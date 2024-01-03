use itfs::{EntryToPath, ReadDirRecursive, ResultFilter};

/// Demonstrates mapping from [DirEntry] to [PathBuf]
fn example_entry_to_path<P: AsRef<std::path::Path>>(path: P) {
    let itr = EntryToPath(ResultFilter(ReadDirRecursive::new(path).unwrap()));

    for item in itr {
        println!("{}", item.to_str().unwrap())
    }
}

/// Demonstrates mapping from [Result<DirEntry>] to [Result<PathBuf>]
fn example_result_to_path<P: AsRef<std::path::Path>>(path: P) {
    let itr = EntryToPath(ReadDirRecursive::new(path).unwrap());

    for item in itr {
        println!("{}", item.unwrap().to_str().unwrap())
    }
}

fn main() {
    let root_path = ".";
    example_entry_to_path(root_path);
    example_result_to_path(root_path)
}
