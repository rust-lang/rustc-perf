#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize)]
pub enum Target {
    X86_64UnknownLinuxGnu
}

 impl Default for Target {
    fn default() -> Self {
        Self::X86_64UnknownLinuxGnu
   }
}
