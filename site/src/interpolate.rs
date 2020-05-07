//! Provides an "interpolating" iterator atop of another iterator.
//!
//! This does not do linear interpolation but rather just keeps the last seen
//! value going until the next point and so forth. For perf's purposes, we
//! mostly want to avoid dropping or improving summary performance when data
//! points are missing as that's misleading and noisy, and this works well for
//! that.

pub struct Interpolate<I, T>
where
    I: Iterator<Item = (T, Option<f64>)>,
{
    iterator: I,
    last_seen: Option<f64>,

    // When we need to seek forward at the start, we store things in here.
    consumed: Vec<T>,
}

impl<I, T> Interpolate<I, T>
where
    I: Iterator<Item = (T, Option<f64>)>,
{
    pub fn new(iterator: I) -> Self {
        Interpolate {
            iterator,
            last_seen: None,
            consumed: Vec::new(),
        }
    }
}

impl<I, T> Iterator for Interpolate<I, T>
where
    I: Iterator<Item = (T, Option<f64>)>,
{
    type Item = (T, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.consumed.pop() {
            return Some((item, self.last_seen.unwrap()));
        }

        let (item, pt) = self.iterator.next()?;

        match pt {
            Some(pt) => {
                self.last_seen = Some(pt);
                return Some((item, pt));
            }
            None => {
                if let Some(last) = self.last_seen {
                    return Some((item, last));
                }

                // We are at the start of the iterator, and do not currently
                // have a point. We need to seek forward until we hit a point,
                // and then back-propagate that point.

                loop {
                    match self.iterator.next() {
                        Some((item, None)) => self.consumed.push(item),
                        Some((item, Some(pt))) => {
                            self.consumed.push(item);
                            self.last_seen = Some(pt);
                            // We flip the vector as we want to consume from the
                            // beginning
                            self.consumed.reverse();

                            let item = self.consumed.pop().unwrap();
                            return Some((item, self.last_seen.unwrap()));
                        }
                        None => {
                            // There were no elements in this iterator.
                            return None;
                        }
                    }
                }
            }
        }
    }
}
