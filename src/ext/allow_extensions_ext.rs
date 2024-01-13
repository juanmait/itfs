//! Extension that adds the method `only_extensions` to any iterator over items
//! of type [DirEntry], `Result<DirEntry>` and [PathBuf]

use std::{ffi::OsStr, fs::DirEntry, path::PathBuf};

use crate::AllowExtensions;

pub trait AllowExtensionsExt<'a, T>: Iterator<Item = T> + Sized {
    /// Returns an iterator equivalent to the original but that skips/drops entries which
    /// file extension is NOT in the provided list of allowed ones.
    ///
    /// The resulting iterator won't change the type of the items coming from the original iterator.
    fn only_extensions(self, extensions: &'a Vec<&'a OsStr>) -> AllowExtensions<'a, T, Self> {
        return AllowExtensions(self, extensions);
    }
}

/// Implementation for iterators over items of the type [DirEntry]
impl<I> AllowExtensionsExt<'_, DirEntry> for I where I: Iterator<Item = DirEntry> {}
/// Implementation for iterators over items of the type [PathBuf]
impl<I> AllowExtensionsExt<'_, PathBuf> for I where I: Iterator<Item = PathBuf> {}
/// Implementation for iterators over items of the type `Result<PathBuf>`
impl<I, E> AllowExtensionsExt<'_, Result<DirEntry, E>> for I where
    I: Iterator<Item = Result<DirEntry, E>>
{
}
