#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software trigger interrupt register"]
    pub stir: crate::Reg<stir::STIR_SPEC>,
}
#[doc = "STIR register accessor: an alias for `Reg<STIR_SPEC>`"]
pub type STIR = crate::Reg<stir::STIR_SPEC>;
#[doc = "Software trigger interrupt register"]
pub mod stir;
