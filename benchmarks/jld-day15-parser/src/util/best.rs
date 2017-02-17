use std::borrow::{Borrow,ToOwned};

// Maybe this is a bad idea and I should just write `Copy` directly but:
pub trait Score: Copy { }
impl<T: Copy> Score for T { }

pub trait Cmp<S: Score> {
    fn better(&self, old: S, shiny: S) -> bool;
}

pub struct Smallest;
impl<S: Score + Ord> Cmp<S> for Smallest {
    fn better(&self, old: S, shiny: S) -> bool { shiny < old }
}

pub struct Largest;
impl<S: Score + Ord> Cmp<S> for Largest {
    fn better(&self, old: S, shiny: S) -> bool { shiny > old }
}

pub struct Best<S: Score, T, C: Cmp<S>> {
    best: Option<(S, T)>,
    cmp: C,
}
impl<S: Score, T, C: Cmp<S>> Best<S, T, C> {
    pub fn new(cmp: C) -> Self {
        Best { best: None, cmp: cmp }
    }
    pub fn add<Q: ?Sized>(&mut self, offer: S, thing: &Q)
        where T: Borrow<Q>,
              Q: ToOwned<Owned = T> {
        let better = match &self.best {
            &None => true,
            &Some((so_far, _)) => self.cmp.better(so_far, offer)
        };
        if better {
            self.best = Some((offer, thing.to_owned()));
        }
    }
    pub fn get(&self) -> Option<(S, &T)> {
        self.best.as_ref().map(|&(s, ref t)| (s, t))
    }
    pub fn unwrap(self) -> (S, T) {
        self.best.unwrap()
    }
    pub fn expect(self, msg: &str) -> (S, T) {
        self.best.expect(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::{Best, Smallest, Largest, Cmp};

    #[test]
    fn empty() {
        let b: Best<i32, i32, _> = Best::new(Smallest);
        assert!(b.get().is_none());
    }

    #[test] #[should_panic(expected = "Option::unwrap")]
    fn empty_unwrap() {
        let b: Best<i32, i32, _> = Best::new(Smallest);
        let _ = b.unwrap();
    }
    #[test] #[should_panic(expected = "insert message here")]
    fn empty_expect() {
        let b: Best<i32, i32, _> = Best::new(Smallest);
        let _ = b.expect("insert message here");
    }
    
    #[test]
    fn add_one() {
        let mut b = Best::new(Smallest);
        b.add(17, &23);
        assert_eq!(b.get(), Some((17, &23)));
        assert_eq!(b.unwrap(), (17, 23));
    }

    #[test]
    fn add_better() {
        let mut b = Best::new(Smallest);
        b.add(17, &23);
        b.add(5, &46);
        assert_eq!(b.get(), Some((5, &46)));
        assert_eq!(b.unwrap(), (5, 46));
    }
    
    #[test]
    fn add_worse() {
        let mut b = Best::new(Smallest);
        b.add(17, &23);
        b.add(42, &46);
        assert_eq!(b.get(), Some((17, &23)));
        assert_eq!(b.unwrap(), (17, 23));
    }

    #[test]
    fn add_equal() {
        let mut b = Best::new(Smallest);
        b.add(17, &23);
        b.add(17, &46);
        assert_eq!(b.get(), Some((17, &23)));
        assert_eq!(b.unwrap(), (17, 23));
    }

    #[test]
    fn larger() {
        let mut b = Best::new(Largest);
        b.add(17, &23);
        b.add(5, &46);
        assert_eq!(b.get(), Some((17, &23)));
        assert_eq!(b.unwrap(), (17, 23));
    }

    struct MoreBits;
    impl Cmp<u32> for MoreBits {
        fn better(&self, old: u32, shiny: u32) -> bool {
            shiny.count_ones() > old.count_ones()
        }
    }

    #[test]
    fn custom() {
        let mut b = Best::new(MoreBits);
        assert_eq!(b.get(), None);
        b.add(8, &23);
        assert_eq!(b.get(), Some((8, &23)));
        b.add(7, &17);
        assert_eq!(b.get(), Some((7, &17)));
        b.add(6, &42);
        assert_eq!(b.get(), Some((7, &17)));
        b.add(5, &42);
        assert_eq!(b.get(), Some((7, &17)));
        b.add(9, &42);
        assert_eq!(b.get(), Some((7, &17)));
    }
}
