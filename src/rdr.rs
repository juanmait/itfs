//! Export the `struct` [`ReadDirRecursive`]. Iterator similar to the standard [fs::ReadDir] but recursive.
use std::{fs, io, path};

/// Iterator similar to the standard [fs::ReadDir] but recursive.
///
/// ## Example:
/// ```
/// use itfs::ReadDirRecursive;
///
/// let rdr = ReadDirRecursive::new(".").unwrap();
///
/// for entry_result in rdr {
///     let entry = entry_result.unwrap();
///     println!("Found file: '{:?}'", entry.path());
/// }
/// ```
pub struct ReadDirRecursive {
    /// This field hods the [fs::ReadDir] instance that is currently being iterated.
    ///
    /// At the beginning, it holds the [fs::ReadDir] iterator of the root directory
    /// (given as param) but later, when all entries in the root where consumed (the
    /// iterator reached the end) it will be replaced by a new instances of [fs::ReadDir]
    /// as the main iteration continues visiting subdirectories of the root.
    pub read_dir: fs::ReadDir,
    /// Sub Directories are not visited immediately when found. Instead they're
    /// pushed onto a vector of pending directories/[entries][fs::DirEntry] (this field)
    /// and the iteration of the current directory continues with the next entry.
    /// Once that iteration is done, [ReadDirRecursive] will `pop` one directory from this stack,
    /// create a new instance of [fs::ReadDir] for it and resume the iteration.
    pub pending_dirs: Vec<fs::DirEntry>,
}

impl ReadDirRecursive {
    /// Create a new instance of [ReadDirRecursive] for the given path. This operation
    /// will fail if the initial call to [fs::read_dir] fails.
    ///
    /// ```
    /// use itfs::ReadDirRecursive;
    ///
    /// let rdr = ReadDirRecursive::new(".").unwrap();
    /// ```
    pub fn new<P: AsRef<path::Path>>(path: P) -> io::Result<Self> {
        Ok(ReadDirRecursive {
            pending_dirs: vec![],
            read_dir: fs::read_dir(&path)?,
        })
    }
}

// Implement Iterator for ReadDirRecursive
impl Iterator for ReadDirRecursive {
    // our Item is the same as the wrapped iter
    type Item = io::Result<fs::DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.read_dir.next() {
                // entry found
                Some(Ok(entry)) => match entry.metadata() {
                    Ok(meta) => {
                        // if the entry is a directory we need to save it for later inspection
                        // and move on to the next entry in in the current directory.
                        if meta.is_dir() {
                            self.pending_dirs.push(entry);
                            // move to the next entry
                            continue;
                        }

                        // DirEntry found. Break the loop and yield it
                        break Some(Ok(entry));
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

/**
Create an instance of [ReadDirRecursive] for the given path.

This function is not lazy. Std [fs::read_dir] is called immediately to read from the given path,
so any error coming from that action must be handled to being able start the iteration.

Same behavior as using [ReadDirRecursive::new].

Example:
```
use itfs::read_dir_recursive;

for (i, r) in read_dir_recursive(".").unwrap().enumerate() {
        match r {
            Ok(entry) => {
                println!("{} Found entry: '{:?}'", i, entry.path());
            }
            Err(e) => {
                eprintln!("{} Found ERROR: {}", i, e);
            }
        }
    }
```
*/
pub fn read_dir_recursive<P: AsRef<path::Path>>(path: P) -> io::Result<ReadDirRecursive> {
    ReadDirRecursive::new(path)
}
