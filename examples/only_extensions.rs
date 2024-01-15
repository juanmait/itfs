use std::{ffi::OsStr, fs::read_dir};

use itfs::{AllowExtensions, EntryToPath, ResultFilter};

fn main() {
    let root_path = ".";

    let allowed_extensions = vec![OsStr::new("md"), OsStr::new("toml")];

    // Support iterators over items of type: [`DirEntry`] ...

    let orig_iter = ResultFilter(read_dir(root_path).unwrap());
    let next_iter = AllowExtensions(orig_iter, &allowed_extensions);

    for entry in next_iter {
        println!("{:?}", entry.file_name())
    }

    // Support iterators over items of type: `[Result<DirEntry>]` ...

    let orig_iter = read_dir(root_path).unwrap();
    let next_iter = AllowExtensions(orig_iter, &allowed_extensions);

    for result in next_iter {
        println!("{:?}", result.unwrap().file_name())
    }

    // Support iterators over items type: `[PathBuf]` ...

    let orig_iter = EntryToPath(ResultFilter(read_dir(root_path).unwrap()));
    let next_iter = AllowExtensions(orig_iter, &allowed_extensions);

    for entry in next_iter {
        println!("{:?}", entry.file_name().unwrap())
    }
}
