//! Export the `struct` [`FindDirsWithComponent`]. This iterator recursively searches in the given
//! path for all directories which name match with one given as parameter.
use std::{ffi::OsStr, fs, io, path};

/// Export the `struct` [`FindDirsWithComponent`]. This iterator recursively searches in the given
/// path for all directories which name match with one given as parameter. Once it found a match,
/// it will yield it's path without inspecting the content of such directory. It will however
/// continue the search in any other found directory that does not match.
///
/// ## Example:
/// ```
/// use itfs::FindDirsWithComponent;
///
/// let rdr = FindDirsWithComponent::new(".", "examples").unwrap();
///
/// for entry_result in rdr {
///     let entry = entry_result.unwrap();
///     println!("Found folder: '{:?}'", entry.path());
/// }
/// ```
pub struct FindDirsWithComponent<'a> {
    component: &'a OsStr,
    /// This field hods the [fs::ReadDir] instance that is currently being iterated.
    ///
    /// At the beginning, it holds the [fs::ReadDir] iterator of the root directory
    /// (given as param) but later, when all entries in the root where consumed (the
    /// iterator reached the end) it will be replaced by a new instances of [fs::ReadDir]
    /// as the main iteration continues visiting subdirectories of the root.
    read_dir: fs::ReadDir,
    /// Sub Directories are not visited immediately when found. Instead they're
    /// pushed onto a vector of pending directories/[entries][fs::DirEntry] (this field)
    /// and the iteration of the current directory continues with the next entry.
    /// Once that iteration is done, [FindDirsWithComponent] will `pop` one directory from this stack,
    /// create a new instance of [fs::ReadDir] for it and resume the iteration.
    pending_dirs: Vec<fs::DirEntry>,
}

impl<'a> FindDirsWithComponent<'a> {
    /// Create a new instance of [FindDirsWithComponent] for the given path. This operation
    /// will fail if the initial call to [fs::read_dir] fails.
    ///
    /// ```
    /// use itfs::FindDirsWithComponent;
    ///
    /// let fdwc = FindDirsWithComponent::new(".", "examples").unwrap();
    /// ```
    pub fn new<P: AsRef<path::Path>, R: AsRef<OsStr> + ?Sized>(
        path: P,
        component: &'a R,
    ) -> io::Result<Self> {
        Ok(FindDirsWithComponent {
            component: component.as_ref(),
            pending_dirs: vec![],
            read_dir: fs::read_dir(&path)?,
        })
    }
}

// Implement Iterator for FindDirsWithComponent
impl Iterator for FindDirsWithComponent<'_> {
    type Item = io::Result<fs::DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.read_dir.next() {
                Some(Ok(entry)) => match entry.metadata() {
                    Ok(meta) => {
                        if meta.is_dir() {
                            if entry
                                .path()
                                .as_path()
                                .components()
                                .any(|c| c.as_os_str() == self.component)
                            {
                                break Some(Ok(entry));
                            }

                            self.pending_dirs.push(entry);
                            // move to the next entry
                        }

                        continue;
                    }
                    // Error trying to obtain the entry's metadata.
                    Err(e) => break Some(Err(e)),
                },
                // Entry found but is an error. No special treatment, we just yield the error as is
                Some(Err(err)) => break Some(Err(err)),
                None => {
                    // The current `ReadDir` iterator finished (there are no more entries in it).
                    // We need to either move on to the next directory in the queue if there is any
                    // or finish the iteration completely.
                    if let Some(dir_entry) = self.pending_dirs.pop() {
                        let entry_path = dir_entry.path();
                        match fs::read_dir(&entry_path) {
                            Ok(read_dir) => {
                                // throw away the consumed iterator and put the new one in his place
                                self.read_dir = read_dir;

                                // skip to the next iteration
                                continue;
                            }
                            // something went wrong reading a directory
                            Err(e) => break Some(Err(e)),
                        }
                    }

                    // there are no more directories to go through
                    break None;
                }
            }
        }
    }
}
