//! Export the `struct` [`EntryToPath`]. Maps an iterator over items of
//! type [`DirEntry`] or [`Result<DirEntry>`] into one over items of type
//! [`PathBuf`] and [`Result<PathBuf>`] respectively.

use std::{fs::DirEntry, io::Result, path::PathBuf};

/// Maps an iterator over items of type [`DirEntry`] or [`Result<DirEntry>`] into one
/// over items of type [`PathBuf`] and [`Result<PathBuf>`] respectively.
pub struct EntryToPath<T, I: Iterator<Item = T>>(pub I);

/// The implementation for `DirEntry` items must yield items of type [PathBuf]
impl<I: Iterator<Item = DirEntry>> Iterator for EntryToPath<DirEntry, I> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(e) => Some(e.path()),
            None => None,
        }
    }
}

/// The implementation for [`Result<DirEntry>`] items must yield items of type [`Result<PathBuf>`].
///
/// In this implementation any [`Err`] variant coming out of the original iterator are left "as is".
impl<I: Iterator<Item = Result<DirEntry>>> Iterator for EntryToPath<Result<DirEntry>, I> {
    type Item = Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(Ok(e)) => Some(Ok(e.path())),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }
}
