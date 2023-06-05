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
                    sum += point
                        .value()
                        .expect("Uninterpolated iterators are not supported");
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

#[cfg(test)]
mod tests {
    use super::average;

    #[test]
    fn test_no_interpolation_average() {
        // Test that averaging works without interpolation.
        use crate::db::Point;

        let v = vec![
            vec![("a", 0.0), ("b", 200.0)],
            vec![("a", 100.0), ("b", 300.0)],
        ];

        let iterators: Vec<_> = v.into_iter().map(|v| v.into_iter()).collect();
        let mut average = average(iterators);

        let a = average.next().unwrap();
        assert_eq!(a, ("a", 50.0));
        assert!(!a.interpolated());

        let b = average.next().unwrap();
        assert_eq!(b, ("b", 250.0));
        assert!(!b.interpolated());

        assert!(average.next().is_none());
    }

    #[test]
    fn test_interpolation_average() {
        // Test that averaging works with interpolation.
        use crate::db::Point;
        use crate::interpolate::{Interpolate, IsInterpolated};

        let v = vec![
            vec![("a", Some(0.0)), ("b", Some(200.0))],
            vec![("a", Some(100.0)), ("b", None)],
        ];

        let iterators: Vec<_> = v
            .into_iter()
            .map(|v| Interpolate::new(v.into_iter()))
            .collect();

        let mut average = average(iterators);

        let a = average.next().unwrap();
        assert_eq!(a, (("a", Some(50.0)), IsInterpolated::No));
        assert!(!a.interpolated());

        let b = average.next().unwrap();
        assert_eq!(b, (("b", Some(150.0)), IsInterpolated::Yes));
        assert!(b.interpolated());

        assert!(average.next().is_none());
    }

    #[test]
    #[should_panic(expected = "Uninterpolated iterators are not supported")]
    fn test_uninterpolated_iterator() {
        let v = vec![vec![("a", Some(1.0)), ("b", Some(2.0))], vec![("a", None)]];
        let iterators: Vec<_> = v.into_iter().map(|v| v.into_iter()).collect();
        let _ = average(iterators).next();
    }

    #[test]
    #[should_panic(expected = "Not all iterators of the same length")]
    fn test_unequal_length_iterators() {
        let v = vec![vec![("a", 1.0), ("b", 2.0)], vec![("a", 3.0)]];
        let iterators: Vec<_> = v.into_iter().map(|v| v.into_iter()).collect();
        for _ in average(iterators) {}
    }
}
