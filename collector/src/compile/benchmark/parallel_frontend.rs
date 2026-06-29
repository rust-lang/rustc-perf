#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FrontendThreads(pub u32);

impl FrontendThreads {
    // Default thread counts for the parallel frontend
    pub fn default_opts() -> Vec<FrontendThreads> {
        database::FrontendThreads::default_opts()
            .iter()
            .map(|v| (*v).into())
            .collect()
    }

    pub fn par_n(self) -> String {
        database::FrontendThreads(self.0).par_n()
    }
}

impl From<u32> for FrontendThreads {
    fn from(item: u32) -> Self {
        Self(item)
    }
}

impl From<database::FrontendThreads> for FrontendThreads {
    fn from(value: database::FrontendThreads) -> Self {
        FrontendThreads(value.0)
    }
}

impl From<FrontendThreads> for database::FrontendThreads {
    fn from(value: FrontendThreads) -> Self {
        database::FrontendThreads(value.0)
    }
}
