//! Export the `struct` [`ErrorCollector`]. Map an iterator over items of type `Result<T>`
//! into one over items of type `T` by discarding [Err] variants.

/// Map an iterator over items of type `Result<T>` into one over items of type `T`
/// by discarding [Err] variants.
pub struct ErrorCollector<'a, T, E, I: Iterator<Item = Result<T, E>>>(pub I, pub &'a mut Vec<E>);

impl<T, E, I: Iterator<Item = Result<T, E>>> Iterator for ErrorCollector<'_, T, E, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(r) => match r {
                    Ok(t) => {
                        break Some(t);
                    }
                    Err(e) => {
                        self.1.push(e);
                        continue;
                    }
                },
                None => break None,
            }
        }
    }
}
