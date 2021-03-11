#[doc = "channel configuration y register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](cfgr1) module"]
pub type CFGR1 = crate::Reg<u32, _CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR1;
#[doc = "`read()` method returns [cfgr1::R](cfgr1::R) reader structure"]
impl crate::Readable for CFGR1 {}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](cfgr1::W) writer structure"]
impl crate::Writable for CFGR1 {}
#[doc = "channel configuration y register"]
pub mod cfgr1;
#[doc = "channel configuration y register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](cfgr2) module"]
pub type CFGR2 = crate::Reg<u32, _CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR2;
#[doc = "`read()` method returns [cfgr2::R](cfgr2::R) reader structure"]
impl crate::Readable for CFGR2 {}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](cfgr2::W) writer structure"]
impl crate::Writable for CFGR2 {}
#[doc = "channel configuration y register"]
pub mod cfgr2;
#[doc = "analog watchdog and short-circuit detector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awscdr](awscdr) module"]
pub type AWSCDR = crate::Reg<u32, _AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWSCDR;
#[doc = "`read()` method returns [awscdr::R](awscdr::R) reader structure"]
impl crate::Readable for AWSCDR {}
#[doc = "`write(|w| ..)` method takes [awscdr::W](awscdr::W) writer structure"]
impl crate::Writable for AWSCDR {}
#[doc = "analog watchdog and short-circuit detector register"]
pub mod awscdr;
#[doc = "channel watchdog filter data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdatr](wdatr) module"]
pub type WDATR = crate::Reg<u32, _WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDATR;
#[doc = "`read()` method returns [wdatr::R](wdatr::R) reader structure"]
impl crate::Readable for WDATR {}
#[doc = "`write(|w| ..)` method takes [wdatr::W](wdatr::W) writer structure"]
impl crate::Writable for WDATR {}
#[doc = "channel watchdog filter data register"]
pub mod wdatr;
#[doc = "channel data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datinr](datinr) module"]
pub type DATINR = crate::Reg<u32, _DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATINR;
#[doc = "`read()` method returns [datinr::R](datinr::R) reader structure"]
impl crate::Readable for DATINR {}
#[doc = "`write(|w| ..)` method takes [datinr::W](datinr::W) writer structure"]
impl crate::Writable for DATINR {}
#[doc = "channel data input register"]
pub mod datinr;
