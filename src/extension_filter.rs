//! Export the `struct` [`ExtensionFilter`]. Drop [DirEntry] items where the file
//! extension is not in a list of allowed ones.

use std::ffi::{OsStr, OsString};
use std::fs::DirEntry;
use std::io::Error;

/// Map an iterator over items of either type [`Result<DirEntry>`] or [`DirEntry`],
/// into one equivalent that only will let through entries where the file extension
/// is in a list of "allowed" ones ("only" filter).
///
/// This iterator does not filter any [Result::Err] coming from the inner iterator
/// so **they will still pass the filter**.
pub struct ExtensionFilter<T, I: Iterator<Item = T>>(I, Vec<OsString>);

/// Create an instance of [ExtensionFilter].
pub fn create_extension_filter<T, I: Iterator<Item = T>, A: AsRef<str>>(
    inner_iter: I,
    extensions: impl IntoIterator<Item = A>,
) -> ExtensionFilter<T, I> {
    ExtensionFilter::<T, I>(
        inner_iter,
        extensions
            .into_iter()
            .map(|e| OsString::from(e.as_ref()))
            .collect(),
    )
}

/// Implement [ExtensionFilter]
impl<T, I: Iterator<Item = T>> ExtensionFilter<T, I> {
    /// Create a new instance of [ExtensionFilter].
    pub fn new<A: AsRef<str>>(inner_iter: I, extensions: impl IntoIterator<Item = A>) -> Self {
        create_extension_filter(inner_iter, extensions)
    }

    /// Check if the given extension is in the list of allowed
    /// extensions
    fn is_allowed_extension(&self, ext: &OsStr) -> bool {
        let mut file_extension_is_allowed = false;

        for e in self.1.iter() {
            if ext == e {
                file_extension_is_allowed = true;
                break;
            }
        }

        file_extension_is_allowed
    }
}

/// Implement Iterator for `Item =  Result<DirEntry, Error>`
impl<T: Iterator<Item = Result<DirEntry, Error>>> Iterator
    for ExtensionFilter<Result<DirEntry, Error>, T>
{
    type Item = Result<DirEntry, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(Ok(entry)) => match entry.path().extension() {
                    Some(ext) => {
                        if self.is_allowed_extension(ext) {
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

/// Implement Iterator for `Item =  DirEntry`
impl<T: Iterator<Item = DirEntry>> Iterator for ExtensionFilter<DirEntry, T> {
    type Item = T::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(entry) => match entry.path().extension() {
                    Some(ext) => {
                        if self.is_allowed_extension(ext) {
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
