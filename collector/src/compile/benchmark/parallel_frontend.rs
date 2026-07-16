use std::num::NonZeroU32;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct FrontendThreads(NonZeroU32);

impl FrontendThreads {
    pub fn new(val: u32) -> Self {
        Self(
            val.try_into()
                .map_err(|_| log::error!("Number of frontend threads cannot be zero"))
                .unwrap(),
        )
    }
    pub fn get(&self) -> u32 {
        self.0.get()
    }
    // Default thread counts for the parallel frontend
    pub fn default_threads_counts() -> Vec<FrontendThreads> {
        database::FrontendThreads::default_threads_counts()
            .iter()
            .map(|v| (*v).into())
            .collect()
    }
}

impl From<u32> for FrontendThreads {
    fn from(item: u32) -> Self {
        Self::new(item)
    }
}

impl From<database::FrontendThreads> for FrontendThreads {
    fn from(value: database::FrontendThreads) -> Self {
        FrontendThreads::new(value.0)
    }
}

impl From<FrontendThreads> for database::FrontendThreads {
    fn from(value: FrontendThreads) -> Self {
        database::FrontendThreads(value.get())
    }
}
