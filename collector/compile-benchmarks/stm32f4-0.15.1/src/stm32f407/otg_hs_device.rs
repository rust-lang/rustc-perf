#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS device configuration register"]
    pub dcfg: crate::Reg<dcfg::DCFG_SPEC>,
    #[doc = "0x04 - OTG_HS device control register"]
    pub dctl: crate::Reg<dctl::DCTL_SPEC>,
    #[doc = "0x08 - OTG_HS device status register"]
    pub dsts: crate::Reg<dsts::DSTS_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - OTG_HS device IN endpoint common interrupt mask register"]
    pub diepmsk: crate::Reg<diepmsk::DIEPMSK_SPEC>,
    #[doc = "0x14 - OTG_HS device OUT endpoint common interrupt mask register"]
    pub doepmsk: crate::Reg<doepmsk::DOEPMSK_SPEC>,
    #[doc = "0x18 - OTG_HS device all endpoints interrupt register"]
    pub daint: crate::Reg<daint::DAINT_SPEC>,
    #[doc = "0x1c - OTG_HS all endpoints interrupt mask register"]
    pub daintmsk: crate::Reg<daintmsk::DAINTMSK_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0x28 - OTG_HS device VBUS discharge time register"]
    pub dvbusdis: crate::Reg<dvbusdis::DVBUSDIS_SPEC>,
    #[doc = "0x2c - OTG_HS device VBUS pulsing time register"]
    pub dvbuspulse: crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>,
    #[doc = "0x30 - OTG_HS Device threshold control register"]
    pub dthrctl: crate::Reg<dthrctl::DTHRCTL_SPEC>,
    #[doc = "0x34 - OTG_HS device IN endpoint FIFO empty interrupt mask register"]
    pub diepempmsk: crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>,
    #[doc = "0x38 - OTG_HS device each endpoint interrupt register"]
    pub deachint: crate::Reg<deachint::DEACHINT_SPEC>,
    #[doc = "0x3c - OTG_HS device each endpoint interrupt register mask"]
    pub deachintmsk: crate::Reg<deachintmsk::DEACHINTMSK_SPEC>,
    #[doc = "0x40 - OTG_HS device each in endpoint-1 interrupt register"]
    pub diepeachmsk1: crate::Reg<diepeachmsk1::DIEPEACHMSK1_SPEC>,
    _reserved14: [u8; 0x3c],
    #[doc = "0x80 - OTG_HS device each OUT endpoint-1 interrupt register"]
    pub doepeachmsk1: crate::Reg<doepeachmsk1::DOEPEACHMSK1_SPEC>,
    _reserved15: [u8; 0x7c],
    #[doc = "0x100 - OTG device endpoint-0 control register"]
    pub diepctl0: crate::Reg<diepctl0::DIEPCTL0_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x108 - OTG device endpoint-%s interrupt register"]
    pub diepint0: crate::Reg<diepint::DIEPINT_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x110 - OTG_HS device IN endpoint 0 transfer size register"]
    pub dieptsiz0: crate::Reg<dieptsiz0::DIEPTSIZ0_SPEC>,
    #[doc = "0x114 - OTG_HS device endpoint-1 DMA address register"]
    pub diepdma1: crate::Reg<diepdma1::DIEPDMA1_SPEC>,
    #[doc = "0x118 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts0: crate::Reg<dtxfsts::DTXFSTS_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x120 - OTG device endpoint-1 control register"]
    pub diepctl1: crate::Reg<diepctl::DIEPCTL_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x128 - OTG device endpoint-%s interrupt register"]
    pub diepint1: crate::Reg<diepint::DIEPINT_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x130 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz1: crate::Reg<dieptsiz::DIEPTSIZ_SPEC>,
    #[doc = "0x134 - OTG_HS device endpoint-2 DMA address register"]
    pub diepdma2: crate::Reg<diepdma2::DIEPDMA2_SPEC>,
    #[doc = "0x138 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts1: crate::Reg<dtxfsts::DTXFSTS_SPEC>,
    _reserved25: [u8; 0x04],
    #[doc = "0x140 - OTG device endpoint-1 control register"]
    pub diepctl2: crate::Reg<diepctl::DIEPCTL_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0x148 - OTG device endpoint-%s interrupt register"]
    pub diepint2: crate::Reg<diepint::DIEPINT_SPEC>,
    _reserved27: [u8; 0x04],
    #[doc = "0x150 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz2: crate::Reg<dieptsiz::DIEPTSIZ_SPEC>,
    #[doc = "0x154 - OTG_HS device endpoint-3 DMA address register"]
    pub diepdma3: crate::Reg<diepdma3::DIEPDMA3_SPEC>,
    #[doc = "0x158 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts2: crate::Reg<dtxfsts::DTXFSTS_SPEC>,
    _reserved30: [u8; 0x04],
    #[doc = "0x160 - OTG device endpoint-1 control register"]
    pub diepctl3: crate::Reg<diepctl::DIEPCTL_SPEC>,
    _reserved31: [u8; 0x04],
    #[doc = "0x168 - OTG device endpoint-%s interrupt register"]
    pub diepint3: crate::Reg<diepint::DIEPINT_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x170 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz3: crate::Reg<dieptsiz::DIEPTSIZ_SPEC>,
    #[doc = "0x174 - OTG_HS device endpoint-4 DMA address register"]
    pub diepdma4: crate::Reg<diepdma4::DIEPDMA4_SPEC>,
    #[doc = "0x178 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts3: crate::Reg<dtxfsts::DTXFSTS_SPEC>,
    _reserved35: [u8; 0x04],
    #[doc = "0x180 - OTG device endpoint-1 control register"]
    pub diepctl4: crate::Reg<diepctl::DIEPCTL_SPEC>,
    _reserved36: [u8; 0x04],
    #[doc = "0x188 - OTG device endpoint-%s interrupt register"]
    pub diepint4: crate::Reg<diepint::DIEPINT_SPEC>,
    _reserved37: [u8; 0x04],
    #[doc = "0x190 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz4: crate::Reg<dieptsiz::DIEPTSIZ_SPEC>,
    #[doc = "0x194 - OTG_HS device endpoint-5 DMA address register"]
    pub diepdma5: crate::Reg<diepdma5::DIEPDMA5_SPEC>,
    #[doc = "0x198 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts4: crate::Reg<dtxfsts::DTXFSTS_SPEC>,
    _reserved40: [u8; 0x04],
    #[doc = "0x1a0 - OTG device endpoint-1 control register"]
    pub diepctl5: crate::Reg<diepctl::DIEPCTL_SPEC>,
    _reserved41: [u8; 0x04],
    #[doc = "0x1a8 - OTG device endpoint-%s interrupt register"]
    pub diepint5: crate::Reg<diepint::DIEPINT_SPEC>,
    _reserved42: [u8; 0x04],
    #[doc = "0x1b0 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz5: crate::Reg<dieptsiz::DIEPTSIZ_SPEC>,
    _reserved43: [u8; 0x04],
    #[doc = "0x1b8 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts5: crate::Reg<dtxfsts::DTXFSTS_SPEC>,
    _reserved44: [u8; 0x0144],
    #[doc = "0x300 - OTG_HS device control OUT endpoint 0 control register"]
    pub doepctl0: crate::Reg<doepctl0::DOEPCTL0_SPEC>,
    _reserved45: [u8; 0x04],
    #[doc = "0x308 - OTG_HS device endpoint-%s interrupt register"]
    pub doepint0: crate::Reg<doepint::DOEPINT_SPEC>,
    _reserved46: [u8; 0x04],
    #[doc = "0x310 - OTG_HS device endpoint-1 transfer size register"]
    pub doeptsiz0: crate::Reg<doeptsiz0::DOEPTSIZ0_SPEC>,
    _reserved47: [u8; 0x0c],
    #[doc = "0x320 - OTG device endpoint-1 control register"]
    pub doepctl1: crate::Reg<doepctl::DOEPCTL_SPEC>,
    _reserved48: [u8; 0x04],
    #[doc = "0x328 - OTG_HS device endpoint-%s interrupt register"]
    pub doepint1: crate::Reg<doepint::DOEPINT_SPEC>,
    _reserved49: [u8; 0x04],
    #[doc = "0x330 - OTG_HS device endpoint-2 transfer size register"]
    pub doeptsiz1: crate::Reg<doeptsiz::DOEPTSIZ_SPEC>,
    _reserved50: [u8; 0x0c],
    #[doc = "0x340 - OTG device endpoint-1 control register"]
    pub doepctl2: crate::Reg<doepctl::DOEPCTL_SPEC>,
    _reserved51: [u8; 0x04],
    #[doc = "0x348 - OTG_HS device endpoint-%s interrupt register"]
    pub doepint2: crate::Reg<doepint::DOEPINT_SPEC>,
    _reserved52: [u8; 0x04],
    #[doc = "0x350 - OTG_HS device endpoint-2 transfer size register"]
    pub doeptsiz2: crate::Reg<doeptsiz::DOEPTSIZ_SPEC>,
    _reserved53: [u8; 0x0c],
    #[doc = "0x360 - OTG device endpoint-1 control register"]
    pub doepctl3: crate::Reg<doepctl::DOEPCTL_SPEC>,
    _reserved54: [u8; 0x04],
    #[doc = "0x368 - OTG_HS device endpoint-%s interrupt register"]
    pub doepint3: crate::Reg<doepint::DOEPINT_SPEC>,
    _reserved55: [u8; 0x04],
    #[doc = "0x370 - OTG_HS device endpoint-2 transfer size register"]
    pub doeptsiz3: crate::Reg<doeptsiz::DOEPTSIZ_SPEC>,
    _reserved56: [u8; 0x0c],
    #[doc = "0x380 - OTG device endpoint-1 control register"]
    pub doepctl4: crate::Reg<doepctl::DOEPCTL_SPEC>,
    _reserved57: [u8; 0x04],
    #[doc = "0x388 - OTG_HS device endpoint-%s interrupt register"]
    pub doepint4: crate::Reg<doepint::DOEPINT_SPEC>,
    _reserved58: [u8; 0x04],
    #[doc = "0x390 - OTG_HS device endpoint-2 transfer size register"]
    pub doeptsiz4: crate::Reg<doeptsiz::DOEPTSIZ_SPEC>,
    _reserved59: [u8; 0x0c],
    #[doc = "0x3a0 - OTG device endpoint-1 control register"]
    pub doepctl5: crate::Reg<doepctl::DOEPCTL_SPEC>,
    _reserved60: [u8; 0x04],
    #[doc = "0x3a8 - OTG_HS device endpoint-%s interrupt register"]
    pub doepint5: crate::Reg<doepint::DOEPINT_SPEC>,
    _reserved61: [u8; 0x04],
    #[doc = "0x3b0 - OTG_HS device endpoint-2 transfer size register"]
    pub doeptsiz5: crate::Reg<doeptsiz::DOEPTSIZ_SPEC>,
}
#[doc = "DCFG register accessor: an alias for `Reg<DCFG_SPEC>`"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "OTG_HS device configuration register"]
pub mod dcfg;
#[doc = "DCTL register accessor: an alias for `Reg<DCTL_SPEC>`"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "OTG_HS device control register"]
pub mod dctl;
#[doc = "DSTS register accessor: an alias for `Reg<DSTS_SPEC>`"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "OTG_HS device status register"]
pub mod dsts;
#[doc = "DIEPMSK register accessor: an alias for `Reg<DIEPMSK_SPEC>`"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "OTG_HS device IN endpoint common interrupt mask register"]
pub mod diepmsk;
#[doc = "DOEPMSK register accessor: an alias for `Reg<DOEPMSK_SPEC>`"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "OTG_HS device OUT endpoint common interrupt mask register"]
pub mod doepmsk;
#[doc = "DAINT register accessor: an alias for `Reg<DAINT_SPEC>`"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "OTG_HS device all endpoints interrupt register"]
pub mod daint;
#[doc = "DAINTMSK register accessor: an alias for `Reg<DAINTMSK_SPEC>`"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "OTG_HS all endpoints interrupt mask register"]
pub mod daintmsk;
#[doc = "DVBUSDIS register accessor: an alias for `Reg<DVBUSDIS_SPEC>`"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
#[doc = "OTG_HS device VBUS discharge time register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE register accessor: an alias for `Reg<DVBUSPULSE_SPEC>`"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
#[doc = "OTG_HS device VBUS pulsing time register"]
pub mod dvbuspulse;
#[doc = "DTHRCTL register accessor: an alias for `Reg<DTHRCTL_SPEC>`"]
pub type DTHRCTL = crate::Reg<dthrctl::DTHRCTL_SPEC>;
#[doc = "OTG_HS Device threshold control register"]
pub mod dthrctl;
#[doc = "DIEPEMPMSK register accessor: an alias for `Reg<DIEPEMPMSK_SPEC>`"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register"]
pub mod diepempmsk;
#[doc = "DEACHINT register accessor: an alias for `Reg<DEACHINT_SPEC>`"]
pub type DEACHINT = crate::Reg<deachint::DEACHINT_SPEC>;
#[doc = "OTG_HS device each endpoint interrupt register"]
pub mod deachint;
#[doc = "DEACHINTMSK register accessor: an alias for `Reg<DEACHINTMSK_SPEC>`"]
pub type DEACHINTMSK = crate::Reg<deachintmsk::DEACHINTMSK_SPEC>;
#[doc = "OTG_HS device each endpoint interrupt register mask"]
pub mod deachintmsk;
#[doc = "DIEPEACHMSK1 register accessor: an alias for `Reg<DIEPEACHMSK1_SPEC>`"]
pub type DIEPEACHMSK1 = crate::Reg<diepeachmsk1::DIEPEACHMSK1_SPEC>;
#[doc = "OTG_HS device each in endpoint-1 interrupt register"]
pub mod diepeachmsk1;
#[doc = "DOEPEACHMSK1 register accessor: an alias for `Reg<DOEPEACHMSK1_SPEC>`"]
pub type DOEPEACHMSK1 = crate::Reg<doepeachmsk1::DOEPEACHMSK1_SPEC>;
#[doc = "OTG_HS device each OUT endpoint-1 interrupt register"]
pub mod doepeachmsk1;
#[doc = "DIEPCTL0 register accessor: an alias for `Reg<DIEPCTL0_SPEC>`"]
pub type DIEPCTL0 = crate::Reg<diepctl0::DIEPCTL0_SPEC>;
#[doc = "OTG device endpoint-0 control register"]
pub mod diepctl0;
#[doc = "DIEPCTL register accessor: an alias for `Reg<DIEPCTL_SPEC>`"]
pub type DIEPCTL = crate::Reg<diepctl::DIEPCTL_SPEC>;
#[doc = "OTG device endpoint-1 control register"]
pub mod diepctl;
#[doc = "DIEPINT register accessor: an alias for `Reg<DIEPINT_SPEC>`"]
pub type DIEPINT = crate::Reg<diepint::DIEPINT_SPEC>;
#[doc = "OTG device endpoint-%s interrupt register"]
pub mod diepint;
#[doc = "DIEPTSIZ0 register accessor: an alias for `Reg<DIEPTSIZ0_SPEC>`"]
pub type DIEPTSIZ0 = crate::Reg<dieptsiz0::DIEPTSIZ0_SPEC>;
#[doc = "OTG_HS device IN endpoint 0 transfer size register"]
pub mod dieptsiz0;
#[doc = "DIEPDMA1 register accessor: an alias for `Reg<DIEPDMA1_SPEC>`"]
pub type DIEPDMA1 = crate::Reg<diepdma1::DIEPDMA1_SPEC>;
#[doc = "OTG_HS device endpoint-1 DMA address register"]
pub mod diepdma1;
#[doc = "DIEPDMA2 register accessor: an alias for `Reg<DIEPDMA2_SPEC>`"]
pub type DIEPDMA2 = crate::Reg<diepdma2::DIEPDMA2_SPEC>;
#[doc = "OTG_HS device endpoint-2 DMA address register"]
pub mod diepdma2;
#[doc = "DIEPDMA3 register accessor: an alias for `Reg<DIEPDMA3_SPEC>`"]
pub type DIEPDMA3 = crate::Reg<diepdma3::DIEPDMA3_SPEC>;
#[doc = "OTG_HS device endpoint-3 DMA address register"]
pub mod diepdma3;
#[doc = "DIEPDMA4 register accessor: an alias for `Reg<DIEPDMA4_SPEC>`"]
pub type DIEPDMA4 = crate::Reg<diepdma4::DIEPDMA4_SPEC>;
#[doc = "OTG_HS device endpoint-4 DMA address register"]
pub mod diepdma4;
#[doc = "DIEPDMA5 register accessor: an alias for `Reg<DIEPDMA5_SPEC>`"]
pub type DIEPDMA5 = crate::Reg<diepdma5::DIEPDMA5_SPEC>;
#[doc = "OTG_HS device endpoint-5 DMA address register"]
pub mod diepdma5;
#[doc = "DTXFSTS register accessor: an alias for `Reg<DTXFSTS_SPEC>`"]
pub type DTXFSTS = crate::Reg<dtxfsts::DTXFSTS_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts;
#[doc = "DIEPTSIZ register accessor: an alias for `Reg<DIEPTSIZ_SPEC>`"]
pub type DIEPTSIZ = crate::Reg<dieptsiz::DIEPTSIZ_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz;
#[doc = "DOEPCTL0 register accessor: an alias for `Reg<DOEPCTL0_SPEC>`"]
pub type DOEPCTL0 = crate::Reg<doepctl0::DOEPCTL0_SPEC>;
#[doc = "OTG_HS device control OUT endpoint 0 control register"]
pub mod doepctl0;
#[doc = "DOEPCTL register accessor: an alias for `Reg<DOEPCTL_SPEC>`"]
pub type DOEPCTL = crate::Reg<doepctl::DOEPCTL_SPEC>;
#[doc = "OTG device endpoint-1 control register"]
pub mod doepctl;
#[doc = "DOEPINT register accessor: an alias for `Reg<DOEPINT_SPEC>`"]
pub type DOEPINT = crate::Reg<doepint::DOEPINT_SPEC>;
#[doc = "OTG_HS device endpoint-%s interrupt register"]
pub mod doepint;
#[doc = "DOEPTSIZ0 register accessor: an alias for `Reg<DOEPTSIZ0_SPEC>`"]
pub type DOEPTSIZ0 = crate::Reg<doeptsiz0::DOEPTSIZ0_SPEC>;
#[doc = "OTG_HS device endpoint-1 transfer size register"]
pub mod doeptsiz0;
#[doc = "DOEPTSIZ register accessor: an alias for `Reg<DOEPTSIZ_SPEC>`"]
pub type DOEPTSIZ = crate::Reg<doeptsiz::DOEPTSIZ_SPEC>;
#[doc = "OTG_HS device endpoint-2 transfer size register"]
pub mod doeptsiz;
