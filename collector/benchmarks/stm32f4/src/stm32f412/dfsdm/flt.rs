#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](cr2) module"]
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
#[doc = "`read()` method returns [cr2::R](cr2::R) reader structure"]
impl crate::Readable for CR2 {}
#[doc = "`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure"]
impl crate::Writable for CR2 {}
#[doc = "control register 1"]
pub mod cr2;
#[doc = "interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "interrupt and status register"]
pub mod isr;
#[doc = "interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "interrupt flag clear register"]
pub mod icr;
#[doc = "injected channel group selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jchgr](jchgr) module"]
pub type JCHGR = crate::Reg<u32, _JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JCHGR;
#[doc = "`read()` method returns [jchgr::R](jchgr::R) reader structure"]
impl crate::Readable for JCHGR {}
#[doc = "`write(|w| ..)` method takes [jchgr::W](jchgr::W) writer structure"]
impl crate::Writable for JCHGR {}
#[doc = "injected channel group selection register"]
pub mod jchgr;
#[doc = "filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](fcr) module"]
pub type FCR = crate::Reg<u32, _FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCR;
#[doc = "`read()` method returns [fcr::R](fcr::R) reader structure"]
impl crate::Readable for FCR {}
#[doc = "`write(|w| ..)` method takes [fcr::W](fcr::W) writer structure"]
impl crate::Writable for FCR {}
#[doc = "filter control register"]
pub mod fcr;
#[doc = "data register for injected group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdatar](jdatar) module"]
pub type JDATAR = crate::Reg<u32, _JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JDATAR;
#[doc = "`read()` method returns [jdatar::R](jdatar::R) reader structure"]
impl crate::Readable for JDATAR {}
#[doc = "data register for injected group"]
pub mod jdatar;
#[doc = "data register for the regular channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdatar](rdatar) module"]
pub type RDATAR = crate::Reg<u32, _RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDATAR;
#[doc = "`read()` method returns [rdatar::R](rdatar::R) reader structure"]
impl crate::Readable for RDATAR {}
#[doc = "data register for the regular channel"]
pub mod rdatar;
#[doc = "analog watchdog high threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awhtr](awhtr) module"]
pub type AWHTR = crate::Reg<u32, _AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWHTR;
#[doc = "`read()` method returns [awhtr::R](awhtr::R) reader structure"]
impl crate::Readable for AWHTR {}
#[doc = "`write(|w| ..)` method takes [awhtr::W](awhtr::W) writer structure"]
impl crate::Writable for AWHTR {}
#[doc = "analog watchdog high threshold register"]
pub mod awhtr;
#[doc = "analog watchdog low threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awltr](awltr) module"]
pub type AWLTR = crate::Reg<u32, _AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWLTR;
#[doc = "`read()` method returns [awltr::R](awltr::R) reader structure"]
impl crate::Readable for AWLTR {}
#[doc = "`write(|w| ..)` method takes [awltr::W](awltr::W) writer structure"]
impl crate::Writable for AWLTR {}
#[doc = "analog watchdog low threshold register"]
pub mod awltr;
#[doc = "analog watchdog status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awsr](awsr) module"]
pub type AWSR = crate::Reg<u32, _AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWSR;
#[doc = "`read()` method returns [awsr::R](awsr::R) reader structure"]
impl crate::Readable for AWSR {}
#[doc = "analog watchdog status register"]
pub mod awsr;
#[doc = "analog watchdog clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awcfr](awcfr) module"]
pub type AWCFR = crate::Reg<u32, _AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWCFR;
#[doc = "`read()` method returns [awcfr::R](awcfr::R) reader structure"]
impl crate::Readable for AWCFR {}
#[doc = "`write(|w| ..)` method takes [awcfr::W](awcfr::W) writer structure"]
impl crate::Writable for AWCFR {}
#[doc = "analog watchdog clear flag register"]
pub mod awcfr;
#[doc = "Extremes detector maximum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exmax](exmax) module"]
pub type EXMAX = crate::Reg<u32, _EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXMAX;
#[doc = "`read()` method returns [exmax::R](exmax::R) reader structure"]
impl crate::Readable for EXMAX {}
#[doc = "Extremes detector maximum register"]
pub mod exmax;
#[doc = "Extremes detector minimum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exmin](exmin) module"]
pub type EXMIN = crate::Reg<u32, _EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXMIN;
#[doc = "`read()` method returns [exmin::R](exmin::R) reader structure"]
impl crate::Readable for EXMIN {}
#[doc = "Extremes detector minimum register"]
pub mod exmin;
#[doc = "conversion timer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnvtimr](cnvtimr) module"]
pub type CNVTIMR = crate::Reg<u32, _CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNVTIMR;
#[doc = "`read()` method returns [cnvtimr::R](cnvtimr::R) reader structure"]
impl crate::Readable for CNVTIMR {}
#[doc = "conversion timer register"]
pub mod cnvtimr;
