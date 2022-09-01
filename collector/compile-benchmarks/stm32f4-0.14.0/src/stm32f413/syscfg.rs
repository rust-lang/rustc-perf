#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - memory remap register"]
    pub memrm: crate::Reg<memrm::MEMRM_SPEC>,
    #[doc = "0x04 - peripheral mode configuration register"]
    pub pmc: crate::Reg<pmc::PMC_SPEC>,
    #[doc = "0x08 - external interrupt configuration register 1"]
    pub exticr1: crate::Reg<exticr1::EXTICR1_SPEC>,
    #[doc = "0x0c - external interrupt configuration register 2"]
    pub exticr2: crate::Reg<exticr2::EXTICR2_SPEC>,
    #[doc = "0x10 - external interrupt configuration register 3"]
    pub exticr3: crate::Reg<exticr3::EXTICR3_SPEC>,
    #[doc = "0x14 - external interrupt configuration register 4"]
    pub exticr4: crate::Reg<exticr4::EXTICR4_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - Compensation cell control register"]
    pub cmpcr: crate::Reg<cmpcr::CMPCR_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0x2c - Configuration register"]
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
}
#[doc = "MEMRM register accessor: an alias for `Reg<MEMRM_SPEC>`"]
pub type MEMRM = crate::Reg<memrm::MEMRM_SPEC>;
#[doc = "memory remap register"]
pub mod memrm;
#[doc = "PMC register accessor: an alias for `Reg<PMC_SPEC>`"]
pub type PMC = crate::Reg<pmc::PMC_SPEC>;
#[doc = "peripheral mode configuration register"]
pub mod pmc;
#[doc = "EXTICR1 register accessor: an alias for `Reg<EXTICR1_SPEC>`"]
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
#[doc = "external interrupt configuration register 1"]
pub mod exticr1;
#[doc = "EXTICR2 register accessor: an alias for `Reg<EXTICR2_SPEC>`"]
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
#[doc = "external interrupt configuration register 2"]
pub mod exticr2;
#[doc = "EXTICR3 register accessor: an alias for `Reg<EXTICR3_SPEC>`"]
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
#[doc = "external interrupt configuration register 3"]
pub mod exticr3;
#[doc = "EXTICR4 register accessor: an alias for `Reg<EXTICR4_SPEC>`"]
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
#[doc = "external interrupt configuration register 4"]
pub mod exticr4;
#[doc = "CMPCR register accessor: an alias for `Reg<CMPCR_SPEC>`"]
pub type CMPCR = crate::Reg<cmpcr::CMPCR_SPEC>;
#[doc = "Compensation cell control register"]
pub mod cmpcr;
#[doc = "CFGR register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "Configuration register"]
pub mod cfgr;
