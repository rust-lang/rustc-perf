#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt mask register (EXTI_IMR)"]
    pub imr: IMR,
    #[doc = "0x04 - Event mask register (EXTI_EMR)"]
    pub emr: EMR,
    #[doc = "0x08 - Rising Trigger selection register (EXTI_RTSR)"]
    pub rtsr: RTSR,
    #[doc = "0x0c - Falling Trigger selection register (EXTI_FTSR)"]
    pub ftsr: FTSR,
    #[doc = "0x10 - Software interrupt event register (EXTI_SWIER)"]
    pub swier: SWIER,
    #[doc = "0x14 - Pending register (EXTI_PR)"]
    pub pr: PR,
}
#[doc = "Interrupt mask register (EXTI_IMR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "`write(|w| ..)` method takes [imr::W](imr::W) writer structure"]
impl crate::Writable for IMR {}
#[doc = "Interrupt mask register (EXTI_IMR)"]
pub mod imr;
#[doc = "Event mask register (EXTI_EMR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr](emr) module"]
pub type EMR = crate::Reg<u32, _EMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMR;
#[doc = "`read()` method returns [emr::R](emr::R) reader structure"]
impl crate::Readable for EMR {}
#[doc = "`write(|w| ..)` method takes [emr::W](emr::W) writer structure"]
impl crate::Writable for EMR {}
#[doc = "Event mask register (EXTI_EMR)"]
pub mod emr;
#[doc = "Rising Trigger selection register (EXTI_RTSR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtsr](rtsr) module"]
pub type RTSR = crate::Reg<u32, _RTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTSR;
#[doc = "`read()` method returns [rtsr::R](rtsr::R) reader structure"]
impl crate::Readable for RTSR {}
#[doc = "`write(|w| ..)` method takes [rtsr::W](rtsr::W) writer structure"]
impl crate::Writable for RTSR {}
#[doc = "Rising Trigger selection register (EXTI_RTSR)"]
pub mod rtsr;
#[doc = "Falling Trigger selection register (EXTI_FTSR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr](ftsr) module"]
pub type FTSR = crate::Reg<u32, _FTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTSR;
#[doc = "`read()` method returns [ftsr::R](ftsr::R) reader structure"]
impl crate::Readable for FTSR {}
#[doc = "`write(|w| ..)` method takes [ftsr::W](ftsr::W) writer structure"]
impl crate::Writable for FTSR {}
#[doc = "Falling Trigger selection register (EXTI_FTSR)"]
pub mod ftsr;
#[doc = "Software interrupt event register (EXTI_SWIER)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier](swier) module"]
pub type SWIER = crate::Reg<u32, _SWIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWIER;
#[doc = "`read()` method returns [swier::R](swier::R) reader structure"]
impl crate::Readable for SWIER {}
#[doc = "`write(|w| ..)` method takes [swier::W](swier::W) writer structure"]
impl crate::Writable for SWIER {}
#[doc = "Software interrupt event register (EXTI_SWIER)"]
pub mod swier;
#[doc = "Pending register (EXTI_PR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr](pr) module"]
pub type PR = crate::Reg<u32, _PR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR;
#[doc = "`read()` method returns [pr::R](pr::R) reader structure"]
impl crate::Readable for PR {}
#[doc = "`write(|w| ..)` method takes [pr::W](pr::W) writer structure"]
impl crate::Writable for PR {}
#[doc = "Pending register (EXTI_PR)"]
pub mod pr;
