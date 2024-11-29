//! Filter those items where any of its path's [Components][std::path::Components]
//! equals one given as parameter.

use std::{
    ffi::OsStr,
    fs::DirEntry,
    path::{Path, PathBuf},
};

pub enum ComponentFilterOperationType {
    Include,
    Exclude,
}

/// Filter those items where any of its path's [Components][std::path::Components]
/// equals one given as parameter.
///
/// This iterator can accept any iterator that yield items of type [`DirEntry`], `Result<DirEntry>`
/// [`PathBuf`][std::path::PathBuf] and `Result<PathBuf>`.
///
/// ## Example
/// ```
/// use itfs::{ComponentFilter, ResultFilter};
/// use std::ffi::OsStr;
///
/// let entry_result_iter = std::fs::read_dir(".").unwrap();
///
/// // use `ResultFilter` to drop `Result::Err` variants and keep only `DirEntry` items..
/// let entry_iter = ResultFilter(entry_result_iter);
///
/// // this iterator will skip any entry where the path contains a component named "target".
/// let iter = ComponentFilter::new(entry_iter, "target");
/// ```
pub struct ComponentFilter<'a, T, I>(pub I, pub &'a OsStr, pub ComponentFilterOperationType)
where
    I: Iterator<Item = T>;

/// It is possible to initialize `ComponentFilter` directly (without using `new`).
///
/// ## Example
/// ```
/// use itfs::{ComponentFilter,ResultFilter};
/// use std::ffi::OsStr;
///
/// let inner = ResultFilter(std::fs::read_dir(".").unwrap());
///
/// //
/// let iter = ComponentFilter(inner, OsStr::new("target"));
/// ```
impl<'a, T, I> ComponentFilter<'a, T, I>
where
    I: Iterator<Item = T>,
{
    /// The only advantage of using the `new` method is that you can pass a regular
    /// `&str` as the second param whereas an [OsStr] is required if the instance is
    /// created directly.
    ///
    /// ## Example
    ///
    /// ```
    /// use itfs::{ComponentFilter,ResultFilter};
    ///
    /// let inner_iter = ResultFilter(std::fs::read_dir(".").unwrap());
    ///
    /// for item in ComponentFilter::new(inner_iter, ".git") {
    /// dbg!(item);
    /// }
    /// ````
    pub fn new<R: AsRef<OsStr> + ?Sized>(
        it: I,
        component: &'a R,
        operation: ComponentFilterOperationType,
    ) -> ComponentFilter<'a, T, I> {
        Self(it, &component.as_ref(), operation)
    }

    fn entry_has_component(&self, dir_entry: &DirEntry) -> bool {
        self.path_buf_has_component(&dir_entry.path())
    }

    fn path_buf_has_component(&self, path_buf: &PathBuf) -> bool {
        Self::path_has_component(path_buf.as_path(), self.1)
    }

    fn path_has_component(path: &Path, osstr: &OsStr) -> bool {
        path.components().any(|c| c.as_os_str() == osstr)
    }
}

/// Implement the [Iterator] trait for a inner iterator that yields items of type [DirEntry].
impl<I> Iterator for ComponentFilter<'_, DirEntry, I>
where
    I: Iterator<Item = DirEntry>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(dir_entry) => match (&self.2, self.entry_has_component(&dir_entry)) {
                    (ComponentFilterOperationType::Include, true)
                    | (ComponentFilterOperationType::Exclude, false) => break Some(dir_entry),
                    (ComponentFilterOperationType::Include, false)
                    | (ComponentFilterOperationType::Exclude, true) => continue,
                },
                None => break None,
            }
        }
    }
}

/// Implement [Iterator] for an inner iterator that yields items of type `Result<DirEntry>`.
///
/// This implementation works similar to the
/// [FilterOk](https://docs.rs/itertools/latest/itertools/structs/struct.FilterOk.html)
/// iterator from the [itertools](https://docs.rs/itertools/latest/itertools/index.html) crate.
/// Items from the inner iterator will be filtered out if they are [Ok] variant with a value
/// that meet the filters criteria. However any `Err` variant coming from the inner iterator
/// will still pass the filter untouched.
impl<I, E> Iterator for ComponentFilter<'_, Result<DirEntry, E>, I>
where
    I: Iterator<Item = Result<DirEntry, E>>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(Ok(dir_entry)) => match (&self.2, self.entry_has_component(&dir_entry)) {
                    (ComponentFilterOperationType::Include, true)
                    | (ComponentFilterOperationType::Exclude, false) => break Some(Ok(dir_entry)),
                    (ComponentFilterOperationType::Include, false)
                    | (ComponentFilterOperationType::Exclude, true) => continue,
                },
                Some(Err(e)) => break Some(Err(e)),
                None => break None,
            }
        }
    }
}

/// Implement the [Iterator] trait for a inner iterator where the `Item` = [PathBuf].
impl<I> Iterator for ComponentFilter<'_, PathBuf, I>
where
    I: Iterator<Item = PathBuf>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(path_buf) => match (&self.2, self.path_buf_has_component(&path_buf)) {
                    (ComponentFilterOperationType::Include, true)
                    | (ComponentFilterOperationType::Exclude, false) => break Some(path_buf),
                    (ComponentFilterOperationType::Include, false)
                    | (ComponentFilterOperationType::Exclude, true) => continue,
                },
                None => break None,
            }
        }
    }
}

/// Implement the [Iterator] trait for a inner iterator where the `Item = Result<PathBuf>`.
///
/// This implementation works similar to the
/// [FilterOk](https://docs.rs/itertools/latest/itertools/structs/struct.FilterOk.html)
/// iterator from the [itertools](https://docs.rs/itertools/latest/itertools/index.html) crate.
/// `Ok` values from the inner iterator will be filtered out if necessary but any `Err`
/// variant coming from the inner iterator will still pass the filter untouched.
impl<I, E> Iterator for ComponentFilter<'_, Result<PathBuf, E>, I>
where
    I: Iterator<Item = Result<PathBuf, E>>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(Ok(path_buf)) => match (&self.2, self.path_buf_has_component(&path_buf)) {
                    (ComponentFilterOperationType::Include, true)
                    | (ComponentFilterOperationType::Exclude, false) => break Some(Ok(path_buf)),
                    (ComponentFilterOperationType::Include, false)
                    | (ComponentFilterOperationType::Exclude, true) => continue,
                },
                Some(Err(e)) => break Some(Err(e)),
                None => break None,
            }
        }
    }
}
