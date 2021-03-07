#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - Control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - Own address register 1"]
    pub oar1: OAR1,
    #[doc = "0x0c - Own address register 2"]
    pub oar2: OAR2,
    #[doc = "0x10 - Timing register"]
    pub timingr: TIMINGR,
    #[doc = "0x14 - Status register 1"]
    pub timeoutr: TIMEOUTR,
    #[doc = "0x18 - Interrupt and Status register"]
    pub isr: ISR,
    #[doc = "0x1c - Interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x20 - PEC register"]
    pub pecr: PECR,
    #[doc = "0x24 - Receive data register"]
    pub rxdr: RXDR,
    #[doc = "0x28 - Transmit data register"]
    pub txdr: TXDR,
}
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure"]
impl crate::Writable for CR1 {}
#[doc = "Control register 1"]
pub mod cr1;
#[doc = "Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](cr2) module"]
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
#[doc = "`read()` method returns [cr2::R](cr2::R) reader structure"]
impl crate::Readable for CR2 {}
#[doc = "`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure"]
impl crate::Writable for CR2 {}
#[doc = "Control register 2"]
pub mod cr2;
#[doc = "Own address register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oar1](oar1) module"]
pub type OAR1 = crate::Reg<u32, _OAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OAR1;
#[doc = "`read()` method returns [oar1::R](oar1::R) reader structure"]
impl crate::Readable for OAR1 {}
#[doc = "`write(|w| ..)` method takes [oar1::W](oar1::W) writer structure"]
impl crate::Writable for OAR1 {}
#[doc = "Own address register 1"]
pub mod oar1;
#[doc = "Own address register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oar2](oar2) module"]
pub type OAR2 = crate::Reg<u32, _OAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OAR2;
#[doc = "`read()` method returns [oar2::R](oar2::R) reader structure"]
impl crate::Readable for OAR2 {}
#[doc = "`write(|w| ..)` method takes [oar2::W](oar2::W) writer structure"]
impl crate::Writable for OAR2 {}
#[doc = "Own address register 2"]
pub mod oar2;
#[doc = "Timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timingr](timingr) module"]
pub type TIMINGR = crate::Reg<u32, _TIMINGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMINGR;
#[doc = "`read()` method returns [timingr::R](timingr::R) reader structure"]
impl crate::Readable for TIMINGR {}
#[doc = "`write(|w| ..)` method takes [timingr::W](timingr::W) writer structure"]
impl crate::Writable for TIMINGR {}
#[doc = "Timing register"]
pub mod timingr;
#[doc = "Status register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timeoutr](timeoutr) module"]
pub type TIMEOUTR = crate::Reg<u32, _TIMEOUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEOUTR;
#[doc = "`read()` method returns [timeoutr::R](timeoutr::R) reader structure"]
impl crate::Readable for TIMEOUTR {}
#[doc = "`write(|w| ..)` method takes [timeoutr::W](timeoutr::W) writer structure"]
impl crate::Writable for TIMEOUTR {}
#[doc = "Status register 1"]
pub mod timeoutr;
#[doc = "Interrupt and Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "`write(|w| ..)` method takes [isr::W](isr::W) writer structure"]
impl crate::Writable for ISR {}
#[doc = "Interrupt and Status register"]
pub mod isr;
#[doc = "Interrupt clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "Interrupt clear register"]
pub mod icr;
#[doc = "PEC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pecr](pecr) module"]
pub type PECR = crate::Reg<u32, _PECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PECR;
#[doc = "`read()` method returns [pecr::R](pecr::R) reader structure"]
impl crate::Readable for PECR {}
#[doc = "PEC register"]
pub mod pecr;
#[doc = "Receive data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdr](rxdr) module"]
pub type RXDR = crate::Reg<u32, _RXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDR;
#[doc = "`read()` method returns [rxdr::R](rxdr::R) reader structure"]
impl crate::Readable for RXDR {}
#[doc = "Receive data register"]
pub mod rxdr;
#[doc = "Transmit data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdr](txdr) module"]
pub type TXDR = crate::Reg<u32, _TXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDR;
#[doc = "`read()` method returns [txdr::R](txdr::R) reader structure"]
impl crate::Readable for TXDR {}
#[doc = "`write(|w| ..)` method takes [txdr::W](txdr::W) writer structure"]
impl crate::Writable for TXDR {}
#[doc = "Transmit data register"]
pub mod txdr;
