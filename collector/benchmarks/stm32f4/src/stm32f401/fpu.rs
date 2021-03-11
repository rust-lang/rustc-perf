#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Floating-point context control register"]
    pub fpccr: FPCCR,
    #[doc = "0x04 - Floating-point context address register"]
    pub fpcar: FPCAR,
    #[doc = "0x08 - Floating-point status control register"]
    pub fpscr: FPSCR,
}
#[doc = "Floating-point context control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpccr](fpccr) module"]
pub type FPCCR = crate::Reg<u32, _FPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPCCR;
#[doc = "`read()` method returns [fpccr::R](fpccr::R) reader structure"]
impl crate::Readable for FPCCR {}
#[doc = "`write(|w| ..)` method takes [fpccr::W](fpccr::W) writer structure"]
impl crate::Writable for FPCCR {}
#[doc = "Floating-point context control register"]
pub mod fpccr;
#[doc = "Floating-point context address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpcar](fpcar) module"]
pub type FPCAR = crate::Reg<u32, _FPCAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPCAR;
#[doc = "`read()` method returns [fpcar::R](fpcar::R) reader structure"]
impl crate::Readable for FPCAR {}
#[doc = "`write(|w| ..)` method takes [fpcar::W](fpcar::W) writer structure"]
impl crate::Writable for FPCAR {}
#[doc = "Floating-point context address register"]
pub mod fpcar;
#[doc = "Floating-point status control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpscr](fpscr) module"]
pub type FPSCR = crate::Reg<u32, _FPSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPSCR;
#[doc = "`read()` method returns [fpscr::R](fpscr::R) reader structure"]
impl crate::Readable for FPSCR {}
#[doc = "`write(|w| ..)` method takes [fpscr::W](fpscr::W) writer structure"]
impl crate::Writable for FPSCR {}
#[doc = "Floating-point status control register"]
pub mod fpscr;
