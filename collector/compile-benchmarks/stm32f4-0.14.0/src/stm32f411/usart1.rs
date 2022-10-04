#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x04 - Data register"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    #[doc = "0x08 - Baud rate register"]
    pub brr: crate::Reg<brr::BRR_SPEC>,
    #[doc = "0x0c - Control register 1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x10 - Control register 2"]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    #[doc = "0x14 - Control register 3"]
    pub cr3: crate::Reg<cr3::CR3_SPEC>,
    #[doc = "0x18 - Guard time and prescaler register"]
    pub gtpr: crate::Reg<gtpr::GTPR_SPEC>,
}
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data register"]
pub mod dr;
#[doc = "BRR register accessor: an alias for `Reg<BRR_SPEC>`"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "Baud rate register"]
pub mod brr;
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Control register 1"]
pub mod cr1;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "Control register 2"]
pub mod cr2;
#[doc = "CR3 register accessor: an alias for `Reg<CR3_SPEC>`"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "Control register 3"]
pub mod cr3;
#[doc = "GTPR register accessor: an alias for `Reg<GTPR_SPEC>`"]
pub type GTPR = crate::Reg<gtpr::GTPR_SPEC>;
#[doc = "Guard time and prescaler register"]
pub mod gtpr;
