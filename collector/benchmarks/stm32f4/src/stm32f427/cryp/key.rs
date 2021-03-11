#[doc = "key registers\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [klr](klr) module"]
pub type KLR = crate::Reg<u32, _KLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KLR;
#[doc = "`write(|w| ..)` method takes [klr::W](klr::W) writer structure"]
impl crate::Writable for KLR {}
#[doc = "key registers"]
pub mod klr;
#[doc = "key registers\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [krr](krr) module"]
pub type KRR = crate::Reg<u32, _KRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KRR;
#[doc = "`write(|w| ..)` method takes [krr::W](krr::W) writer structure"]
impl crate::Writable for KRR {}
#[doc = "key registers"]
pub mod krr;
