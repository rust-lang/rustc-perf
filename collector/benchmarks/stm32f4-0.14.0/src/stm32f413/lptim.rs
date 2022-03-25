#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt and Status Register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x04 - Interrupt Clear Register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x08 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x0c - Configuration Register"]
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
    #[doc = "0x10 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x14 - Compare Register"]
    pub cmp: crate::Reg<cmp::CMP_SPEC>,
    #[doc = "0x18 - Autoreload Register"]
    pub arr: crate::Reg<arr::ARR_SPEC>,
    #[doc = "0x1c - Counter Register"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
}
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt and Status Register"]
pub mod isr;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "CFGR register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "Configuration Register"]
pub mod cfgr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "CMP register accessor: an alias for `Reg<CMP_SPEC>`"]
pub type CMP = crate::Reg<cmp::CMP_SPEC>;
#[doc = "Compare Register"]
pub mod cmp;
#[doc = "ARR register accessor: an alias for `Reg<ARR_SPEC>`"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "Autoreload Register"]
pub mod arr;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter Register"]
pub mod cnt;
