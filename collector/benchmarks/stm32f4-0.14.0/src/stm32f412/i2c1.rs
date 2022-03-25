#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x04 - Control register 2"]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    #[doc = "0x08 - Own address register 1"]
    pub oar1: crate::Reg<oar1::OAR1_SPEC>,
    #[doc = "0x0c - Own address register 2"]
    pub oar2: crate::Reg<oar2::OAR2_SPEC>,
    #[doc = "0x10 - Data register"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    #[doc = "0x14 - Status register 1"]
    pub sr1: crate::Reg<sr1::SR1_SPEC>,
    #[doc = "0x18 - Status register 2"]
    pub sr2: crate::Reg<sr2::SR2_SPEC>,
    #[doc = "0x1c - Clock control register"]
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
    #[doc = "0x20 - TRISE register"]
    pub trise: crate::Reg<trise::TRISE_SPEC>,
    #[doc = "0x24 - FLTR register"]
    pub fltr: crate::Reg<fltr::FLTR_SPEC>,
}
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Control register 1"]
pub mod cr1;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "Control register 2"]
pub mod cr2;
#[doc = "OAR1 register accessor: an alias for `Reg<OAR1_SPEC>`"]
pub type OAR1 = crate::Reg<oar1::OAR1_SPEC>;
#[doc = "Own address register 1"]
pub mod oar1;
#[doc = "OAR2 register accessor: an alias for `Reg<OAR2_SPEC>`"]
pub type OAR2 = crate::Reg<oar2::OAR2_SPEC>;
#[doc = "Own address register 2"]
pub mod oar2;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data register"]
pub mod dr;
#[doc = "SR1 register accessor: an alias for `Reg<SR1_SPEC>`"]
pub type SR1 = crate::Reg<sr1::SR1_SPEC>;
#[doc = "Status register 1"]
pub mod sr1;
#[doc = "SR2 register accessor: an alias for `Reg<SR2_SPEC>`"]
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
#[doc = "Status register 2"]
pub mod sr2;
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Clock control register"]
pub mod ccr;
#[doc = "TRISE register accessor: an alias for `Reg<TRISE_SPEC>`"]
pub type TRISE = crate::Reg<trise::TRISE_SPEC>;
#[doc = "TRISE register"]
pub mod trise;
#[doc = "FLTR register accessor: an alias for `Reg<FLTR_SPEC>`"]
pub type FLTR = crate::Reg<fltr::FLTR_SPEC>;
#[doc = "FLTR register"]
pub mod fltr;
