#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x08 - data input register"]
    pub din: crate::Reg<din::DIN_SPEC>,
    #[doc = "0x0c - data output register"]
    pub dout: crate::Reg<dout::DOUT_SPEC>,
    #[doc = "0x10 - DMA control register"]
    pub dmacr: crate::Reg<dmacr::DMACR_SPEC>,
    #[doc = "0x14 - interrupt mask set/clear register"]
    pub imscr: crate::Reg<imscr::IMSCR_SPEC>,
    #[doc = "0x18 - raw interrupt status register"]
    pub risr: crate::Reg<risr::RISR_SPEC>,
    #[doc = "0x1c - masked interrupt status register"]
    pub misr: crate::Reg<misr::MISR_SPEC>,
    #[doc = "0x20..0x40 - Cluster KEY%s, containing K?LR, K?RR"]
    pub key: [KEY; 4],
    #[doc = "0x40..0x50 - Cluster INIT%s, containing IV?LR, IV?RR"]
    pub init: [INIT; 2],
    #[doc = "0x50..0x70 - context swap register"]
    pub csgcmccmr: [crate::Reg<csgcmccmr::CSGCMCCMR_SPEC>; 8],
    #[doc = "0x70..0x90 - context swap register"]
    pub csgcmr: [crate::Reg<csgcmr::CSGCMR_SPEC>; 8],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct KEY {
    #[doc = "0x00 - key registers"]
    pub klr: crate::Reg<self::key::klr::KLR_SPEC>,
    #[doc = "0x04 - key registers"]
    pub krr: crate::Reg<self::key::krr::KRR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Cluster KEY%s, containing K?LR, K?RR"]
pub mod key;
#[doc = r"Register block"]
#[repr(C)]
pub struct INIT {
    #[doc = "0x00 - initialization vector registers"]
    pub ivlr: crate::Reg<self::init::ivlr::IVLR_SPEC>,
    #[doc = "0x04 - initialization vector registers"]
    pub ivrr: crate::Reg<self::init::ivrr::IVRR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Cluster INIT%s, containing IV?LR, IV?RR"]
pub mod init;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "DIN register accessor: an alias for `Reg<DIN_SPEC>`"]
pub type DIN = crate::Reg<din::DIN_SPEC>;
#[doc = "data input register"]
pub mod din;
#[doc = "DOUT register accessor: an alias for `Reg<DOUT_SPEC>`"]
pub type DOUT = crate::Reg<dout::DOUT_SPEC>;
#[doc = "data output register"]
pub mod dout;
#[doc = "DMACR register accessor: an alias for `Reg<DMACR_SPEC>`"]
pub type DMACR = crate::Reg<dmacr::DMACR_SPEC>;
#[doc = "DMA control register"]
pub mod dmacr;
#[doc = "IMSCR register accessor: an alias for `Reg<IMSCR_SPEC>`"]
pub type IMSCR = crate::Reg<imscr::IMSCR_SPEC>;
#[doc = "interrupt mask set/clear register"]
pub mod imscr;
#[doc = "RISR register accessor: an alias for `Reg<RISR_SPEC>`"]
pub type RISR = crate::Reg<risr::RISR_SPEC>;
#[doc = "raw interrupt status register"]
pub mod risr;
#[doc = "MISR register accessor: an alias for `Reg<MISR_SPEC>`"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "masked interrupt status register"]
pub mod misr;
#[doc = "CSGCMCCMR register accessor: an alias for `Reg<CSGCMCCMR_SPEC>`"]
pub type CSGCMCCMR = crate::Reg<csgcmccmr::CSGCMCCMR_SPEC>;
#[doc = "context swap register"]
pub mod csgcmccmr;
#[doc = "CSGCMR register accessor: an alias for `Reg<CSGCMR_SPEC>`"]
pub type CSGCMR = crate::Reg<csgcmr::CSGCMR_SPEC>;
#[doc = "context swap register"]
pub mod csgcmr;
