use std::fmt;

pub use crate::average::average;
pub use database::*;

pub trait Point {
    type Key: fmt::Debug + PartialEq + Clone;

    fn key(&self) -> &Self::Key;
    fn set_key(&mut self, key: Self::Key);
    fn value(&self) -> Option<f64>;
    fn set_value(&mut self, value: f64);
    fn interpolated(&self) -> bool;
    fn set_interpolated(&mut self);
}

impl<T: Clone + PartialEq + fmt::Debug> Point for (T, Option<f64>) {
    type Key = T;

    fn key(&self) -> &T {
        &self.0
    }
    fn set_key(&mut self, key: T) {
        self.0 = key;
    }
    fn value(&self) -> Option<f64> {
        self.1
    }
    fn set_value(&mut self, value: f64) {
        self.1 = Some(value);
    }
    fn interpolated(&self) -> bool {
        false
    }
    fn set_interpolated(&mut self) {
        // no-op
    }
}

impl<T: Clone + PartialEq + fmt::Debug> Point for (T, f64) {
    type Key = T;

    fn key(&self) -> &T {
        &self.0
    }
    fn set_key(&mut self, key: T) {
        self.0 = key;
    }
    fn value(&self) -> Option<f64> {
        Some(self.1)
    }
    fn set_value(&mut self, value: f64) {
        self.1 = value;
    }
    fn interpolated(&self) -> bool {
        false
    }
    fn set_interpolated(&mut self) {
        // no-op
    }
}
