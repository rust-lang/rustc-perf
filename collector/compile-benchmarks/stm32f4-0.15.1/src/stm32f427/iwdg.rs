#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key register"]
    pub kr: crate::Reg<kr::KR_SPEC>,
    #[doc = "0x04 - Prescaler register"]
    pub pr: crate::Reg<pr::PR_SPEC>,
    #[doc = "0x08 - Reload register"]
    pub rlr: crate::Reg<rlr::RLR_SPEC>,
    #[doc = "0x0c - Status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
}
#[doc = "KR register accessor: an alias for `Reg<KR_SPEC>`"]
pub type KR = crate::Reg<kr::KR_SPEC>;
#[doc = "Key register"]
pub mod kr;
#[doc = "PR register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "Prescaler register"]
pub mod pr;
#[doc = "RLR register accessor: an alias for `Reg<RLR_SPEC>`"]
pub type RLR = crate::Reg<rlr::RLR_SPEC>;
#[doc = "Reload register"]
pub mod rlr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
