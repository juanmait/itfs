/*!
Iterator similar to the standard [fs::ReadDir] but recursive.

**WARNING** The recursive iteration of directories seems to be very fast but applied to directories
with many files and/or subdirectories can lead to panics due to **stack overflows** 💥.
Seems like not much of the hip is being used and everything goes into the stack 🔥.

*/

use std::{fs, io, path};

/// Stores statistic data about the items found in the iteration of [ReadDirRecursive]
#[derive(Default, Debug)]
pub struct RDRStats {
    /// Maximum registered number of directories waiting for inspection.
    pub max_stacked_dirs: usize,
    pub total_files_consumed: usize,
    pub total_dirs_consumed: usize,
    pub total_iterations: usize,
}

/// Iterator similar to the standard [fs::ReadDir] but recursive.
#[derive(Debug)]
pub struct ReadDirRecursive {
    /// This field hods the [fs::ReadDir] instance that is currently being iterated.
    ///
    /// At the beginning it holds the iterator for the root directory (the path given to
    /// the `new` method), but later when all files on it where consumed (the iterator for
    /// the folder reached the end) it will be replaced by new instances of [fs::ReadDir]
    /// as the process visit the subdirectories.
    pub read_dir: fs::ReadDir,
    /// Directories are not visited immediately after being found. Instead they're
    /// pushed onto a queue for a later inspection once the current directory is done and all
    /// entries were consumed.
    pub pending_dirs: Vec<fs::DirEntry>,
    /// stores some stats about the ongoing iteration
    pub stats: RDRStats,
}

impl ReadDirRecursive {
    /// Create a new instance of [ReadDirRecursive] for the given path.
    ///
    /// ```
    /// let rdr = ReadDirRecursive::new("/some/path");
    /// ```
    pub fn new<P: AsRef<path::Path>>(path: P) -> io::Result<Self> {
        Ok(ReadDirRecursive {
            pending_dirs: vec![],
            read_dir: fs::read_dir(path)?,
            stats: RDRStats::default(),
        })
    }
}

// Implement Iterator for ReadDirRecursive
impl Iterator for ReadDirRecursive {
    // our Item is the same as the wrapped iter
    type Item = io::Result<fs::DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.stats.total_iterations += 1;
        match self.read_dir.next() {
            // entry found
            Some(Ok(entry)) => match entry.metadata() {
                Ok(meta) => {
                    // if the entry is a directory we need to save it for later inspection
                    // and move to the next entry in the iterator..
                    if meta.is_dir() {
                        // push the directory onto the stack
                        self.pending_dirs.push(entry);

                        self.stats.total_dirs_consumed += 1;

                        if self.pending_dirs.len() > self.stats.max_stacked_dirs {
                            self.stats.max_stacked_dirs = self.pending_dirs.len();
                        }
                        // move to the next entry
                        return self.next();
                    }

                    // if the entry is not a directory we just assume that is a file
                    self.stats.total_files_consumed += 1;
                    Some(Ok(entry))
                }
                // Error trying to obtain the entry's metadata.
                // Since we can know if the entry is a file or directory we just return the
                // error instead of the entry.
                Err(e) => Some(Err(e)),
            },
            // Entry found but is an error. No special treatment, we just return the error as is
            Some(Err(err)) => Some(Err(err)),
            // The current `read_dir` iterator reached the end (there are no more
            // files/entries in this directory).
            None => {
                // deal with the next pending directory (if any)
                if let Some(dir_entry) = self.pending_dirs.pop() {
                    match fs::read_dir(dir_entry.path()) {
                        Ok(read_dir) => {
                            // throw away the consumed iterator and put the new one in his place
                            self.read_dir = read_dir;
                            return self.next();
                        }
                        Err(e) => return Some(Err(e)),
                    }
                }
                None
            }
        }
    }
}

/// Create an instance of [ReadDirRecursive] for the given path.
/// The same can be achieved by passing the path to the `new` method of [ReadDirRecursive].
///
/// Example:
/// ```
/// let rdr = ReadDirRecursive::new("/some/path");
/// ```
pub fn read_dir_recursive<P: AsRef<path::Path>>(path: P) -> io::Result<ReadDirRecursive> {
    ReadDirRecursive::new(path)
}
