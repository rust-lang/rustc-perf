use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct AutoVec<T: Default>(Vec<T>);

impl<T: Default> AutoVec<T> {
    pub fn new() -> Self { AutoVec(Vec::new()) }
    fn ensure_len(&mut self, len: usize) {
        while len > self.0.len() {
            self.0.push(T::default());
        }
    }
    pub fn into_boxed_slice(mut self, len: usize) -> Box<[T]> {
        self.ensure_len(len);
        self.0.into_boxed_slice()
    }
    pub fn at(&mut self, i: usize) -> &mut T {
        self.ensure_len(i + 1);
        &mut self.0[i]
    }
}

impl<T: Default> Default for AutoVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl <T: Default> Deref for AutoVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl <T: Default> IntoIterator for AutoVec<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::AutoVec;

    #[test]
    fn empty() {
        let v: AutoVec<i32> = AutoVec::new();
        assert_eq!(v.0, vec![]);
    }

    #[test]
    fn one() {
        let mut v = AutoVec::new();
        *v.at(0) = 17;
        assert_eq!(v.0, vec![17]);
    }

    #[test]
    fn two() {
        let mut v = AutoVec::new();
        *v.at(1) = 17;
        assert_eq!(*v, vec![0, 17]);
        *v.at(0) = 23;
        assert_eq!(v.0, vec![23, 17]);
    }

    #[test]
    fn sadness() {
        let mut v = AutoVec::new();
        let _: &mut i32 = v.at(5);
        assert_eq!(v.0, vec![0,0,0,0,0,0]);
    }

    #[test]
    fn autoderef() {
        let mut v = AutoVec::new();
        assert_eq!(v.len(), 0);
        *v.at(3) = 1000;
        assert_eq!(v.iter().collect::<Vec<_>>(), vec![&0, &0, &0, &1000]);
    }

    #[test]
    fn move_iter() {
        let mut v = AutoVec::new();
        *v.at(3) = 1000;
        assert_eq!(v.into_iter().collect::<Vec<_>>(), vec![0, 0, 0, 1000]);
    }
}
