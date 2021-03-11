#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CEC control register"]
    pub cec_cr: CEC_CR,
    #[doc = "0x04 - CEC configuration register"]
    pub cec_cfgr: CEC_CFGR,
    #[doc = "0x08 - CEC Tx data register"]
    pub cec_txdr: CEC_TXDR,
    #[doc = "0x0c - CEC Rx Data Register"]
    pub cec_rxdr: CEC_RXDR,
    #[doc = "0x10 - CEC Interrupt and Status Register"]
    pub cec_isr: CEC_ISR,
    #[doc = "0x14 - CEC interrupt enable register"]
    pub cec_ier: CEC_IER,
}
#[doc = "CEC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_cr](cec_cr) module"]
pub type CEC_CR = crate::Reg<u32, _CEC_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEC_CR;
#[doc = "`read()` method returns [cec_cr::R](cec_cr::R) reader structure"]
impl crate::Readable for CEC_CR {}
#[doc = "`write(|w| ..)` method takes [cec_cr::W](cec_cr::W) writer structure"]
impl crate::Writable for CEC_CR {}
#[doc = "CEC control register"]
pub mod cec_cr;
#[doc = "CEC configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_cfgr](cec_cfgr) module"]
pub type CEC_CFGR = crate::Reg<u32, _CEC_CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEC_CFGR;
#[doc = "`read()` method returns [cec_cfgr::R](cec_cfgr::R) reader structure"]
impl crate::Readable for CEC_CFGR {}
#[doc = "`write(|w| ..)` method takes [cec_cfgr::W](cec_cfgr::W) writer structure"]
impl crate::Writable for CEC_CFGR {}
#[doc = "CEC configuration register"]
pub mod cec_cfgr;
#[doc = "CEC Tx data register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_txdr](cec_txdr) module"]
pub type CEC_TXDR = crate::Reg<u32, _CEC_TXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEC_TXDR;
#[doc = "`write(|w| ..)` method takes [cec_txdr::W](cec_txdr::W) writer structure"]
impl crate::Writable for CEC_TXDR {}
#[doc = "CEC Tx data register"]
pub mod cec_txdr;
#[doc = "CEC Rx Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_rxdr](cec_rxdr) module"]
pub type CEC_RXDR = crate::Reg<u32, _CEC_RXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEC_RXDR;
#[doc = "`read()` method returns [cec_rxdr::R](cec_rxdr::R) reader structure"]
impl crate::Readable for CEC_RXDR {}
#[doc = "CEC Rx Data Register"]
pub mod cec_rxdr;
#[doc = "CEC Interrupt and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_isr](cec_isr) module"]
pub type CEC_ISR = crate::Reg<u32, _CEC_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEC_ISR;
#[doc = "`read()` method returns [cec_isr::R](cec_isr::R) reader structure"]
impl crate::Readable for CEC_ISR {}
#[doc = "`write(|w| ..)` method takes [cec_isr::W](cec_isr::W) writer structure"]
impl crate::Writable for CEC_ISR {}
#[doc = "CEC Interrupt and Status Register"]
pub mod cec_isr;
#[doc = "CEC interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_ier](cec_ier) module"]
pub type CEC_IER = crate::Reg<u32, _CEC_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEC_IER;
#[doc = "`read()` method returns [cec_ier::R](cec_ier::R) reader structure"]
impl crate::Readable for CEC_IER {}
#[doc = "`write(|w| ..)` method takes [cec_ier::W](cec_ier::W) writer structure"]
impl crate::Writable for CEC_IER {}
#[doc = "CEC interrupt enable register"]
pub mod cec_ier;
