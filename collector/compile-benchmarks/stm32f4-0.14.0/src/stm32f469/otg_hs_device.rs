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
    #[doc = "0x108 - OTG device endpoint-0 interrupt register"]
    pub diepint0: crate::Reg<diepint0::DIEPINT0_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x110 - OTG_HS device IN endpoint 0 transfer size register"]
    pub dieptsiz0: crate::Reg<dieptsiz0::DIEPTSIZ0_SPEC>,
    #[doc = "0x114 - OTG_HS device endpoint-1 DMA address register"]
    pub diepdma1: crate::Reg<diepdma1::DIEPDMA1_SPEC>,
    #[doc = "0x118 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts0: crate::Reg<dtxfsts0::DTXFSTS0_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x120 - OTG device endpoint-1 control register"]
    pub diepctl1: crate::Reg<diepctl1::DIEPCTL1_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x128 - OTG device endpoint-1 interrupt register"]
    pub diepint1: crate::Reg<diepint1::DIEPINT1_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x130 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz1: crate::Reg<dieptsiz1::DIEPTSIZ1_SPEC>,
    #[doc = "0x134 - OTG_HS device endpoint-2 DMA address register"]
    pub diepdma2: crate::Reg<diepdma2::DIEPDMA2_SPEC>,
    #[doc = "0x138 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts1: crate::Reg<dtxfsts1::DTXFSTS1_SPEC>,
    _reserved25: [u8; 0x04],
    #[doc = "0x140 - OTG device endpoint-2 control register"]
    pub diepctl2: crate::Reg<diepctl2::DIEPCTL2_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0x148 - OTG device endpoint-2 interrupt register"]
    pub diepint2: crate::Reg<diepint2::DIEPINT2_SPEC>,
    _reserved27: [u8; 0x04],
    #[doc = "0x150 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz2: crate::Reg<dieptsiz2::DIEPTSIZ2_SPEC>,
    #[doc = "0x154 - OTG_HS device endpoint-3 DMA address register"]
    pub diepdma3: crate::Reg<diepdma3::DIEPDMA3_SPEC>,
    #[doc = "0x158 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts2: crate::Reg<dtxfsts2::DTXFSTS2_SPEC>,
    _reserved30: [u8; 0x04],
    #[doc = "0x160 - OTG device endpoint-3 control register"]
    pub diepctl3: crate::Reg<diepctl3::DIEPCTL3_SPEC>,
    _reserved31: [u8; 0x04],
    #[doc = "0x168 - OTG device endpoint-3 interrupt register"]
    pub diepint3: crate::Reg<diepint3::DIEPINT3_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x170 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz3: crate::Reg<dieptsiz3::DIEPTSIZ3_SPEC>,
    #[doc = "0x174 - OTG_HS device endpoint-4 DMA address register"]
    pub diepdma4: crate::Reg<diepdma4::DIEPDMA4_SPEC>,
    #[doc = "0x178 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts3: crate::Reg<dtxfsts3::DTXFSTS3_SPEC>,
    _reserved35: [u8; 0x04],
    #[doc = "0x180 - OTG device endpoint-4 control register"]
    pub diepctl4: crate::Reg<diepctl4::DIEPCTL4_SPEC>,
    _reserved36: [u8; 0x04],
    #[doc = "0x188 - OTG device endpoint-4 interrupt register"]
    pub diepint4: crate::Reg<diepint4::DIEPINT4_SPEC>,
    _reserved37: [u8; 0x04],
    #[doc = "0x190 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz4: crate::Reg<dieptsiz4::DIEPTSIZ4_SPEC>,
    #[doc = "0x194 - OTG_HS device endpoint-5 DMA address register"]
    pub diepdma5: crate::Reg<diepdma5::DIEPDMA5_SPEC>,
    #[doc = "0x198 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts4: crate::Reg<dtxfsts4::DTXFSTS4_SPEC>,
    _reserved40: [u8; 0x04],
    #[doc = "0x1a0 - OTG device endpoint-5 control register"]
    pub diepctl5: crate::Reg<diepctl5::DIEPCTL5_SPEC>,
    _reserved41: [u8; 0x04],
    #[doc = "0x1a8 - OTG device endpoint-5 interrupt register"]
    pub diepint5: crate::Reg<diepint5::DIEPINT5_SPEC>,
    _reserved42: [u8; 0x04],
    #[doc = "0x1b0 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz5: crate::Reg<dieptsiz5::DIEPTSIZ5_SPEC>,
    _reserved43: [u8; 0x04],
    #[doc = "0x1b8 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts5: crate::Reg<dtxfsts5::DTXFSTS5_SPEC>,
    _reserved44: [u8; 0x04],
    #[doc = "0x1c0 - OTG device endpoint-6 control register"]
    pub diepctl6: crate::Reg<diepctl6::DIEPCTL6_SPEC>,
    _reserved45: [u8; 0x04],
    #[doc = "0x1c8 - OTG device endpoint-6 interrupt register"]
    pub diepint6: crate::Reg<diepint6::DIEPINT6_SPEC>,
    _reserved46: [u8; 0x14],
    #[doc = "0x1e0 - OTG device endpoint-7 control register"]
    pub diepctl7: crate::Reg<diepctl7::DIEPCTL7_SPEC>,
    _reserved47: [u8; 0x04],
    #[doc = "0x1e8 - OTG device endpoint-7 interrupt register"]
    pub diepint7: crate::Reg<diepint7::DIEPINT7_SPEC>,
    _reserved48: [u8; 0x0114],
    #[doc = "0x300 - OTG_HS device control OUT endpoint 0 control register"]
    pub doepctl0: crate::Reg<doepctl0::DOEPCTL0_SPEC>,
    _reserved49: [u8; 0x04],
    #[doc = "0x308 - OTG_HS device endpoint-0 interrupt register"]
    pub doepint0: crate::Reg<doepint0::DOEPINT0_SPEC>,
    _reserved50: [u8; 0x04],
    #[doc = "0x310 - OTG_HS device endpoint-1 transfer size register"]
    pub doeptsiz0: crate::Reg<doeptsiz0::DOEPTSIZ0_SPEC>,
    _reserved51: [u8; 0x0c],
    #[doc = "0x320 - OTG device endpoint-1 control register"]
    pub doepctl1: crate::Reg<doepctl1::DOEPCTL1_SPEC>,
    _reserved52: [u8; 0x04],
    #[doc = "0x328 - OTG_HS device endpoint-1 interrupt register"]
    pub doepint1: crate::Reg<doepint1::DOEPINT1_SPEC>,
    _reserved53: [u8; 0x04],
    #[doc = "0x330 - OTG_HS device endpoint-2 transfer size register"]
    pub doeptsiz1: crate::Reg<doeptsiz1::DOEPTSIZ1_SPEC>,
    _reserved54: [u8; 0x0c],
    #[doc = "0x340 - OTG device endpoint-2 control register"]
    pub doepctl2: crate::Reg<doepctl2::DOEPCTL2_SPEC>,
    _reserved55: [u8; 0x04],
    #[doc = "0x348 - OTG_HS device endpoint-2 interrupt register"]
    pub doepint2: crate::Reg<doepint2::DOEPINT2_SPEC>,
    _reserved56: [u8; 0x04],
    #[doc = "0x350 - OTG_HS device endpoint-3 transfer size register"]
    pub doeptsiz2: crate::Reg<doeptsiz2::DOEPTSIZ2_SPEC>,
    _reserved57: [u8; 0x0c],
    #[doc = "0x360 - OTG device endpoint-3 control register"]
    pub doepctl3: crate::Reg<doepctl3::DOEPCTL3_SPEC>,
    _reserved58: [u8; 0x04],
    #[doc = "0x368 - OTG_HS device endpoint-3 interrupt register"]
    pub doepint3: crate::Reg<doepint3::DOEPINT3_SPEC>,
    _reserved59: [u8; 0x04],
    #[doc = "0x370 - OTG_HS device endpoint-4 transfer size register"]
    pub doeptsiz3: crate::Reg<doeptsiz3::DOEPTSIZ3_SPEC>,
    _reserved60: [u8; 0x14],
    #[doc = "0x388 - OTG_HS device endpoint-4 interrupt register"]
    pub doepint4: crate::Reg<doepint4::DOEPINT4_SPEC>,
    _reserved61: [u8; 0x04],
    #[doc = "0x390 - OTG_HS device endpoint-5 transfer size register"]
    pub doeptsiz4: crate::Reg<doeptsiz4::DOEPTSIZ4_SPEC>,
    _reserved62: [u8; 0x14],
    #[doc = "0x3a8 - OTG_HS device endpoint-5 interrupt register"]
    pub doepint5: crate::Reg<doepint5::DOEPINT5_SPEC>,
    _reserved63: [u8; 0x1c],
    #[doc = "0x3c8 - OTG_HS device endpoint-6 interrupt register"]
    pub doepint6: crate::Reg<doepint6::DOEPINT6_SPEC>,
    _reserved64: [u8; 0x1c],
    #[doc = "0x3e8 - OTG_HS device endpoint-7 interrupt register"]
    pub doepint7: crate::Reg<doepint7::DOEPINT7_SPEC>,
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
#[doc = "DIEPCTL1 register accessor: an alias for `Reg<DIEPCTL1_SPEC>`"]
pub type DIEPCTL1 = crate::Reg<diepctl1::DIEPCTL1_SPEC>;
#[doc = "OTG device endpoint-1 control register"]
pub mod diepctl1;
#[doc = "DIEPCTL2 register accessor: an alias for `Reg<DIEPCTL2_SPEC>`"]
pub type DIEPCTL2 = crate::Reg<diepctl2::DIEPCTL2_SPEC>;
#[doc = "OTG device endpoint-2 control register"]
pub mod diepctl2;
#[doc = "DIEPCTL3 register accessor: an alias for `Reg<DIEPCTL3_SPEC>`"]
pub type DIEPCTL3 = crate::Reg<diepctl3::DIEPCTL3_SPEC>;
#[doc = "OTG device endpoint-3 control register"]
pub mod diepctl3;
#[doc = "DIEPCTL4 register accessor: an alias for `Reg<DIEPCTL4_SPEC>`"]
pub type DIEPCTL4 = crate::Reg<diepctl4::DIEPCTL4_SPEC>;
#[doc = "OTG device endpoint-4 control register"]
pub mod diepctl4;
#[doc = "DIEPCTL5 register accessor: an alias for `Reg<DIEPCTL5_SPEC>`"]
pub type DIEPCTL5 = crate::Reg<diepctl5::DIEPCTL5_SPEC>;
#[doc = "OTG device endpoint-5 control register"]
pub mod diepctl5;
#[doc = "DIEPCTL6 register accessor: an alias for `Reg<DIEPCTL6_SPEC>`"]
pub type DIEPCTL6 = crate::Reg<diepctl6::DIEPCTL6_SPEC>;
#[doc = "OTG device endpoint-6 control register"]
pub mod diepctl6;
#[doc = "DIEPCTL7 register accessor: an alias for `Reg<DIEPCTL7_SPEC>`"]
pub type DIEPCTL7 = crate::Reg<diepctl7::DIEPCTL7_SPEC>;
#[doc = "OTG device endpoint-7 control register"]
pub mod diepctl7;
#[doc = "DIEPINT0 register accessor: an alias for `Reg<DIEPINT0_SPEC>`"]
pub type DIEPINT0 = crate::Reg<diepint0::DIEPINT0_SPEC>;
#[doc = "OTG device endpoint-0 interrupt register"]
pub mod diepint0;
#[doc = "DIEPINT1 register accessor: an alias for `Reg<DIEPINT1_SPEC>`"]
pub type DIEPINT1 = crate::Reg<diepint1::DIEPINT1_SPEC>;
#[doc = "OTG device endpoint-1 interrupt register"]
pub mod diepint1;
#[doc = "DIEPINT2 register accessor: an alias for `Reg<DIEPINT2_SPEC>`"]
pub type DIEPINT2 = crate::Reg<diepint2::DIEPINT2_SPEC>;
#[doc = "OTG device endpoint-2 interrupt register"]
pub mod diepint2;
#[doc = "DIEPINT3 register accessor: an alias for `Reg<DIEPINT3_SPEC>`"]
pub type DIEPINT3 = crate::Reg<diepint3::DIEPINT3_SPEC>;
#[doc = "OTG device endpoint-3 interrupt register"]
pub mod diepint3;
#[doc = "DIEPINT4 register accessor: an alias for `Reg<DIEPINT4_SPEC>`"]
pub type DIEPINT4 = crate::Reg<diepint4::DIEPINT4_SPEC>;
#[doc = "OTG device endpoint-4 interrupt register"]
pub mod diepint4;
#[doc = "DIEPINT5 register accessor: an alias for `Reg<DIEPINT5_SPEC>`"]
pub type DIEPINT5 = crate::Reg<diepint5::DIEPINT5_SPEC>;
#[doc = "OTG device endpoint-5 interrupt register"]
pub mod diepint5;
#[doc = "DIEPINT6 register accessor: an alias for `Reg<DIEPINT6_SPEC>`"]
pub type DIEPINT6 = crate::Reg<diepint6::DIEPINT6_SPEC>;
#[doc = "OTG device endpoint-6 interrupt register"]
pub mod diepint6;
#[doc = "DIEPINT7 register accessor: an alias for `Reg<DIEPINT7_SPEC>`"]
pub type DIEPINT7 = crate::Reg<diepint7::DIEPINT7_SPEC>;
#[doc = "OTG device endpoint-7 interrupt register"]
pub mod diepint7;
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
#[doc = "DTXFSTS0 register accessor: an alias for `Reg<DTXFSTS0_SPEC>`"]
pub type DTXFSTS0 = crate::Reg<dtxfsts0::DTXFSTS0_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts0;
#[doc = "DTXFSTS1 register accessor: an alias for `Reg<DTXFSTS1_SPEC>`"]
pub type DTXFSTS1 = crate::Reg<dtxfsts1::DTXFSTS1_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts1;
#[doc = "DTXFSTS2 register accessor: an alias for `Reg<DTXFSTS2_SPEC>`"]
pub type DTXFSTS2 = crate::Reg<dtxfsts2::DTXFSTS2_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts2;
#[doc = "DTXFSTS3 register accessor: an alias for `Reg<DTXFSTS3_SPEC>`"]
pub type DTXFSTS3 = crate::Reg<dtxfsts3::DTXFSTS3_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts3;
#[doc = "DTXFSTS4 register accessor: an alias for `Reg<DTXFSTS4_SPEC>`"]
pub type DTXFSTS4 = crate::Reg<dtxfsts4::DTXFSTS4_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts4;
#[doc = "DTXFSTS5 register accessor: an alias for `Reg<DTXFSTS5_SPEC>`"]
pub type DTXFSTS5 = crate::Reg<dtxfsts5::DTXFSTS5_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts5;
#[doc = "DIEPTSIZ1 register accessor: an alias for `Reg<DIEPTSIZ1_SPEC>`"]
pub type DIEPTSIZ1 = crate::Reg<dieptsiz1::DIEPTSIZ1_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz1;
#[doc = "DIEPTSIZ2 register accessor: an alias for `Reg<DIEPTSIZ2_SPEC>`"]
pub type DIEPTSIZ2 = crate::Reg<dieptsiz2::DIEPTSIZ2_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz2;
#[doc = "DIEPTSIZ3 register accessor: an alias for `Reg<DIEPTSIZ3_SPEC>`"]
pub type DIEPTSIZ3 = crate::Reg<dieptsiz3::DIEPTSIZ3_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz3;
#[doc = "DIEPTSIZ4 register accessor: an alias for `Reg<DIEPTSIZ4_SPEC>`"]
pub type DIEPTSIZ4 = crate::Reg<dieptsiz4::DIEPTSIZ4_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz4;
#[doc = "DIEPTSIZ5 register accessor: an alias for `Reg<DIEPTSIZ5_SPEC>`"]
pub type DIEPTSIZ5 = crate::Reg<dieptsiz5::DIEPTSIZ5_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz5;
#[doc = "DOEPCTL0 register accessor: an alias for `Reg<DOEPCTL0_SPEC>`"]
pub type DOEPCTL0 = crate::Reg<doepctl0::DOEPCTL0_SPEC>;
#[doc = "OTG_HS device control OUT endpoint 0 control register"]
pub mod doepctl0;
#[doc = "DOEPCTL1 register accessor: an alias for `Reg<DOEPCTL1_SPEC>`"]
pub type DOEPCTL1 = crate::Reg<doepctl1::DOEPCTL1_SPEC>;
#[doc = "OTG device endpoint-1 control register"]
pub mod doepctl1;
#[doc = "DOEPCTL2 register accessor: an alias for `Reg<DOEPCTL2_SPEC>`"]
pub type DOEPCTL2 = crate::Reg<doepctl2::DOEPCTL2_SPEC>;
#[doc = "OTG device endpoint-2 control register"]
pub mod doepctl2;
#[doc = "DOEPCTL3 register accessor: an alias for `Reg<DOEPCTL3_SPEC>`"]
pub type DOEPCTL3 = crate::Reg<doepctl3::DOEPCTL3_SPEC>;
#[doc = "OTG device endpoint-3 control register"]
pub mod doepctl3;
#[doc = "DOEPINT0 register accessor: an alias for `Reg<DOEPINT0_SPEC>`"]
pub type DOEPINT0 = crate::Reg<doepint0::DOEPINT0_SPEC>;
#[doc = "OTG_HS device endpoint-0 interrupt register"]
pub mod doepint0;
#[doc = "DOEPINT1 register accessor: an alias for `Reg<DOEPINT1_SPEC>`"]
pub type DOEPINT1 = crate::Reg<doepint1::DOEPINT1_SPEC>;
#[doc = "OTG_HS device endpoint-1 interrupt register"]
pub mod doepint1;
#[doc = "DOEPINT2 register accessor: an alias for `Reg<DOEPINT2_SPEC>`"]
pub type DOEPINT2 = crate::Reg<doepint2::DOEPINT2_SPEC>;
#[doc = "OTG_HS device endpoint-2 interrupt register"]
pub mod doepint2;
#[doc = "DOEPINT3 register accessor: an alias for `Reg<DOEPINT3_SPEC>`"]
pub type DOEPINT3 = crate::Reg<doepint3::DOEPINT3_SPEC>;
#[doc = "OTG_HS device endpoint-3 interrupt register"]
pub mod doepint3;
#[doc = "DOEPINT4 register accessor: an alias for `Reg<DOEPINT4_SPEC>`"]
pub type DOEPINT4 = crate::Reg<doepint4::DOEPINT4_SPEC>;
#[doc = "OTG_HS device endpoint-4 interrupt register"]
pub mod doepint4;
#[doc = "DOEPINT5 register accessor: an alias for `Reg<DOEPINT5_SPEC>`"]
pub type DOEPINT5 = crate::Reg<doepint5::DOEPINT5_SPEC>;
#[doc = "OTG_HS device endpoint-5 interrupt register"]
pub mod doepint5;
#[doc = "DOEPINT6 register accessor: an alias for `Reg<DOEPINT6_SPEC>`"]
pub type DOEPINT6 = crate::Reg<doepint6::DOEPINT6_SPEC>;
#[doc = "OTG_HS device endpoint-6 interrupt register"]
pub mod doepint6;
#[doc = "DOEPINT7 register accessor: an alias for `Reg<DOEPINT7_SPEC>`"]
pub type DOEPINT7 = crate::Reg<doepint7::DOEPINT7_SPEC>;
#[doc = "OTG_HS device endpoint-7 interrupt register"]
pub mod doepint7;
#[doc = "DOEPTSIZ0 register accessor: an alias for `Reg<DOEPTSIZ0_SPEC>`"]
pub type DOEPTSIZ0 = crate::Reg<doeptsiz0::DOEPTSIZ0_SPEC>;
#[doc = "OTG_HS device endpoint-1 transfer size register"]
pub mod doeptsiz0;
#[doc = "DOEPTSIZ1 register accessor: an alias for `Reg<DOEPTSIZ1_SPEC>`"]
pub type DOEPTSIZ1 = crate::Reg<doeptsiz1::DOEPTSIZ1_SPEC>;
#[doc = "OTG_HS device endpoint-2 transfer size register"]
pub mod doeptsiz1;
#[doc = "DOEPTSIZ2 register accessor: an alias for `Reg<DOEPTSIZ2_SPEC>`"]
pub type DOEPTSIZ2 = crate::Reg<doeptsiz2::DOEPTSIZ2_SPEC>;
#[doc = "OTG_HS device endpoint-3 transfer size register"]
pub mod doeptsiz2;
#[doc = "DOEPTSIZ3 register accessor: an alias for `Reg<DOEPTSIZ3_SPEC>`"]
pub type DOEPTSIZ3 = crate::Reg<doeptsiz3::DOEPTSIZ3_SPEC>;
#[doc = "OTG_HS device endpoint-4 transfer size register"]
pub mod doeptsiz3;
#[doc = "DOEPTSIZ4 register accessor: an alias for `Reg<DOEPTSIZ4_SPEC>`"]
pub type DOEPTSIZ4 = crate::Reg<doeptsiz4::DOEPTSIZ4_SPEC>;
#[doc = "OTG_HS device endpoint-5 transfer size register"]
pub mod doeptsiz4;
