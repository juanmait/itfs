//! Extension that add the filter_extension method to

use std::{ffi::OsStr, fs::DirEntry, path::PathBuf};

use crate::OnlyExtensions;

pub trait ExtensionFilterExt<'a, T>: Iterator<Item = T> + Sized {
    /// Returns an iterator equivalent to the original but that i'll "hide/skip/drop" entries having a file extension that is NOT part of a given group.
    /// It won't change the type of the items coming from the original iterator
    fn only_extensions(self, extensions: &'a Vec<&'a OsStr>) -> OnlyExtensions<'a, T, Self> {
        return OnlyExtensions(self, extensions);
    }
}

impl<I> ExtensionFilterExt<'_, DirEntry> for I where I: Iterator<Item = DirEntry> {}
impl<I> ExtensionFilterExt<'_, PathBuf> for I where I: Iterator<Item = PathBuf> {}
impl<I, E> ExtensionFilterExt<'_, Result<DirEntry, E>> for I where
    I: Iterator<Item = Result<DirEntry, E>>
{
}
