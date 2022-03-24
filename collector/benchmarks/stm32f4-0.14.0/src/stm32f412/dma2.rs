#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - low interrupt status register"]
    pub lisr: crate::Reg<lisr::LISR_SPEC>,
    #[doc = "0x04 - high interrupt status register"]
    pub hisr: crate::Reg<hisr::HISR_SPEC>,
    #[doc = "0x08 - low interrupt flag clear register"]
    pub lifcr: crate::Reg<lifcr::LIFCR_SPEC>,
    #[doc = "0x0c - high interrupt flag clear register"]
    pub hifcr: crate::Reg<hifcr::HIFCR_SPEC>,
    #[doc = "0x10..0xd0 - Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
    pub st: [ST; 8],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct ST {
    #[doc = "0x00 - stream x configuration register"]
    pub cr: crate::Reg<self::st::cr::CR_SPEC>,
    #[doc = "0x04 - stream x number of data register"]
    pub ndtr: crate::Reg<self::st::ndtr::NDTR_SPEC>,
    #[doc = "0x08 - stream x peripheral address register"]
    pub par: crate::Reg<self::st::par::PAR_SPEC>,
    #[doc = "0x0c - stream x memory 0 address register"]
    pub m0ar: crate::Reg<self::st::m0ar::M0AR_SPEC>,
    #[doc = "0x10 - stream x memory 1 address register"]
    pub m1ar: crate::Reg<self::st::m1ar::M1AR_SPEC>,
    #[doc = "0x14 - stream x FIFO control register"]
    pub fcr: crate::Reg<self::st::fcr::FCR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Stream cluster: S?CR, S?NDTR, S?M0AR, S?M1AR and S?FCR registers"]
pub mod st;
#[doc = "LISR register accessor: an alias for `Reg<LISR_SPEC>`"]
pub type LISR = crate::Reg<lisr::LISR_SPEC>;
#[doc = "low interrupt status register"]
pub mod lisr;
#[doc = "HISR register accessor: an alias for `Reg<HISR_SPEC>`"]
pub type HISR = crate::Reg<hisr::HISR_SPEC>;
#[doc = "high interrupt status register"]
pub mod hisr;
#[doc = "LIFCR register accessor: an alias for `Reg<LIFCR_SPEC>`"]
pub type LIFCR = crate::Reg<lifcr::LIFCR_SPEC>;
#[doc = "low interrupt flag clear register"]
pub mod lifcr;
#[doc = "HIFCR register accessor: an alias for `Reg<HIFCR_SPEC>`"]
pub type HIFCR = crate::Reg<hifcr::HIFCR_SPEC>;
#[doc = "high interrupt flag clear register"]
pub mod hifcr;
