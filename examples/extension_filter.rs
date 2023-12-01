use std::fs::read_dir;

use fs_iter::extension_filter::create_extension_filter;
use fs_iter::rdr::read_dir_recursive;
use fs_iter::result_filter::ResultFilter;

fn main() {
    const PATH: &'static str = ".";

    let allowed_extensions = vec!["md", "toml", "o"];

    // Use ExtensionFilter for the type `Result<DirEntry>`
    // ---------------------------------------------------------------

    let files_iterator = read_dir(PATH).unwrap();
    let extension_filter = create_extension_filter(files_iterator, allowed_extensions.clone());

    println!("\nStarting ExtensionFilter -> ReadDir ...");
    for result in extension_filter {
        println!("{:?}", result.unwrap().file_name())
    }

    // Use ExtensionFilter for the type `DirEntry` (we use `ResultFilter`
    // first to filter out items of type `Result::Err`)
    // ---------------------------------------------------------------

    let files_iterator = read_dir(PATH).unwrap();
    let result_filter = ResultFilter(files_iterator);

    let extension_filter = create_extension_filter(result_filter, allowed_extensions.clone());

    println!("\nStarting ExtensionFilter -> ResultFilter -> ReadDir ...");
    for item in extension_filter {
        println!("{:?}", item.file_name())
    }

    // Same as above but using `ReadDirRecursive` instead of ReadDir
    // ---------------------------------------------------------------

    let files_iterator = read_dir_recursive(PATH).unwrap();
    let result_filter = ResultFilter(files_iterator);
    let extension_filter = create_extension_filter(result_filter, allowed_extensions.clone());

    println!("\nStarting ExtensionFilter -> ResultFilter -> ReadDirRecursive...");
    for item in extension_filter {
        println!("{:?}", item.file_name())
    }
}
