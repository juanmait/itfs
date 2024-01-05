//! Extension that adds the method `only_extensions` to any iterator over items
//! of type [DirEntry], `Result<DirEntry>`, [PathBuf]

use std::{ffi::OsStr, fs::DirEntry, path::PathBuf};

use crate::OnlyExtensions;

pub trait OnlyExtensionsExt<'a, T>: Iterator<Item = T> + Sized {
    /// Returns an iterator equivalent to the original but that i'll "hide/skip/drop" entries having a
    /// file extension that is NOT part of a given group.
    ///
    /// The resulting iterator won't change the type of the items coming from the original iterator.
    fn only_extensions(self, extensions: &'a Vec<&'a OsStr>) -> OnlyExtensions<'a, T, Self> {
        return OnlyExtensions(self, extensions);
    }
}

impl<I> OnlyExtensionsExt<'_, DirEntry> for I where I: Iterator<Item = DirEntry> {}
impl<I> OnlyExtensionsExt<'_, PathBuf> for I where I: Iterator<Item = PathBuf> {}
impl<I, E> OnlyExtensionsExt<'_, Result<DirEntry, E>> for I where
    I: Iterator<Item = Result<DirEntry, E>>
{
}
