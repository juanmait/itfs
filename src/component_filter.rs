//! Filter those items where any of its path's [Components][std::path::Components]
//! equals one given as parameter.

use std::{
    ffi::OsStr,
    fs::DirEntry,
    path::{Path, PathBuf},
};

/// Filter those items where any of its path's [Components][std::path::Components]
/// equals one given as parameter.
///
/// This iterator can accept any iterator that yield items of type [`DirEntry`], `Result<DirEntry>`
/// [`PathBuf`][std::path::PathBuf] and `Result<PathBuf>`.
///
/// ## Example
/// ```
/// use itfs::component_filter::ComponentFilter;
/// use itfs::result_filter::ResultFilter;
/// use std::ffi::OsStr;
///
/// let entry_result_iter = std::fs::read_dir(".").unwrap();
///
/// // use `ResultFilter` to drop `Result::Err` variants and keep only `DirEntry` items..
/// let entry_iter = ResultFilter(entry_result_iter);
///
/// // this iterator will skip any entry where the path contains a component named "target".
/// let iter = ComponentFilter::new(entry_iter, "target");
/// ````
pub struct ComponentFilter<'a, T, I>(pub I, pub &'a OsStr)
where
    I: Iterator<Item = T>;

/// It is possible to initialize `ComponentFilter` directly (without using `new`).
///
/// ## Example
/// ```
/// use itfs::component_filter::ComponentFilter;
/// use itfs::result_filter::ResultFilter;
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
    /// use itfs::component_filter::ComponentFilter;
    /// use itfs::result_filter::ResultFilter;
    ///
    /// let inner_iter = ResultFilter(std::fs::read_dir(".").unwrap());
    ///
    /// for item in ComponentFilter::new(inner_iter, ".git") {
    /// dbg!(item);
    /// }
    /// ````
    pub fn new<R: AsRef<OsStr> + ?Sized>(it: I, component: &'a R) -> ComponentFilter<'a, T, I> {
        Self(it, &component.as_ref())
    }
}

/// Implement the [Iterator] trait for a inner iterator where the `Item` = [DirEntry].
impl<I> Iterator for ComponentFilter<'_, DirEntry, I>
where
    I: Iterator<Item = DirEntry>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(e) => {
                    path_has_component(e.path().as_path(), self.1);
                    let b = e.path().components().any(|c| c.as_os_str() == self.1);
                    if !b {
                        break Some(e);
                    }

                    continue;
                }
                None => break None,
            }
        }
    }
}

/// Implement the [Iterator] trait for a inner iterator where the `Item = Result<DirEntry>`.
///
/// This implementation works similar to the
/// [FilterOk](https://docs.rs/itertools/latest/itertools/structs/struct.FilterOk.html)
/// iterator from the [itertools](https://docs.rs/itertools/latest/itertools/index.html) crate.
/// `Ok` values from the inner iterator will be filtered out if necessary but any `Err`
/// variant coming from the inner iterator will still pass the filter untouched.
impl<I, E> Iterator for ComponentFilter<'_, Result<DirEntry, E>, I>
where
    I: Iterator<Item = Result<DirEntry, E>>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(Ok(e)) => {
                    path_has_component(e.path().as_path(), self.1);
                    let b = e.path().components().any(|c| c.as_os_str() == self.1);
                    if !b {
                        break Some(Ok(e));
                    }

                    continue;
                }
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
                Some(p) => {
                    if !path_has_component(p.as_path(), self.1) {
                        break Some(p);
                    }

                    continue;
                }
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
                Some(Ok(e)) => {
                    path_has_component(e.as_path(), self.1);
                    let b = e.components().any(|c| c.as_os_str() == self.1);
                    if !b {
                        break Some(Ok(e));
                    }

                    continue;
                }
                Some(Err(e)) => break Some(Err(e)),
                None => break None,
            }
        }
    }
}

fn path_has_component(path: &Path, osstr: &OsStr) -> bool {
    path.components().any(|c| c.as_os_str() == osstr)
}
