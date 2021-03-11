#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power and clock gating control register"]
    pub pcgcr: PCGCR,
}
#[doc = "Power and clock gating control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcgcr](pcgcr) module"]
pub type PCGCR = crate::Reg<u32, _PCGCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCGCR;
#[doc = "`read()` method returns [pcgcr::R](pcgcr::R) reader structure"]
impl crate::Readable for PCGCR {}
#[doc = "`write(|w| ..)` method takes [pcgcr::W](pcgcr::W) writer structure"]
impl crate::Writable for PCGCR {}
#[doc = "Power and clock gating control register"]
pub mod pcgcr;
