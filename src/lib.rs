//! Iterator similar to the standard [fs::ReadDir] but recursive.

use std::{collections::HashMap, fs, io, path, time};

/// Stores some numbers about the items found in the iteration of [ReadDirRecursive]
#[derive(Default, Debug)]
pub struct RDRStats {
    /// Marks the start of the iteration (first call to next) in seconds since [time::UNIX_EPOCH]
    pub iteration_started: Option<u64>,
    /// Maximum registered number of directories waiting for inspection.
    pub max_stacked_dirs: usize,
    /// Count of the total files seen so far.
    pub total_files_consumed: usize,
    /// Count of the total directories seen so far.
    pub total_dirs_consumed: usize,
    /// This is essentially the total number of calls to the `next` method
    pub total_iterations: usize,
}

/// Iterator similar to the standard [fs::ReadDir] but recursive.
///
/// ## Iteration Example:
/// ```
/// use readdir_recursive::ReadDirRecursive;
///
/// let rdr = ReadDirRecursive::new(".").unwrap();
///
/// for entry_result in rdr {
///     let entry = entry_result.unwrap();
///     println!("Found file: '{:?}'", entry.path());
/// }
/// ```
///
/// ## Print some stats afterward:
/// ```
/// use readdir_recursive::ReadDirRecursive;
///
/// let mut rdr = ReadDirRecursive::new(".").unwrap();
///
/// // use a reference to `rdr` this time (it allows to access
/// // it later after the loop).
/// for entry_result in rdr.by_ref() {
///     let entry = entry_result.unwrap();
///     println!("Found file: '{:?}'", entry.path());
/// }
///
/// println!("{:?}", rdr.stats);
/// ```
pub struct ReadDirRecursive {
    /// Path of the directory currently being iterated. Initially this is the root path
    /// where the iteration will start.
    pub current_path: path::PathBuf,
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
    /// Once that iteration is done, [ReadDirRecursive] will `pop` one entry from this stack,
    /// create an instance of [fs::ReadDir] and resume the iteration.
    pub pending_dirs: Vec<fs::DirEntry>,
    /// Stores some data about the ongoing iteration. Mostly for debugging.
    pub stats: RDRStats,
    /// Record of errors triggered while obtaining DirEntry's metadata.
    /// It is [HashMap] where the keys are the string representation of the error
    /// and values are collections of the different entry paths that triggered
    /// such error.
    pub meta_errors: HashMap<String, Vec<String>>,
    /// Errors registered while calling the inner readdir's `next` method
    pub rd_errors: HashMap<String, Vec<String>>,
}

impl ReadDirRecursive {
    /// Create a new instance of [ReadDirRecursive] for the given path. This operation
    /// can fail if [fs::read_dir] fails while trying to read from the specified path.
    ///
    /// ```
    /// use readdir_recursive::ReadDirRecursive;
    ///
    /// let rdr = ReadDirRecursive::new("/some/path").unwrap();
    /// ```
    pub fn new<P: AsRef<path::Path>>(path: P) -> io::Result<Self> {
        Ok(ReadDirRecursive {
            current_path: path.as_ref().to_owned(),
            pending_dirs: vec![],
            read_dir: fs::read_dir(&path)?,
            stats: RDRStats::default(),
            meta_errors: HashMap::default(),
            rd_errors: HashMap::default(),
        })
    }

    fn mark_start(&mut self) {
        let r = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.stats.iteration_started = Some(r);
    }

    /// Push the directory onto `pending_dirs` queue and update
    /// directory related stats.
    fn digest_dir(&mut self, entry: fs::DirEntry) {
        // push the directory onto the stack
        self.pending_dirs.push(entry);
        self.stats.total_dirs_consumed += 1;

        // inspect/update max_stacked_dirs
        if self.pending_dirs.len() > self.stats.max_stacked_dirs {
            self.stats.max_stacked_dirs = self.pending_dirs.len();
        }
    }

    /// update the `meta_errors`` record
    fn register_meta_error(&mut self, e: &io::Error, entry: &fs::DirEntry) {
        let err_str = e.to_string();
        // register the error String alongside the path
        if !self.meta_errors.contains_key(err_str.as_str()) {
            self.meta_errors.insert(err_str.to_owned(), vec![]);
        }

        // get a mutable reference to the collection of paths associated with this error
        let errors_collection = self.meta_errors.get_mut(err_str.as_str()).unwrap();
        // create an owned path string from the current entry
        let path_string = entry.path().as_os_str().to_str().unwrap().to_string();
        // move the path into the errors collection
        errors_collection.push(path_string);
    }

