#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet DMA bus mode register"]
    pub dmabmr: crate::Reg<dmabmr::DMABMR_SPEC>,
    #[doc = "0x04 - Ethernet DMA transmit poll demand register"]
    pub dmatpdr: crate::Reg<dmatpdr::DMATPDR_SPEC>,
    #[doc = "0x08 - EHERNET DMA receive poll demand register"]
    pub dmarpdr: crate::Reg<dmarpdr::DMARPDR_SPEC>,
    #[doc = "0x0c - Ethernet DMA receive descriptor list address register"]
    pub dmardlar: crate::Reg<dmardlar::DMARDLAR_SPEC>,
    #[doc = "0x10 - Ethernet DMA transmit descriptor list address register"]
    pub dmatdlar: crate::Reg<dmatdlar::DMATDLAR_SPEC>,
    #[doc = "0x14 - Ethernet DMA status register"]
    pub dmasr: crate::Reg<dmasr::DMASR_SPEC>,
    #[doc = "0x18 - Ethernet DMA operation mode register"]
    pub dmaomr: crate::Reg<dmaomr::DMAOMR_SPEC>,
    #[doc = "0x1c - Ethernet DMA interrupt enable register"]
    pub dmaier: crate::Reg<dmaier::DMAIER_SPEC>,
    #[doc = "0x20 - Ethernet DMA missed frame and buffer overflow counter register"]
    pub dmamfbocr: crate::Reg<dmamfbocr::DMAMFBOCR_SPEC>,
    #[doc = "0x24 - Ethernet DMA receive status watchdog timer register"]
    pub dmarswtr: crate::Reg<dmarswtr::DMARSWTR_SPEC>,
    _reserved10: [u8; 0x20],
    #[doc = "0x48 - Ethernet DMA current host transmit descriptor register"]
    pub dmachtdr: crate::Reg<dmachtdr::DMACHTDR_SPEC>,
    #[doc = "0x4c - Ethernet DMA current host receive descriptor register"]
    pub dmachrdr: crate::Reg<dmachrdr::DMACHRDR_SPEC>,
    #[doc = "0x50 - Ethernet DMA current host transmit buffer address register"]
    pub dmachtbar: crate::Reg<dmachtbar::DMACHTBAR_SPEC>,
    #[doc = "0x54 - Ethernet DMA current host receive buffer address register"]
    pub dmachrbar: crate::Reg<dmachrbar::DMACHRBAR_SPEC>,
}
#[doc = "DMABMR register accessor: an alias for `Reg<DMABMR_SPEC>`"]
pub type DMABMR = crate::Reg<dmabmr::DMABMR_SPEC>;
#[doc = "Ethernet DMA bus mode register"]
pub mod dmabmr;
#[doc = "DMATPDR register accessor: an alias for `Reg<DMATPDR_SPEC>`"]
pub type DMATPDR = crate::Reg<dmatpdr::DMATPDR_SPEC>;
#[doc = "Ethernet DMA transmit poll demand register"]
pub mod dmatpdr;
#[doc = "DMARPDR register accessor: an alias for `Reg<DMARPDR_SPEC>`"]
pub type DMARPDR = crate::Reg<dmarpdr::DMARPDR_SPEC>;
#[doc = "EHERNET DMA receive poll demand register"]
pub mod dmarpdr;
#[doc = "DMARDLAR register accessor: an alias for `Reg<DMARDLAR_SPEC>`"]
pub type DMARDLAR = crate::Reg<dmardlar::DMARDLAR_SPEC>;
#[doc = "Ethernet DMA receive descriptor list address register"]
pub mod dmardlar;
#[doc = "DMATDLAR register accessor: an alias for `Reg<DMATDLAR_SPEC>`"]
pub type DMATDLAR = crate::Reg<dmatdlar::DMATDLAR_SPEC>;
#[doc = "Ethernet DMA transmit descriptor list address register"]
pub mod dmatdlar;
#[doc = "DMASR register accessor: an alias for `Reg<DMASR_SPEC>`"]
pub type DMASR = crate::Reg<dmasr::DMASR_SPEC>;
#[doc = "Ethernet DMA status register"]
pub mod dmasr;
#[doc = "DMAOMR register accessor: an alias for `Reg<DMAOMR_SPEC>`"]
pub type DMAOMR = crate::Reg<dmaomr::DMAOMR_SPEC>;
#[doc = "Ethernet DMA operation mode register"]
pub mod dmaomr;
#[doc = "DMAIER register accessor: an alias for `Reg<DMAIER_SPEC>`"]
pub type DMAIER = crate::Reg<dmaier::DMAIER_SPEC>;
#[doc = "Ethernet DMA interrupt enable register"]
pub mod dmaier;
#[doc = "DMAMFBOCR register accessor: an alias for `Reg<DMAMFBOCR_SPEC>`"]
pub type DMAMFBOCR = crate::Reg<dmamfbocr::DMAMFBOCR_SPEC>;
#[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
pub mod dmamfbocr;
#[doc = "DMARSWTR register accessor: an alias for `Reg<DMARSWTR_SPEC>`"]
pub type DMARSWTR = crate::Reg<dmarswtr::DMARSWTR_SPEC>;
#[doc = "Ethernet DMA receive status watchdog timer register"]
pub mod dmarswtr;
#[doc = "DMACHTDR register accessor: an alias for `Reg<DMACHTDR_SPEC>`"]
pub type DMACHTDR = crate::Reg<dmachtdr::DMACHTDR_SPEC>;
#[doc = "Ethernet DMA current host transmit descriptor register"]
pub mod dmachtdr;
#[doc = "DMACHRDR register accessor: an alias for `Reg<DMACHRDR_SPEC>`"]
pub type DMACHRDR = crate::Reg<dmachrdr::DMACHRDR_SPEC>;
#[doc = "Ethernet DMA current host receive descriptor register"]
pub mod dmachrdr;
#[doc = "DMACHTBAR register accessor: an alias for `Reg<DMACHTBAR_SPEC>`"]
pub type DMACHTBAR = crate::Reg<dmachtbar::DMACHTBAR_SPEC>;
#[doc = "Ethernet DMA current host transmit buffer address register"]
pub mod dmachtbar;
#[doc = "DMACHRBAR register accessor: an alias for `Reg<DMACHRBAR_SPEC>`"]
pub type DMACHRBAR = crate::Reg<dmachrbar::DMACHRBAR_SPEC>;
#[doc = "Ethernet DMA current host receive buffer address register"]
pub mod dmachrbar;
