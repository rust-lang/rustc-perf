//! Provides an "interpolating" iterator atop of another iterator.
//!
//! This does not do linear interpolation but rather just keeps the last seen
//! value going until the next point and so forth. For perf's purposes, we
//! mostly want to avoid dropping or improving summary performance when data
//! points are missing as that's misleading and noisy, and this works well for
//! that.
//!
//! As an example:
//! Given a series with some missing data `[1, 2, ?, 4]`,
//! this iterator yields `[1, 2, 2, 4]`.

use crate::selector::Point;

/// Whether a point has been interpolated or not
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IsInterpolated {
    No,
    Yes,
}

impl IsInterpolated {
    pub fn as_bool(self) -> bool {
        self == IsInterpolated::Yes
    }
}

impl<P> Point for (P, IsInterpolated)
where
    P: Point,
{
    type Key = P::Key;

    fn key(&self) -> &P::Key {
        self.0.key()
    }
    fn set_key(&mut self, key: P::Key) {
        self.0.set_key(key)
    }
    fn value(&self) -> Option<f64> {
        self.0.value()
    }
    fn set_value(&mut self, value: f64) {
        self.0.set_value(value)
    }
    fn interpolated(&self) -> bool {
        self.1.as_bool()
    }
    fn set_interpolated(&mut self) {
        self.1 = IsInterpolated::Yes;
    }
}

/// The interpolated iterator
pub struct Interpolate<I>
where
    I: Iterator,
{
    /// The base iterator we're interpolating
    iterator: I,
    /// The last seen point which will be used for interpolation
    last_seen: Option<f64>,
    /// When we need to seek forward at the start, we store things in here.
    consumed: Vec<I::Item>,
}

impl<I> Interpolate<I>
where
    I: Iterator,
    I::Item: Point,
{
    pub fn new(iterator: I) -> Self {
        Interpolate {
            iterator,
            last_seen: None,
            consumed: Vec::new(),
        }
    }
}

impl<I> Iterator for Interpolate<I>
where
    I: Iterator,
    I::Item: Point,
{
    type Item = (I::Item, IsInterpolated);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut item) = self.consumed.pop() {
            item.set_value(self.last_seen.unwrap());
            let interpolation = if self.consumed.is_empty() {
                IsInterpolated::No
            } else {
                IsInterpolated::Yes
            };
            return Some((item, interpolation));
        }

        let mut item = self.iterator.next()?;

        match item.value() {
            Some(pt) => {
                self.last_seen = Some(pt);
                Some((item, IsInterpolated::No))
            }
            None => {
                if let Some(last) = self.last_seen {
                    item.set_value(last);
                    return Some((item, IsInterpolated::Yes));
                }

                self.consumed.push(item);

                // We are at the start of the iterator, and do not currently
                // have a point. We need to seek forward until we hit a point,
                // and then back-propagate that point.

                loop {
                    match self.iterator.next() {
                        Some(item) => {
                            match item.value() {
                                None => self.consumed.push(item),
                                Some(pt) => {
                                    self.consumed.push(item);
                                    self.last_seen = Some(pt);
                                    // We flip the vector as we want to consume from the
                                    // beginning
                                    self.consumed.reverse();

                                    let mut item = self.consumed.pop().unwrap();
                                    item.set_value(self.last_seen.unwrap());
                                    return Some((item, IsInterpolated::Yes));
                                }
                            }
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

#[cfg(test)]
mod tests {
    use super::{Interpolate, IsInterpolated};

    #[test]
    fn test_no_interpolation() {
        let v = vec![("a", 1.0), ("b", 2.0)];
        let mut iter = Interpolate::new(v.into_iter());

        assert_eq!(iter.next().unwrap(), (("a", 1.0), IsInterpolated::No));
        assert_eq!(iter.next().unwrap(), (("b", 2.0), IsInterpolated::No));
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_leading_interpolation() {
        let v = vec![("a", None), ("b", None), ("c", Some(3.0)), ("d", Some(4.0))];
        let mut iter = Interpolate::new(v.into_iter());

        assert_eq!(
            iter.next().unwrap(),
            (("a", Some(3.0)), IsInterpolated::Yes)
        );
        assert_eq!(
            iter.next().unwrap(),
            (("b", Some(3.0)), IsInterpolated::Yes)
        );
        assert_eq!(iter.next().unwrap(), (("c", Some(3.0)), IsInterpolated::No));
        assert_eq!(iter.next().unwrap(), (("d", Some(4.0)), IsInterpolated::No));
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_inner_interpolation() {
        let v = vec![
            ("a", Some(1.0)),
            ("b", Some(2.0)),
            ("c", None),
            ("d", None),
            ("e", Some(5.0)),
            ("f", Some(6.0)),
        ];
        let mut iter = Interpolate::new(v.into_iter());

        assert_eq!(iter.next().unwrap(), (("a", Some(1.0)), IsInterpolated::No));
        assert_eq!(iter.next().unwrap(), (("b", Some(2.0)), IsInterpolated::No));
        assert_eq!(
            iter.next().unwrap(),
            (("c", Some(2.0)), IsInterpolated::Yes)
        );
        assert_eq!(
            iter.next().unwrap(),
            (("d", Some(2.0)), IsInterpolated::Yes)
        );
        assert_eq!(iter.next().unwrap(), (("e", Some(5.0)), IsInterpolated::No));
        assert_eq!(iter.next().unwrap(), (("f", Some(6.0)), IsInterpolated::No));
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_trailing_interpolation() {
        let v = vec![("a", Some(1.0)), ("b", Some(2.0)), ("c", None), ("d", None)];
        let mut iter = Interpolate::new(v.into_iter());

        assert_eq!(iter.next().unwrap(), (("a", Some(1.0)), IsInterpolated::No));
        assert_eq!(iter.next().unwrap(), (("b", Some(2.0)), IsInterpolated::No));
        assert_eq!(
            iter.next().unwrap(),
            (("c", Some(2.0)), IsInterpolated::Yes)
        );
        assert_eq!(
            iter.next().unwrap(),
            (("d", Some(2.0)), IsInterpolated::Yes)
        );
        assert!(iter.next().is_none());
    }
}
