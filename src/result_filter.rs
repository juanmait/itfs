//! Export the `struct` [`ResultFilter`]. Map an iterator over items of type [Result<T>]
//! into one over items of type `T` by discarding [Err] variants.

/// Map an iterator over items of type [Result<T>] into one over items of type `T`
/// by discarding [Err] variants.
pub struct ResultFilter<T, E, I: Iterator<Item = Result<T, E>>>(pub I);

impl<T, E, I: Iterator<Item = Result<T, E>>> Iterator for ResultFilter<T, E, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(r) => {
                    if r.is_ok() {
                        break r.ok();
                    }

                    continue; // not okay
                }
                None => break None,
            }
        }
    }
}
