#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IDCODE"]
    pub idcode: crate::Reg<idcode::IDCODE_SPEC>,
    #[doc = "0x04 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x08 - Debug MCU APB1 Freeze registe"]
    pub apb1_fz: crate::Reg<apb1_fz::APB1_FZ_SPEC>,
    #[doc = "0x0c - Debug MCU APB2 Freeze registe"]
    pub apb2_fz: crate::Reg<apb2_fz::APB2_FZ_SPEC>,
}
#[doc = "IDCODE register accessor: an alias for `Reg<IDCODE_SPEC>`"]
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
#[doc = "IDCODE"]
pub mod idcode;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "APB1_FZ register accessor: an alias for `Reg<APB1_FZ_SPEC>`"]
pub type APB1_FZ = crate::Reg<apb1_fz::APB1_FZ_SPEC>;
#[doc = "Debug MCU APB1 Freeze registe"]
pub mod apb1_fz;
#[doc = "APB2_FZ register accessor: an alias for `Reg<APB2_FZ_SPEC>`"]
pub type APB2_FZ = crate::Reg<apb2_fz::APB2_FZ_SPEC>;
#[doc = "Debug MCU APB2 Freeze registe"]
pub mod apb2_fz;
