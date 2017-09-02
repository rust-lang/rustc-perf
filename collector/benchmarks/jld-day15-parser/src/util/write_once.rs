use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WriteOnce<T>(Option<T>);
impl<T> WriteOnce<T> {
    pub fn new() -> Self { WriteOnce(None) }
    pub fn into_inner(self) -> Option<T> { self.0 }
    pub fn get(&self) -> Option<&T> { self.as_ref() }
    pub fn set(&mut self, val: T) -> Result<(), T> {
        if let Some(_ptr) = self.0.as_mut() {
            return Err(val);
        }
        self.0 = Some(val);
        Ok(())
    }
}

impl<T> Default for WriteOnce<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Deref for WriteOnce<T> {
    type Target = Option<T>;
    fn deref(&self) -> &Option<T> {
        &self.0
    }
}
impl<T> IntoIterator for WriteOnce<T> {
    type Item = T;
    type IntoIter = <Option<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::WriteOnce;

    #[test]
    fn get_and_set() {
        assert_eq!(WriteOnce::new().into_inner(), None::<i32>);
        let mut wo = WriteOnce::new();
        assert_eq!(wo.get(), None);
        assert_eq!(wo.set(17), Ok(()));
        assert_eq!(wo.get(), Some(&17));
        assert_eq!(wo.set(99), Err(99));
        assert_eq!(wo.into_inner(), Some(17));
    }

    #[test]
    fn iter_via_deref() {
        let mut wo = WriteOnce::new();
        for _ in wo.iter() {
            panic!();
        }
        wo.set(17).unwrap();
        let mut got_thing = false;
        for r in wo.iter() {
            assert_eq!(*r, 17);
            got_thing = true;
        }
        assert!(got_thing);
    }

    #[test]
    fn iter_none() {
        let wo = WriteOnce::new();
        assert_eq!(wo.into_iter().collect::<Vec<i32>>(), vec![]);
    }

    #[test]
    fn iter_some() {
        let mut wo = WriteOnce::new();
        wo.set(17).unwrap();
        assert_eq!(wo.into_iter().collect::<Vec<_>>(), vec![17]);
    }
}
