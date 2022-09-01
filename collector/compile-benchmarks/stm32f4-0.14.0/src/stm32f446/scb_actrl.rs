#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Auxiliary control register"]
    pub actrl: crate::Reg<actrl::ACTRL_SPEC>,
}
#[doc = "ACTRL register accessor: an alias for `Reg<ACTRL_SPEC>`"]
pub type ACTRL = crate::Reg<actrl::ACTRL_SPEC>;
#[doc = "Auxiliary control register"]
pub mod actrl;
