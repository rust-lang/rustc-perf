use crate::db::Point;

/// This aggregates interpolated iterators.
///
/// It could support non-interpolated iterators too but that's a bit more work
/// and not currently used anyway.
pub fn average<I>(iterators: Vec<I>) -> Average<I>
where
    I: Iterator,
    I::Item: Point,
{
    Average {
        iterators,
        is_first: true,
    }
}

pub struct Average<I> {
    iterators: Vec<I>,
    is_first: bool,
}

impl<I> Iterator for Average<I>
where
    I: Iterator,
    I::Item: Point,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sum = 0.0;
        let mut count = 0;

        let mut i = 0;
        let mut first = None::<I::Item>;
        let mut removed = false;
        // replace with drain_filter when it stabilizes
        while i != self.iterators.len() {
            match self.iterators[i].next() {
                None => {
                    removed = true;
                    self.iterators.remove(i);
                }
                Some(point) => {
                    count += 1;
                    sum += point.value().expect("present");
                    i += 1;
                    if let Some(t) = &mut first {
                        if point.interpolated() {
                            // Interpolated is like a taint
                            t.set_interpolated();
                        }
                        assert_eq!(*t.key(), *point.key());
                    } else {
                        first = Some(point);
                    }
                }
            }
        }

        if removed && !self.iterators.is_empty() && !self.is_first {
            panic!("Not all iterators of the same length");
        }
        self.is_first = false;

        match first {
            None => {
                assert!(self.iterators.is_empty());
                None
            }
            Some(mut t) => {
                t.set_value(sum / (count as f64));
                Some(t)
            }
        }
    }
}
