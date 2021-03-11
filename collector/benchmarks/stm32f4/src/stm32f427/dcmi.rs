#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr: CR,
    #[doc = "0x04 - status register"]
    pub sr: SR,
    #[doc = "0x08 - raw interrupt status register"]
    pub ris: RIS,
    #[doc = "0x0c - interrupt enable register"]
    pub ier: IER,
    #[doc = "0x10 - masked interrupt status register"]
    pub mis: MIS,
    #[doc = "0x14 - interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x18 - embedded synchronization code register"]
    pub escr: ESCR,
    #[doc = "0x1c - embedded synchronization unmask register"]
    pub esur: ESUR,
    #[doc = "0x20 - crop window start"]
    pub cwstrt: CWSTRT,
    #[doc = "0x24 - crop window size"]
    pub cwsize: CWSIZE,
    #[doc = "0x28 - data register"]
    pub dr: DR,
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "control register 1"]
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
#[doc = "raw interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "raw interrupt status register"]
pub mod ris;
#[doc = "interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "masked interrupt status register"]
pub mod mis;
#[doc = "interrupt clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "embedded synchronization code register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [escr](escr) module"]
pub type ESCR = crate::Reg<u32, _ESCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESCR;
#[doc = "`read()` method returns [escr::R](escr::R) reader structure"]
impl crate::Readable for ESCR {}
#[doc = "`write(|w| ..)` method takes [escr::W](escr::W) writer structure"]
impl crate::Writable for ESCR {}
#[doc = "embedded synchronization code register"]
pub mod escr;
#[doc = "embedded synchronization unmask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esur](esur) module"]
pub type ESUR = crate::Reg<u32, _ESUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESUR;
#[doc = "`read()` method returns [esur::R](esur::R) reader structure"]
impl crate::Readable for ESUR {}
#[doc = "`write(|w| ..)` method takes [esur::W](esur::W) writer structure"]
impl crate::Writable for ESUR {}
#[doc = "embedded synchronization unmask register"]
pub mod esur;
#[doc = "crop window start\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwstrt](cwstrt) module"]
pub type CWSTRT = crate::Reg<u32, _CWSTRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CWSTRT;
#[doc = "`read()` method returns [cwstrt::R](cwstrt::R) reader structure"]
impl crate::Readable for CWSTRT {}
#[doc = "`write(|w| ..)` method takes [cwstrt::W](cwstrt::W) writer structure"]
impl crate::Writable for CWSTRT {}
#[doc = "crop window start"]
pub mod cwstrt;
#[doc = "crop window size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwsize](cwsize) module"]
pub type CWSIZE = crate::Reg<u32, _CWSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CWSIZE;
#[doc = "`read()` method returns [cwsize::R](cwsize::R) reader structure"]
impl crate::Readable for CWSIZE {}
#[doc = "`write(|w| ..)` method takes [cwsize::W](cwsize::W) writer structure"]
impl crate::Writable for CWSIZE {}
#[doc = "crop window size"]
pub mod cwsize;
#[doc = "data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "data register"]
pub mod dr;
