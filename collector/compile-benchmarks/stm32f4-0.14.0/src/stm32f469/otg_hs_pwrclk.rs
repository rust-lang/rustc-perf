#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power and clock gating control register"]
    pub pcgcr: crate::Reg<pcgcr::PCGCR_SPEC>,
}
#[doc = "PCGCR register accessor: an alias for `Reg<PCGCR_SPEC>`"]
pub type PCGCR = crate::Reg<pcgcr::PCGCR_SPEC>;
#[doc = "Power and clock gating control register"]
pub mod pcgcr;
