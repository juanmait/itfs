use std::ffi::OsStr;

use itfs::{
    ext::only_extensions_ext::ExtensionFilterExt, read_dir_recursive, EntryToPath, ResultFilter,
};

fn main() {
    let allowed_extensions = vec![OsStr::new("o")];

    let orig_iter = ResultFilter(EntryToPath(read_dir_recursive(".").unwrap()));
    let next_iter = orig_iter.only_extensions(&allowed_extensions);

    for item in next_iter {
        println!("{:?}", item);
    }
}
