#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA2D control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - DMA2D Interrupt Status Register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x08 - DMA2D interrupt flag clear register"]
    pub ifcr: crate::Reg<ifcr::IFCR_SPEC>,
    #[doc = "0x0c - DMA2D foreground memory address register"]
    pub fgmar: crate::Reg<fgmar::FGMAR_SPEC>,
    #[doc = "0x10 - DMA2D foreground offset register"]
    pub fgor: crate::Reg<fgor::FGOR_SPEC>,
    #[doc = "0x14 - DMA2D background memory address register"]
    pub bgmar: crate::Reg<bgmar::BGMAR_SPEC>,
    #[doc = "0x18 - DMA2D background offset register"]
    pub bgor: crate::Reg<bgor::BGOR_SPEC>,
    #[doc = "0x1c - DMA2D foreground PFC control register"]
    pub fgpfccr: crate::Reg<fgpfccr::FGPFCCR_SPEC>,
    #[doc = "0x20 - DMA2D foreground color register"]
    pub fgcolr: crate::Reg<fgcolr::FGCOLR_SPEC>,
    #[doc = "0x24 - DMA2D background PFC control register"]
    pub bgpfccr: crate::Reg<bgpfccr::BGPFCCR_SPEC>,
    #[doc = "0x28 - DMA2D background color register"]
    pub bgcolr: crate::Reg<bgcolr::BGCOLR_SPEC>,
    #[doc = "0x2c - DMA2D foreground CLUT memory address register"]
    pub fgcmar: crate::Reg<fgcmar::FGCMAR_SPEC>,
    #[doc = "0x30 - DMA2D background CLUT memory address register"]
    pub bgcmar: crate::Reg<bgcmar::BGCMAR_SPEC>,
    #[doc = "0x34 - DMA2D output PFC control register"]
    pub opfccr: crate::Reg<opfccr::OPFCCR_SPEC>,
    #[doc = "0x38 - DMA2D output color register"]
    pub ocolr: crate::Reg<ocolr::OCOLR_SPEC>,
    #[doc = "0x3c - DMA2D output memory address register"]
    pub omar: crate::Reg<omar::OMAR_SPEC>,
    #[doc = "0x40 - DMA2D output offset register"]
    pub oor: crate::Reg<oor::OOR_SPEC>,
    #[doc = "0x44 - DMA2D number of line register"]
    pub nlr: crate::Reg<nlr::NLR_SPEC>,
    #[doc = "0x48 - DMA2D line watermark register"]
    pub lwr: crate::Reg<lwr::LWR_SPEC>,
    #[doc = "0x4c - DMA2D AXI master timer configuration register"]
    pub amtcr: crate::Reg<amtcr::AMTCR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "DMA2D control register"]
pub mod cr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "DMA2D Interrupt Status Register"]
pub mod isr;
#[doc = "IFCR register accessor: an alias for `Reg<IFCR_SPEC>`"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "DMA2D interrupt flag clear register"]
pub mod ifcr;
#[doc = "FGMAR register accessor: an alias for `Reg<FGMAR_SPEC>`"]
pub type FGMAR = crate::Reg<fgmar::FGMAR_SPEC>;
#[doc = "DMA2D foreground memory address register"]
pub mod fgmar;
#[doc = "FGOR register accessor: an alias for `Reg<FGOR_SPEC>`"]
pub type FGOR = crate::Reg<fgor::FGOR_SPEC>;
#[doc = "DMA2D foreground offset register"]
pub mod fgor;
#[doc = "BGMAR register accessor: an alias for `Reg<BGMAR_SPEC>`"]
pub type BGMAR = crate::Reg<bgmar::BGMAR_SPEC>;
#[doc = "DMA2D background memory address register"]
pub mod bgmar;
#[doc = "BGOR register accessor: an alias for `Reg<BGOR_SPEC>`"]
pub type BGOR = crate::Reg<bgor::BGOR_SPEC>;
#[doc = "DMA2D background offset register"]
pub mod bgor;
#[doc = "FGPFCCR register accessor: an alias for `Reg<FGPFCCR_SPEC>`"]
pub type FGPFCCR = crate::Reg<fgpfccr::FGPFCCR_SPEC>;
#[doc = "DMA2D foreground PFC control register"]
pub mod fgpfccr;
#[doc = "FGCOLR register accessor: an alias for `Reg<FGCOLR_SPEC>`"]
pub type FGCOLR = crate::Reg<fgcolr::FGCOLR_SPEC>;
#[doc = "DMA2D foreground color register"]
pub mod fgcolr;
#[doc = "BGPFCCR register accessor: an alias for `Reg<BGPFCCR_SPEC>`"]
pub type BGPFCCR = crate::Reg<bgpfccr::BGPFCCR_SPEC>;
#[doc = "DMA2D background PFC control register"]
pub mod bgpfccr;
#[doc = "BGCOLR register accessor: an alias for `Reg<BGCOLR_SPEC>`"]
pub type BGCOLR = crate::Reg<bgcolr::BGCOLR_SPEC>;
#[doc = "DMA2D background color register"]
pub mod bgcolr;
#[doc = "FGCMAR register accessor: an alias for `Reg<FGCMAR_SPEC>`"]
pub type FGCMAR = crate::Reg<fgcmar::FGCMAR_SPEC>;
#[doc = "DMA2D foreground CLUT memory address register"]
pub mod fgcmar;
#[doc = "BGCMAR register accessor: an alias for `Reg<BGCMAR_SPEC>`"]
pub type BGCMAR = crate::Reg<bgcmar::BGCMAR_SPEC>;
#[doc = "DMA2D background CLUT memory address register"]
pub mod bgcmar;
#[doc = "OPFCCR register accessor: an alias for `Reg<OPFCCR_SPEC>`"]
pub type OPFCCR = crate::Reg<opfccr::OPFCCR_SPEC>;
#[doc = "DMA2D output PFC control register"]
pub mod opfccr;
#[doc = "OCOLR register accessor: an alias for `Reg<OCOLR_SPEC>`"]
pub type OCOLR = crate::Reg<ocolr::OCOLR_SPEC>;
#[doc = "DMA2D output color register"]
pub mod ocolr;
#[doc = "OMAR register accessor: an alias for `Reg<OMAR_SPEC>`"]
pub type OMAR = crate::Reg<omar::OMAR_SPEC>;
#[doc = "DMA2D output memory address register"]
pub mod omar;
#[doc = "OOR register accessor: an alias for `Reg<OOR_SPEC>`"]
pub type OOR = crate::Reg<oor::OOR_SPEC>;
#[doc = "DMA2D output offset register"]
pub mod oor;
#[doc = "NLR register accessor: an alias for `Reg<NLR_SPEC>`"]
pub type NLR = crate::Reg<nlr::NLR_SPEC>;
#[doc = "DMA2D number of line register"]
pub mod nlr;
#[doc = "LWR register accessor: an alias for `Reg<LWR_SPEC>`"]
pub type LWR = crate::Reg<lwr::LWR_SPEC>;
#[doc = "DMA2D line watermark register"]
pub mod lwr;
#[doc = "AMTCR register accessor: an alias for `Reg<AMTCR_SPEC>`"]
pub type AMTCR = crate::Reg<amtcr::AMTCR_SPEC>;
#[doc = "DMA2D AXI master timer configuration register"]
pub mod amtcr;
