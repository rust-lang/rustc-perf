use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Parallel(pub u32);

impl Parallel {
    // Default parallel frontend options
    pub fn default_opts() -> Vec<Parallel> {
        database::Parallel::default_opts()
            .iter()
            .map(|v| (*v).into())
            .collect()
    }

    pub fn par_n(self) -> String {
        database::Parallel(self.0).par_n()
    }
}

impl From<u32> for Parallel {
    fn from(item: u32) -> Self {
        Self(item)
    }
}

impl From<database::Parallel> for Parallel {
    fn from(value: database::Parallel) -> Self {
        Parallel(value.0)
    }
}

impl From<Parallel> for database::Parallel {
    fn from(value: Parallel) -> Self {
        database::Parallel(value.0)
    }
}

impl fmt::Display for Parallel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-Zthreads={}", self.0)
    }
}
