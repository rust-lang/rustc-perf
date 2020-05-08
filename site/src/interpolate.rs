//! Provides an "interpolating" iterator atop of another iterator.
//!
//! This does not do linear interpolation but rather just keeps the last seen
//! value going until the next point and so forth. For perf's purposes, we
//! mostly want to avoid dropping or improving summary performance when data
//! points are missing as that's misleading and noisy, and this works well for
//! that.

use crate::db::Point;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Interpolated {
    No,
    Yes,
}

impl Interpolated {
    pub fn is_interpolated(self) -> bool {
        self == Interpolated::Yes
    }
}

impl<P> Point for (P, Interpolated)
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
        self.1.is_interpolated()
    }
    fn set_interpolated(&mut self) {
        self.1 = Interpolated::Yes;
    }
}

pub struct Interpolate<I>
where
    I: Iterator,
{
    iterator: I,
    last_seen: Option<f64>,

    // When we need to seek forward at the start, we store things in here.
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
    type Item = (I::Item, Interpolated);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut item) = self.consumed.pop() {
            item.set_value(self.last_seen.unwrap());
            return Some((item, Interpolated::Yes));
        }

        let mut item = self.iterator.next()?;

        match item.value() {
            Some(pt) => {
                self.last_seen = Some(pt);
                return Some((item, Interpolated::No));
            }
            None => {
                if let Some(last) = self.last_seen {
                    item.set_value(last);
                    return Some((item, Interpolated::Yes));
                }

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
                                    return Some((item, Interpolated::Yes));
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
