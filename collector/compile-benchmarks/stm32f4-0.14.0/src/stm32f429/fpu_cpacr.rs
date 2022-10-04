#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Coprocessor access control register"]
    pub cpacr: crate::Reg<cpacr::CPACR_SPEC>,
}
#[doc = "CPACR register accessor: an alias for `Reg<CPACR_SPEC>`"]
pub type CPACR = crate::Reg<cpacr::CPACR_SPEC>;
#[doc = "Coprocessor access control register"]
pub mod cpacr;
