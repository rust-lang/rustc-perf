#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA2D control register"]
    pub dma2d_cr: crate::Reg<dma2d_cr::DMA2D_CR_SPEC>,
    #[doc = "0x04 - DMA2D Interrupt Status Register"]
    pub dma2d_isr: crate::Reg<dma2d_isr::DMA2D_ISR_SPEC>,
    #[doc = "0x08 - DMA2D interrupt flag clear register"]
    pub dma2d_ifcr: crate::Reg<dma2d_ifcr::DMA2D_IFCR_SPEC>,
    #[doc = "0x0c - DMA2D foreground memory address register"]
    pub dma2d_fgmar: crate::Reg<dma2d_fgmar::DMA2D_FGMAR_SPEC>,
    #[doc = "0x10 - DMA2D foreground offset register"]
    pub dma2d_fgor: crate::Reg<dma2d_fgor::DMA2D_FGOR_SPEC>,
    #[doc = "0x14 - DMA2D background memory address register"]
    pub dma2d_bgmar: crate::Reg<dma2d_bgmar::DMA2D_BGMAR_SPEC>,
    #[doc = "0x18 - DMA2D background offset register"]
    pub dma2d_bgor: crate::Reg<dma2d_bgor::DMA2D_BGOR_SPEC>,
    #[doc = "0x1c - DMA2D foreground PFC control register"]
    pub dma2d_fgpfccr: crate::Reg<dma2d_fgpfccr::DMA2D_FGPFCCR_SPEC>,
    #[doc = "0x20 - DMA2D foreground color register"]
    pub dma2d_fgcolr: crate::Reg<dma2d_fgcolr::DMA2D_FGCOLR_SPEC>,
    #[doc = "0x24 - DMA2D background PFC control register"]
    pub dma2d_bgpfccr: crate::Reg<dma2d_bgpfccr::DMA2D_BGPFCCR_SPEC>,
    #[doc = "0x28 - DMA2D background color register"]
    pub dma2d_bgcolr: crate::Reg<dma2d_bgcolr::DMA2D_BGCOLR_SPEC>,
    #[doc = "0x2c - DMA2D foreground CLUT memory address register"]
    pub dma2d_fgcmar: crate::Reg<dma2d_fgcmar::DMA2D_FGCMAR_SPEC>,
    #[doc = "0x30 - DMA2D background CLUT memory address register"]
    pub dma2d_bgcmar: crate::Reg<dma2d_bgcmar::DMA2D_BGCMAR_SPEC>,
    #[doc = "0x34 - DMA2D output PFC control register"]
    pub dma2d_opfccr: crate::Reg<dma2d_opfccr::DMA2D_OPFCCR_SPEC>,
    #[doc = "0x38 - DMA2D output color register"]
    pub dma2d_ocolr: crate::Reg<dma2d_ocolr::DMA2D_OCOLR_SPEC>,
    #[doc = "0x3c - DMA2D output memory address register"]
    pub dma2d_omar: crate::Reg<dma2d_omar::DMA2D_OMAR_SPEC>,
    #[doc = "0x40 - DMA2D output offset register"]
    pub dma2d_oor: crate::Reg<dma2d_oor::DMA2D_OOR_SPEC>,
    #[doc = "0x44 - DMA2D number of line register"]
    pub dma2d_nlr: crate::Reg<dma2d_nlr::DMA2D_NLR_SPEC>,
    #[doc = "0x48 - DMA2D line watermark register"]
    pub dma2d_lwr: crate::Reg<dma2d_lwr::DMA2D_LWR_SPEC>,
    #[doc = "0x4c - DMA2D AXI master timer configuration register"]
    pub dma2d_amtcr: crate::Reg<dma2d_amtcr::DMA2D_AMTCR_SPEC>,
}
#[doc = "DMA2D_CR register accessor: an alias for `Reg<DMA2D_CR_SPEC>`"]
pub type DMA2D_CR = crate::Reg<dma2d_cr::DMA2D_CR_SPEC>;
#[doc = "DMA2D control register"]
pub mod dma2d_cr;
#[doc = "DMA2D_ISR register accessor: an alias for `Reg<DMA2D_ISR_SPEC>`"]
pub type DMA2D_ISR = crate::Reg<dma2d_isr::DMA2D_ISR_SPEC>;
#[doc = "DMA2D Interrupt Status Register"]
pub mod dma2d_isr;
#[doc = "DMA2D_IFCR register accessor: an alias for `Reg<DMA2D_IFCR_SPEC>`"]
pub type DMA2D_IFCR = crate::Reg<dma2d_ifcr::DMA2D_IFCR_SPEC>;
#[doc = "DMA2D interrupt flag clear register"]
pub mod dma2d_ifcr;
#[doc = "DMA2D_FGMAR register accessor: an alias for `Reg<DMA2D_FGMAR_SPEC>`"]
pub type DMA2D_FGMAR = crate::Reg<dma2d_fgmar::DMA2D_FGMAR_SPEC>;
#[doc = "DMA2D foreground memory address register"]
pub mod dma2d_fgmar;
#[doc = "DMA2D_FGOR register accessor: an alias for `Reg<DMA2D_FGOR_SPEC>`"]
pub type DMA2D_FGOR = crate::Reg<dma2d_fgor::DMA2D_FGOR_SPEC>;
#[doc = "DMA2D foreground offset register"]
pub mod dma2d_fgor;
#[doc = "DMA2D_BGMAR register accessor: an alias for `Reg<DMA2D_BGMAR_SPEC>`"]
pub type DMA2D_BGMAR = crate::Reg<dma2d_bgmar::DMA2D_BGMAR_SPEC>;
#[doc = "DMA2D background memory address register"]
pub mod dma2d_bgmar;
#[doc = "DMA2D_BGOR register accessor: an alias for `Reg<DMA2D_BGOR_SPEC>`"]
pub type DMA2D_BGOR = crate::Reg<dma2d_bgor::DMA2D_BGOR_SPEC>;
#[doc = "DMA2D background offset register"]
pub mod dma2d_bgor;
#[doc = "DMA2D_FGPFCCR register accessor: an alias for `Reg<DMA2D_FGPFCCR_SPEC>`"]
pub type DMA2D_FGPFCCR = crate::Reg<dma2d_fgpfccr::DMA2D_FGPFCCR_SPEC>;
#[doc = "DMA2D foreground PFC control register"]
pub mod dma2d_fgpfccr;
#[doc = "DMA2D_FGCOLR register accessor: an alias for `Reg<DMA2D_FGCOLR_SPEC>`"]
pub type DMA2D_FGCOLR = crate::Reg<dma2d_fgcolr::DMA2D_FGCOLR_SPEC>;
#[doc = "DMA2D foreground color register"]
pub mod dma2d_fgcolr;
#[doc = "DMA2D_BGPFCCR register accessor: an alias for `Reg<DMA2D_BGPFCCR_SPEC>`"]
pub type DMA2D_BGPFCCR = crate::Reg<dma2d_bgpfccr::DMA2D_BGPFCCR_SPEC>;
#[doc = "DMA2D background PFC control register"]
pub mod dma2d_bgpfccr;
#[doc = "DMA2D_BGCOLR register accessor: an alias for `Reg<DMA2D_BGCOLR_SPEC>`"]
pub type DMA2D_BGCOLR = crate::Reg<dma2d_bgcolr::DMA2D_BGCOLR_SPEC>;
#[doc = "DMA2D background color register"]
pub mod dma2d_bgcolr;
#[doc = "DMA2D_FGCMAR register accessor: an alias for `Reg<DMA2D_FGCMAR_SPEC>`"]
pub type DMA2D_FGCMAR = crate::Reg<dma2d_fgcmar::DMA2D_FGCMAR_SPEC>;
#[doc = "DMA2D foreground CLUT memory address register"]
pub mod dma2d_fgcmar;
#[doc = "DMA2D_BGCMAR register accessor: an alias for `Reg<DMA2D_BGCMAR_SPEC>`"]
pub type DMA2D_BGCMAR = crate::Reg<dma2d_bgcmar::DMA2D_BGCMAR_SPEC>;
#[doc = "DMA2D background CLUT memory address register"]
pub mod dma2d_bgcmar;
#[doc = "DMA2D_OPFCCR register accessor: an alias for `Reg<DMA2D_OPFCCR_SPEC>`"]
pub type DMA2D_OPFCCR = crate::Reg<dma2d_opfccr::DMA2D_OPFCCR_SPEC>;
#[doc = "DMA2D output PFC control register"]
pub mod dma2d_opfccr;
#[doc = "DMA2D_OCOLR register accessor: an alias for `Reg<DMA2D_OCOLR_SPEC>`"]
pub type DMA2D_OCOLR = crate::Reg<dma2d_ocolr::DMA2D_OCOLR_SPEC>;
#[doc = "DMA2D output color register"]
pub mod dma2d_ocolr;
#[doc = "DMA2D_OMAR register accessor: an alias for `Reg<DMA2D_OMAR_SPEC>`"]
pub type DMA2D_OMAR = crate::Reg<dma2d_omar::DMA2D_OMAR_SPEC>;
#[doc = "DMA2D output memory address register"]
pub mod dma2d_omar;
#[doc = "DMA2D_OOR register accessor: an alias for `Reg<DMA2D_OOR_SPEC>`"]
pub type DMA2D_OOR = crate::Reg<dma2d_oor::DMA2D_OOR_SPEC>;
#[doc = "DMA2D output offset register"]
pub mod dma2d_oor;
#[doc = "DMA2D_NLR register accessor: an alias for `Reg<DMA2D_NLR_SPEC>`"]
pub type DMA2D_NLR = crate::Reg<dma2d_nlr::DMA2D_NLR_SPEC>;
#[doc = "DMA2D number of line register"]
pub mod dma2d_nlr;
#[doc = "DMA2D_LWR register accessor: an alias for `Reg<DMA2D_LWR_SPEC>`"]
pub type DMA2D_LWR = crate::Reg<dma2d_lwr::DMA2D_LWR_SPEC>;
#[doc = "DMA2D line watermark register"]
pub mod dma2d_lwr;
#[doc = "DMA2D_AMTCR register accessor: an alias for `Reg<DMA2D_AMTCR_SPEC>`"]
pub type DMA2D_AMTCR = crate::Reg<dma2d_amtcr::DMA2D_AMTCR_SPEC>;
#[doc = "DMA2D AXI master timer configuration register"]
pub mod dma2d_amtcr;
