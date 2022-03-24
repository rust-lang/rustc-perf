#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - data input register"]
    pub din: crate::Reg<din::DIN_SPEC>,
    #[doc = "0x08 - start register"]
    pub str: crate::Reg<str::STR_SPEC>,
    #[doc = "0x0c..0x20 - digest registers"]
    pub hr: [crate::Reg<hr::HR_SPEC>; 5],
    #[doc = "0x20 - interrupt enable register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x24 - status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    _reserved6: [u8; 0xd0],
    #[doc = "0xf8..0x1d0 - context swap registers"]
    pub csr: [crate::Reg<csr::CSR_SPEC>; 54],
    _reserved7: [u8; 0x0140],
    #[doc = "0x310..0x330 - HASH digest register"]
    pub hash_hr: [crate::Reg<hash_hr::HASH_HR_SPEC>; 8],
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "DIN register accessor: an alias for `Reg<DIN_SPEC>`"]
pub type DIN = crate::Reg<din::DIN_SPEC>;
#[doc = "data input register"]
pub mod din;
#[doc = "STR register accessor: an alias for `Reg<STR_SPEC>`"]
pub type STR = crate::Reg<str::STR_SPEC>;
#[doc = "start register"]
pub mod str;
#[doc = "HR register accessor: an alias for `Reg<HR_SPEC>`"]
pub type HR = crate::Reg<hr::HR_SPEC>;
#[doc = "digest registers"]
pub mod hr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "interrupt enable register"]
pub mod imr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "context swap registers"]
pub mod csr;
#[doc = "HASH_HR register accessor: an alias for `Reg<HASH_HR_SPEC>`"]
pub type HASH_HR = crate::Reg<hash_hr::HASH_HR_SPEC>;
#[doc = "HASH digest register"]
pub mod hash_hr;
