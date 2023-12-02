use std::fs::read_dir;

use itfs::extension_filter::{create_extension_filter, ExtensionFilter};
use itfs::rdr::ReadDirRecursive;
use itfs::result_filter::ResultFilter;

fn main() {
    const PATH: &'static str = ".";

    let allowed_extensions = ["md", "toml", "o"];

    // Use ExtensionFilter for the type `Result<DirEntry>`
    // ---------------------------------------------------------------

    let files_iterator = read_dir(PATH).unwrap();
    let extension_filter = create_extension_filter(files_iterator, allowed_extensions);

    println!("\nStarting ExtensionFilter -> ReadDir ...");
    for result in extension_filter {
        println!("{:?}", result.unwrap().file_name())
    }

    // Use ExtensionFilter for the type `DirEntry` (we use `ResultFilter`
    // first to filter out items of type `Result::Err`)
    // ---------------------------------------------------------------

    let files_iterator = read_dir(PATH).unwrap();
    let result_filter = ResultFilter(files_iterator);

    let extension_filter = create_extension_filter(result_filter, allowed_extensions);

    println!("\nStarting ExtensionFilter -> ResultFilter -> ReadDir ...");
    for item in extension_filter {
        println!("{:?}", item.file_name())
    }

    // Same as above but using `ReadDirRecursive` instead of ReadDir
    // and using a more "structured" api
    // ---------------------------------------------------------------

    let itr = ExtensionFilter::new(
        ResultFilter(ReadDirRecursive::new(".").unwrap()),
        allowed_extensions,
    );

    println!("\nStarting ExtensionFilter -> ResultFilter -> ReadDirRecursive...");
    for entry in itr {
        println!("{:?}", entry.file_name())
    }
}
