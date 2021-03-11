#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - low interrupt status register"]
    pub lisr: LISR,
    #[doc = "0x04 - high interrupt status register"]
    pub hisr: HISR,
    #[doc = "0x08 - low interrupt flag clear register"]
    pub lifcr: LIFCR,
    #[doc = "0x0c - high interrupt flag clear register"]
    pub hifcr: HIFCR,
    #[doc = "0x10 - Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
    pub st: [ST; 8],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct ST {
    #[doc = "0x00 - stream x configuration register"]
    pub cr: self::st::CR,
    #[doc = "0x04 - stream x number of data register"]
    pub ndtr: self::st::NDTR,
    #[doc = "0x08 - stream x peripheral address register"]
    pub par: self::st::PAR,
    #[doc = "0x0c - stream x memory 0 address register"]
    pub m0ar: self::st::M0AR,
    #[doc = "0x10 - stream x memory 1 address register"]
    pub m1ar: self::st::M1AR,
    #[doc = "0x14 - stream x FIFO control register"]
    pub fcr: self::st::FCR,
}
#[doc = r"Register block"]
#[doc = "Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
pub mod st;
#[doc = "low interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lisr](lisr) module"]
pub type LISR = crate::Reg<u32, _LISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LISR;
#[doc = "`read()` method returns [lisr::R](lisr::R) reader structure"]
impl crate::Readable for LISR {}
#[doc = "low interrupt status register"]
pub mod lisr;
#[doc = "high interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hisr](hisr) module"]
pub type HISR = crate::Reg<u32, _HISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HISR;
#[doc = "`read()` method returns [hisr::R](hisr::R) reader structure"]
impl crate::Readable for HISR {}
#[doc = "high interrupt status register"]
pub mod hisr;
#[doc = "low interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lifcr](lifcr) module"]
pub type LIFCR = crate::Reg<u32, _LIFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LIFCR;
#[doc = "`write(|w| ..)` method takes [lifcr::W](lifcr::W) writer structure"]
impl crate::Writable for LIFCR {}
#[doc = "low interrupt flag clear register"]
pub mod lifcr;
#[doc = "high interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hifcr](hifcr) module"]
pub type HIFCR = crate::Reg<u32, _HIFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HIFCR;
#[doc = "`write(|w| ..)` method takes [hifcr::W](hifcr::W) writer structure"]
impl crate::Writable for HIFCR {}
#[doc = "high interrupt flag clear register"]
pub mod hifcr;
