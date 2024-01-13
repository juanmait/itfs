//! Export the `struct` [`AllowExtensions`]. Only will let through entries which extensions
/// are in a list of "allowed" ones.
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::io::Error;
use std::path::PathBuf;

/// Map an iterator over items of either type [`Result<DirEntry>`] or [`DirEntry`] or [`PathBuf`],
/// into one equivalent that will only let through entries which file extensions are in a list of
/// "allowed" ones.
///
/// This iterator does not filter any [Result::Err] coming from the inner iterator.
/// Those items will still pass the filter.
pub struct AllowExtensions<'a, T, I: Iterator<Item = T>>(
    /// Iterator to be filtered
    pub I,
    /// Reference to a vector of allowed extensions
    pub &'a Vec<&'a OsStr>,
);

/// Supports iterators over items of type `Result<DirEntry, Error>`
impl<I: Iterator<Item = Result<DirEntry, Error>>> Iterator
    for AllowExtensions<'_, Result<DirEntry, Error>, I>
{
    type Item = Result<DirEntry, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(Ok(entry)) => match entry.path().extension() {
                    Some(ext) => {
                        if self.1.contains(&ext) {
                            break Some(Ok(entry)); // pass ok
                        }

                        continue; // extension not allowed
                    }
                    None => continue, // no extension
                },
                Some(Err(err)) => {
                    break Some(Err(err)); // errors pass ok
                }
                None => {
                    break None; // self.0 reached the end
                }
            }
        }
    }
}

/// Supports iterators over items of type [DirEntry]
impl<I: Iterator<Item = DirEntry>> Iterator for AllowExtensions<'_, DirEntry, I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(entry) => match entry.path().extension() {
                    Some(ext) => {
                        if self.1.contains(&ext) {
                            break Some(entry);
                        }
                        continue; // extension not allowed
                    }
                    None => continue, // no extension
                },
                None => break None, // self.0 reached the end
            }
        }
    }
}

/// Supports iterators over items of type [PathBuf]
impl<I: Iterator<Item = PathBuf>> Iterator for AllowExtensions<'_, PathBuf, I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(entry) => match entry.extension() {
                    Some(ext) => {
                        if self.1.contains(&ext) {
                            break Some(entry);
                        }
                        continue; // extension not allowed
                    }
                    None => continue, // no extension
                },
                None => break None, // self.0 reached the end
            }
        }
    }
}
