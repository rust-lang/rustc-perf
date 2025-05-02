#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - software trigger register"]
    pub swtrigr: crate::Reg<swtrigr::SWTRIGR_SPEC>,
    #[doc = "0x08 - channel1 12-bit right-aligned data holding register"]
    pub dhr12r1: crate::Reg<dhr12r1::DHR12R1_SPEC>,
    #[doc = "0x0c - channel1 12-bit left aligned data holding register"]
    pub dhr12l1: crate::Reg<dhr12l1::DHR12L1_SPEC>,
    #[doc = "0x10 - channel1 8-bit right aligned data holding register"]
    pub dhr8r1: crate::Reg<dhr8r1::DHR8R1_SPEC>,
    #[doc = "0x14 - channel2 12-bit right aligned data holding register"]
    pub dhr12r2: crate::Reg<dhr12r2::DHR12R2_SPEC>,
    #[doc = "0x18 - channel2 12-bit left aligned data holding register"]
    pub dhr12l2: crate::Reg<dhr12l2::DHR12L2_SPEC>,
    #[doc = "0x1c - channel2 8-bit right-aligned data holding register"]
    pub dhr8r2: crate::Reg<dhr8r2::DHR8R2_SPEC>,
    #[doc = "0x20 - Dual DAC 12-bit right-aligned data holding register"]
    pub dhr12rd: crate::Reg<dhr12rd::DHR12RD_SPEC>,
    #[doc = "0x24 - DUAL DAC 12-bit left aligned data holding register"]
    pub dhr12ld: crate::Reg<dhr12ld::DHR12LD_SPEC>,
    #[doc = "0x28 - DUAL DAC 8-bit right aligned data holding register"]
    pub dhr8rd: crate::Reg<dhr8rd::DHR8RD_SPEC>,
    #[doc = "0x2c - channel1 data output register"]
    pub dor1: crate::Reg<dor1::DOR1_SPEC>,
    #[doc = "0x30 - channel2 data output register"]
    pub dor2: crate::Reg<dor2::DOR2_SPEC>,
    #[doc = "0x34 - status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "SWTRIGR register accessor: an alias for `Reg<SWTRIGR_SPEC>`"]
pub type SWTRIGR = crate::Reg<swtrigr::SWTRIGR_SPEC>;
#[doc = "software trigger register"]
pub mod swtrigr;
#[doc = "DHR12R1 register accessor: an alias for `Reg<DHR12R1_SPEC>`"]
pub type DHR12R1 = crate::Reg<dhr12r1::DHR12R1_SPEC>;
#[doc = "channel1 12-bit right-aligned data holding register"]
pub mod dhr12r1;
#[doc = "DHR12L1 register accessor: an alias for `Reg<DHR12L1_SPEC>`"]
pub type DHR12L1 = crate::Reg<dhr12l1::DHR12L1_SPEC>;
#[doc = "channel1 12-bit left aligned data holding register"]
pub mod dhr12l1;
#[doc = "DHR8R1 register accessor: an alias for `Reg<DHR8R1_SPEC>`"]
pub type DHR8R1 = crate::Reg<dhr8r1::DHR8R1_SPEC>;
#[doc = "channel1 8-bit right aligned data holding register"]
pub mod dhr8r1;
#[doc = "DHR12R2 register accessor: an alias for `Reg<DHR12R2_SPEC>`"]
pub type DHR12R2 = crate::Reg<dhr12r2::DHR12R2_SPEC>;
#[doc = "channel2 12-bit right aligned data holding register"]
pub mod dhr12r2;
#[doc = "DHR12L2 register accessor: an alias for `Reg<DHR12L2_SPEC>`"]
pub type DHR12L2 = crate::Reg<dhr12l2::DHR12L2_SPEC>;
#[doc = "channel2 12-bit left aligned data holding register"]
pub mod dhr12l2;
#[doc = "DHR8R2 register accessor: an alias for `Reg<DHR8R2_SPEC>`"]
pub type DHR8R2 = crate::Reg<dhr8r2::DHR8R2_SPEC>;
#[doc = "channel2 8-bit right-aligned data holding register"]
pub mod dhr8r2;
#[doc = "DHR12RD register accessor: an alias for `Reg<DHR12RD_SPEC>`"]
pub type DHR12RD = crate::Reg<dhr12rd::DHR12RD_SPEC>;
#[doc = "Dual DAC 12-bit right-aligned data holding register"]
pub mod dhr12rd;
#[doc = "DHR12LD register accessor: an alias for `Reg<DHR12LD_SPEC>`"]
pub type DHR12LD = crate::Reg<dhr12ld::DHR12LD_SPEC>;
#[doc = "DUAL DAC 12-bit left aligned data holding register"]
pub mod dhr12ld;
#[doc = "DHR8RD register accessor: an alias for `Reg<DHR8RD_SPEC>`"]
pub type DHR8RD = crate::Reg<dhr8rd::DHR8RD_SPEC>;
#[doc = "DUAL DAC 8-bit right aligned data holding register"]
pub mod dhr8rd;
#[doc = "DOR1 register accessor: an alias for `Reg<DOR1_SPEC>`"]
pub type DOR1 = crate::Reg<dor1::DOR1_SPEC>;
#[doc = "channel1 data output register"]
pub mod dor1;
#[doc = "DOR2 register accessor: an alias for `Reg<DOR2_SPEC>`"]
pub type DOR2 = crate::Reg<dor2::DOR2_SPEC>;
#[doc = "channel2 data output register"]
pub mod dor2;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
