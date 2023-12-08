//! Rust iterator adaptors useful when iterating over the file system.
//!
//! -   [extension_filter]: Export the struct [`ExtensionFilter`][extension_filter::ExtensionFilter].
//!     Filter [`DirEntry`][std::fs::DirEntry] items where the the file extension is not in a list of allowed ones.
//! -   [rdr]: Export the struct [`ReadDirRecursive`][rdr::ReadDirRecursive]. Iterator similar to the standard [`fs::ReadDir`][std::fs::ReadDir]
//!     but recursive.
//! -   [result_filter]: Export the struct [`ResultFilter`][result_filter::ResultFilter]. It maps an iterator over items of type
//!     `Result<T>` into one over items of type `T` by discarding `Err` variants.

pub mod extension_filter;
pub mod rdr;
pub mod result_filter;
