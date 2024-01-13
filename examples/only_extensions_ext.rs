use std::ffi::OsStr;

use itfs::{
    ext::allow_extensions_ext::AllowExtensionsExt, read_dir_recursive, EntryToPath, ResultFilter,
};

fn main() {
    let root_path = ".";
    let allowed_extensions = vec![OsStr::new("md"), OsStr::new("toml")];

    // Support iterators over items of type: [`DirEntry`] ...
    let orig_iter = ResultFilter(read_dir_recursive(root_path).unwrap());
    let next_iter = orig_iter.only_extensions(&allowed_extensions);

    for item in next_iter {
        println!("{:?}", item);
    }

    // Support iterators over items of type: [Result<DirEntry>] ...
    let orig_iter = read_dir_recursive(root_path).unwrap();
    let next_iter = orig_iter.only_extensions(&allowed_extensions);

    for item in next_iter {
        println!("{:?}", item);
    }

    // Support iterators over items type: `[PathBuf]` ...
    let orig_iter = EntryToPath(ResultFilter(read_dir_recursive(root_path).unwrap()));
    let next_iter = orig_iter.only_extensions(&allowed_extensions);

    for entry in next_iter {
        println!("{:?}", entry)
    }
}
