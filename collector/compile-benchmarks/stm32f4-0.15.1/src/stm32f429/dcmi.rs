#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x08 - raw interrupt status register"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x0c - interrupt enable register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x10 - masked interrupt status register"]
    pub mis: crate::Reg<mis::MIS_SPEC>,
    #[doc = "0x14 - interrupt clear register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x18 - embedded synchronization code register"]
    pub escr: crate::Reg<escr::ESCR_SPEC>,
    #[doc = "0x1c - embedded synchronization unmask register"]
    pub esur: crate::Reg<esur::ESUR_SPEC>,
    #[doc = "0x20 - crop window start"]
    pub cwstrt: crate::Reg<cwstrt::CWSTRT_SPEC>,
    #[doc = "0x24 - crop window size"]
    pub cwsize: crate::Reg<cwsize::CWSIZE_SPEC>,
    #[doc = "0x28 - data register"]
    pub dr: crate::Reg<dr::DR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register 1"]
pub mod cr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "raw interrupt status register"]
pub mod ris;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "MIS register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "masked interrupt status register"]
pub mod mis;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "ESCR register accessor: an alias for `Reg<ESCR_SPEC>`"]
pub type ESCR = crate::Reg<escr::ESCR_SPEC>;
#[doc = "embedded synchronization code register"]
pub mod escr;
#[doc = "ESUR register accessor: an alias for `Reg<ESUR_SPEC>`"]
pub type ESUR = crate::Reg<esur::ESUR_SPEC>;
#[doc = "embedded synchronization unmask register"]
pub mod esur;
#[doc = "CWSTRT register accessor: an alias for `Reg<CWSTRT_SPEC>`"]
pub type CWSTRT = crate::Reg<cwstrt::CWSTRT_SPEC>;
#[doc = "crop window start"]
pub mod cwstrt;
#[doc = "CWSIZE register accessor: an alias for `Reg<CWSIZE_SPEC>`"]
pub type CWSIZE = crate::Reg<cwsize::CWSIZE_SPEC>;
#[doc = "crop window size"]
pub mod cwsize;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "data register"]
pub mod dr;
