//! Rust iterator adaptors useful when iterating over the file system.

mod allow_extensions;
mod component_filter;
mod entry_to_path;
mod error_collector;
mod find_dirs_with_component;
mod path_reroot;
mod rdr;
mod result_filter;

pub mod ext;

pub use allow_extensions::AllowExtensions;
pub use component_filter::{ComponentFilter, ComponentFilterOperationType};
pub use entry_to_path::EntryToPath;
pub use error_collector::ErrorCollector;
pub use find_dirs_with_component::FindDirsWithComponent;
pub use path_reroot::PathReRoot;
pub use rdr::read_dir_recursive;
pub use rdr::ReadDirRecursive;
pub use result_filter::ResultFilter;
