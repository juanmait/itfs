//! Export the `struct` [`PathReRoot`]. Given an iterator over items of type [PathBuf]
//! rewrite the root of those that contains a given prefix, by using another one
//! given as a replacement.

use std::path::{Path, PathBuf, StripPrefixError};

/// function that performs the prefix replacement
pub fn path_re_root<P>(path: P, find: P, replace_by: P) -> Result<PathBuf, StripPrefixError>
where
    P: AsRef<Path>,
{
    match path.as_ref().strip_prefix(find) {
        Ok(rest) => Ok(replace_by.as_ref().join(rest)),
        Err(e) => Err(e),
    }
}

/// Given an iterator over items of type [PathBuf] rewrite the root of those that
/// contains a given prefix, by using another one given as a replacement.
pub struct PathReRoot<I, P>
where
    P: AsRef<Path>,
    I: Iterator<Item = PathBuf>,
{
    pub inner_iter: I,
    pub strip_prefix: P,
    pub replace_by: P,
}

impl<I, P> Iterator for PathReRoot<I, P>
where
    I: Iterator<Item = PathBuf>,
    P: AsRef<Path>,
{
    /// Item is a tuple where the first element is the original path
    /// and the second one is the result of the re root operation
    type Item = (PathBuf, Result<PathBuf, StripPrefixError>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner_iter.next() {
            Some(p) => {
                let rooted = path_re_root(
                    p.as_path(),
                    self.strip_prefix.as_ref(),
                    self.replace_by.as_ref(),
                );

                Some((p, rooted))
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};

    use super::path_re_root;

    struct Subject<P: AsRef<Path>> {
        path: P,
        strip_prefix: P,
        replace_prefix: P,
        expect: Option<PathBuf>,
    }

    #[test]
    fn re_root_fn() {
        let subjects = [
            Subject {
                path: "/a/b/c/d",
                strip_prefix: "/a/b",
                replace_prefix: "/x/y",
                expect: Some(PathBuf::from("/x/y/c/d")),
            },
            // prefix can be erased from the target
            Subject {
                path: "/a/b/c/d",
                strip_prefix: "/a/b",
                replace_prefix: "",
                expect: Some(PathBuf::from("c/d")),
            },
            // not a valid prefix
            Subject {
                path: "/a/b/c/d",
                strip_prefix: "/c/d",
                replace_prefix: "",
                expect: None,
            },
            // mismatch: target path is relative, given prefix is absolute
            Subject {
                path: "./c/d",
                strip_prefix: "/c/d",
                replace_prefix: "",
                expect: None,
            },
            // mismatch: target path is absolute, given prefix is relative
            Subject {
                path: "/c/d",
                strip_prefix: ".c/d",
                replace_prefix: "",
                expect: None,
            },
        ];

        for subject in subjects {
            let res = path_re_root(subject.path, subject.strip_prefix, subject.replace_prefix);

            match subject.expect.is_some() {
                true => assert_eq!(res, Result::Ok(subject.expect.unwrap())),
                false => assert!(res.is_err()),
            }
        }
    }
}
