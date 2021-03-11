#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - status register"]
    pub sr: SR,
    #[doc = "0x08 - data input register"]
    pub din: DIN,
    #[doc = "0x0c - data output register"]
    pub dout: DOUT,
    #[doc = "0x10 - DMA control register"]
    pub dmacr: DMACR,
    #[doc = "0x14 - interrupt mask set/clear register"]
    pub imscr: IMSCR,
    #[doc = "0x18 - raw interrupt status register"]
    pub risr: RISR,
    #[doc = "0x1c - masked interrupt status register"]
    pub misr: MISR,
    #[doc = "0x20 - Cluster KEY%s, containing K?LR, K?RR"]
    pub key: [KEY; 4],
    #[doc = "0x40 - Cluster INIT%s, containing IV?LR, IV?RR"]
    pub init: [INIT; 2],
    #[doc = "0x50 - context swap register"]
    pub csgcmccmr: [CSGCMCCMR; 8],
    #[doc = "0x70 - context swap register"]
    pub csgcmr: [CSGCMR; 8],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct KEY {
    #[doc = "0x00 - key registers"]
    pub klr: self::key::KLR,
    #[doc = "0x04 - key registers"]
    pub krr: self::key::KRR,
}
#[doc = r"Register block"]
#[doc = "Cluster KEY%s, containing K?LR, K?RR"]
pub mod key;
#[doc = r"Register block"]
#[repr(C)]
pub struct INIT {
    #[doc = "0x00 - initialization vector registers"]
    pub ivlr: self::init::IVLR,
    #[doc = "0x04 - initialization vector registers"]
    pub ivrr: self::init::IVRR,
}
#[doc = r"Register block"]
#[doc = "Cluster INIT%s, containing IV?LR, IV?RR"]
pub mod init;
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "control register"]
pub mod cr;
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "status register"]
pub mod sr;
#[doc = "data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [din](din) module"]
pub type DIN = crate::Reg<u32, _DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIN;
#[doc = "`read()` method returns [din::R](din::R) reader structure"]
impl crate::Readable for DIN {}
#[doc = "`write(|w| ..)` method takes [din::W](din::W) writer structure"]
impl crate::Writable for DIN {}
#[doc = "data input register"]
pub mod din;
#[doc = "data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout](dout) module"]
pub type DOUT = crate::Reg<u32, _DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT;
#[doc = "`read()` method returns [dout::R](dout::R) reader structure"]
impl crate::Readable for DOUT {}
#[doc = "data output register"]
pub mod dout;
#[doc = "DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacr](dmacr) module"]
pub type DMACR = crate::Reg<u32, _DMACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACR;
#[doc = "`read()` method returns [dmacr::R](dmacr::R) reader structure"]
impl crate::Readable for DMACR {}
#[doc = "`write(|w| ..)` method takes [dmacr::W](dmacr::W) writer structure"]
impl crate::Writable for DMACR {}
#[doc = "DMA control register"]
pub mod dmacr;
#[doc = "interrupt mask set/clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imscr](imscr) module"]
pub type IMSCR = crate::Reg<u32, _IMSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMSCR;
#[doc = "`read()` method returns [imscr::R](imscr::R) reader structure"]
impl crate::Readable for IMSCR {}
#[doc = "`write(|w| ..)` method takes [imscr::W](imscr::W) writer structure"]
impl crate::Writable for IMSCR {}
#[doc = "interrupt mask set/clear register"]
pub mod imscr;
#[doc = "raw interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [risr](risr) module"]
pub type RISR = crate::Reg<u32, _RISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RISR;
#[doc = "`read()` method returns [risr::R](risr::R) reader structure"]
impl crate::Readable for RISR {}
#[doc = "raw interrupt status register"]
pub mod risr;
#[doc = "masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misr](misr) module"]
pub type MISR = crate::Reg<u32, _MISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISR;
#[doc = "`read()` method returns [misr::R](misr::R) reader structure"]
impl crate::Readable for MISR {}
#[doc = "masked interrupt status register"]
pub mod misr;
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcmccmr](csgcmccmr) module"]
pub type CSGCMCCMR = crate::Reg<u32, _CSGCMCCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCMCCMR;
#[doc = "`read()` method returns [csgcmccmr::R](csgcmccmr::R) reader structure"]
impl crate::Readable for CSGCMCCMR {}
#[doc = "`write(|w| ..)` method takes [csgcmccmr::W](csgcmccmr::W) writer structure"]
impl crate::Writable for CSGCMCCMR {}
#[doc = "context swap register"]
pub mod csgcmccmr;
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcmr](csgcmr) module"]
pub type CSGCMR = crate::Reg<u32, _CSGCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCMR;
#[doc = "`read()` method returns [csgcmr::R](csgcmr::R) reader structure"]
impl crate::Readable for CSGCMR {}
#[doc = "`write(|w| ..)` method takes [csgcmr::W](csgcmr::W) writer structure"]
impl crate::Writable for CSGCMR {}
#[doc = "context swap register"]
pub mod csgcmr;