    /// update the `rd_errors`` record
    fn register_rd_error(&mut self, e: &io::Error) {
        let err_str = e.to_string();
        // register the error String alongside the path
        if !self.rd_errors.contains_key(err_str.as_str()) {
            self.rd_errors.insert(err_str.to_owned(), vec![]);
        }

        // get a mutable reference to the collection of paths associated with this error
        let errors_collection = self.rd_errors.get_mut(err_str.as_str()).unwrap();
        let path_string = self.current_path.as_os_str().to_str().unwrap().to_string();
        // move the path into the errors collection
        errors_collection.push(path_string);
    }
}

// Implement Iterator for ReadDirRecursive
impl Iterator for ReadDirRecursive {
    // our Item is the same as the wrapped iter
    type Item = io::Result<fs::DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stats.iteration_started.is_none() {
            self.mark_start();
        }

        self.stats.total_iterations += 1;

        loop {
            match self.read_dir.next() {
                // entry found
                Some(Ok(entry)) => match entry.metadata() {
                    Ok(meta) => {
                        // if the entry is a directory we need to save it for later inspection
                        // and move on to the next entry in the iterator..
                        if meta.is_dir() {
                            self.digest_dir(entry);
                            // move to the next entry
                            continue;
                        }

                        // if the entry is not a directory we just assume that is a file
                        self.stats.total_files_consumed += 1;

                        // Something was found. Break the loop
                        // and return what was found..
                        break Some(Ok(entry));
                    }
                    // Error trying to obtain the entry's metadata.
                    // Since we won't know if the entry was a file or a directory we just return the
                    // error instead of the entry.
                    Err(e) => {
                        self.register_meta_error(&e, &entry);

                        // Something was found. Break the loop
                        // and return what was found..
                        break Some(Err(e));
                    }
                },
                // Entry found but is an error. No special treatment, we just return the error as is
                Some(Err(err)) => {
                    self.register_rd_error(&err);

                    // Something was found. Break the loop
                    // and return what was found..
                    break Some(Err(err));
                }
                // The current `read_dir` iterator reached the end (there are no more
                // files/entries in this directory).
                // We need to either move on to the next directory in the queue if there is any
                // or finish the iteration completely.
                None => {
                    // deal with the next pending directory (if any)
                    if let Some(dir_entry) = self.pending_dirs.pop() {
                        let entry_path = dir_entry.path();
                        match fs::read_dir(&entry_path) {
                            Ok(read_dir) => {
                                // throw away the consumed iterator and put the new one in his place
                                self.read_dir = read_dir;
                                // do the same for the path
                                self.current_path = entry_path;

                                // skip to the next iteration
                                continue;
                            }
                            Err(e) => {
                                self.register_rd_error(&e);

                                // Something was found. Stop the loop
                                // so we can return what was found
                                break Some(Err(e));
                            }
                        }
                    }

                    // there are no more directories to go through
                    break None;
                }
            }
        }
    }
}

/// Create an instance of [ReadDirRecursive] for the given path. This operation
/// will fail if [fs::read_dir] fails while trying to read from the specified path.
///
/// The same can be achieved by passing the path to the `new` method of [ReadDirRecursive].
///
/// Example:
/// ```
/// use readdir_recursive::ReadDirRecursive;
///
/// let rdr = ReadDirRecursive::new("/some/path").unwrap();
/// ```
pub fn read_dir_recursive<P: AsRef<path::Path>>(path: P) -> io::Result<ReadDirRecursive> {
    ReadDirRecursive::new(path)
}

/// Run this tests:
///
/// ```
/// cargo test -- --nocapture
/// ```
#[cfg(test)]
mod test {
    /// ```bash
    /// cargo test test::iterate -- --nocapture
    /// ```
    #[test]
    fn iterate() {
        let rdr = super::ReadDirRecursive::new(".").unwrap();

        for (i, entry_result) in rdr.enumerate() {
            println!("{} Found file: '{:?}'", i, entry_result.unwrap().path());
        }
    }

    /// ```bash
    /// cargo test test::print_stats -- --nocapture
    /// ```
    #[test]
    fn print_stats() {
        let mut rdr = super::ReadDirRecursive::new(".").unwrap();

        for (i, entry_result) in rdr.by_ref().enumerate() {
            println!("{} Found file: '{:?}'", i, entry_result.unwrap().path());
        }

        println!("{:?}", rdr.stats);
    }
}
