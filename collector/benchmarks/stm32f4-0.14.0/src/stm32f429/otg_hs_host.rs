#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS host configuration register"]
    pub hcfg: crate::Reg<hcfg::HCFG_SPEC>,
    #[doc = "0x04 - OTG_HS Host frame interval register"]
    pub hfir: crate::Reg<hfir::HFIR_SPEC>,
    #[doc = "0x08 - OTG_HS host frame number/frame time remaining register"]
    pub hfnum: crate::Reg<hfnum::HFNUM_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - OTG_HS_Host periodic transmit FIFO/queue status register"]
    pub hptxsts: crate::Reg<hptxsts::HPTXSTS_SPEC>,
    #[doc = "0x14 - OTG_HS Host all channels interrupt register"]
    pub haint: crate::Reg<haint::HAINT_SPEC>,
    #[doc = "0x18 - OTG_HS host all channels interrupt mask register"]
    pub haintmsk: crate::Reg<haintmsk::HAINTMSK_SPEC>,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - OTG_HS host port control and status register"]
    pub hprt: crate::Reg<hprt::HPRT_SPEC>,
    _reserved7: [u8; 0xbc],
    #[doc = "0x100 - OTG_HS host channel-0 characteristics register"]
    pub hcchar0: crate::Reg<hcchar0::HCCHAR0_SPEC>,
    #[doc = "0x104 - OTG_HS host channel-0 split control register"]
    pub hcsplt0: crate::Reg<hcsplt0::HCSPLT0_SPEC>,
    #[doc = "0x108 - OTG_HS host channel-11 interrupt register"]
    pub hcint0: crate::Reg<hcint0::HCINT0_SPEC>,
    #[doc = "0x10c - OTG_HS host channel-11 interrupt mask register"]
    pub hcintmsk0: crate::Reg<hcintmsk0::HCINTMSK0_SPEC>,
    #[doc = "0x110 - OTG_HS host channel-11 transfer size register"]
    pub hctsiz0: crate::Reg<hctsiz0::HCTSIZ0_SPEC>,
    #[doc = "0x114 - OTG_HS host channel-0 DMA address register"]
    pub hcdma0: crate::Reg<hcdma0::HCDMA0_SPEC>,
    _reserved13: [u8; 0x08],
    #[doc = "0x120 - OTG_HS host channel-1 characteristics register"]
    pub hcchar1: crate::Reg<hcchar1::HCCHAR1_SPEC>,
    #[doc = "0x124 - OTG_HS host channel-1 split control register"]
    pub hcsplt1: crate::Reg<hcsplt1::HCSPLT1_SPEC>,
    #[doc = "0x128 - OTG_HS host channel-1 interrupt register"]
    pub hcint1: crate::Reg<hcint1::HCINT1_SPEC>,
    #[doc = "0x12c - OTG_HS host channel-1 interrupt mask register"]
    pub hcintmsk1: crate::Reg<hcintmsk1::HCINTMSK1_SPEC>,
    #[doc = "0x130 - OTG_HS host channel-1 transfer size register"]
    pub hctsiz1: crate::Reg<hctsiz1::HCTSIZ1_SPEC>,
    #[doc = "0x134 - OTG_HS host channel-1 DMA address register"]
    pub hcdma1: crate::Reg<hcdma1::HCDMA1_SPEC>,
    _reserved19: [u8; 0x08],
    #[doc = "0x140 - OTG_HS host channel-2 characteristics register"]
    pub hcchar2: crate::Reg<hcchar2::HCCHAR2_SPEC>,
    #[doc = "0x144 - OTG_HS host channel-2 split control register"]
    pub hcsplt2: crate::Reg<hcsplt2::HCSPLT2_SPEC>,
    #[doc = "0x148 - OTG_HS host channel-2 interrupt register"]
    pub hcint2: crate::Reg<hcint2::HCINT2_SPEC>,
    #[doc = "0x14c - OTG_HS host channel-2 interrupt mask register"]
    pub hcintmsk2: crate::Reg<hcintmsk2::HCINTMSK2_SPEC>,
    #[doc = "0x150 - OTG_HS host channel-2 transfer size register"]
    pub hctsiz2: crate::Reg<hctsiz2::HCTSIZ2_SPEC>,
    #[doc = "0x154 - OTG_HS host channel-2 DMA address register"]
    pub hcdma2: crate::Reg<hcdma2::HCDMA2_SPEC>,
    _reserved25: [u8; 0x08],
    #[doc = "0x160 - OTG_HS host channel-3 characteristics register"]
    pub hcchar3: crate::Reg<hcchar3::HCCHAR3_SPEC>,
    #[doc = "0x164 - OTG_HS host channel-3 split control register"]
    pub hcsplt3: crate::Reg<hcsplt3::HCSPLT3_SPEC>,
    #[doc = "0x168 - OTG_HS host channel-3 interrupt register"]
    pub hcint3: crate::Reg<hcint3::HCINT3_SPEC>,
    #[doc = "0x16c - OTG_HS host channel-3 interrupt mask register"]
    pub hcintmsk3: crate::Reg<hcintmsk3::HCINTMSK3_SPEC>,
    #[doc = "0x170 - OTG_HS host channel-3 transfer size register"]
    pub hctsiz3: crate::Reg<hctsiz3::HCTSIZ3_SPEC>,
    #[doc = "0x174 - OTG_HS host channel-3 DMA address register"]
    pub hcdma3: crate::Reg<hcdma3::HCDMA3_SPEC>,
    _reserved31: [u8; 0x08],
    #[doc = "0x180 - OTG_HS host channel-4 characteristics register"]
    pub hcchar4: crate::Reg<hcchar4::HCCHAR4_SPEC>,
    #[doc = "0x184 - OTG_HS host channel-4 split control register"]
    pub hcsplt4: crate::Reg<hcsplt4::HCSPLT4_SPEC>,
    #[doc = "0x188 - OTG_HS host channel-4 interrupt register"]
    pub hcint4: crate::Reg<hcint4::HCINT4_SPEC>,
    #[doc = "0x18c - OTG_HS host channel-4 interrupt mask register"]
    pub hcintmsk4: crate::Reg<hcintmsk4::HCINTMSK4_SPEC>,
    #[doc = "0x190 - OTG_HS host channel-4 transfer size register"]
    pub hctsiz4: crate::Reg<hctsiz4::HCTSIZ4_SPEC>,
    #[doc = "0x194 - OTG_HS host channel-4 DMA address register"]
    pub hcdma4: crate::Reg<hcdma4::HCDMA4_SPEC>,
    _reserved37: [u8; 0x08],
    #[doc = "0x1a0 - OTG_HS host channel-5 characteristics register"]
    pub hcchar5: crate::Reg<hcchar5::HCCHAR5_SPEC>,
    #[doc = "0x1a4 - OTG_HS host channel-5 split control register"]
    pub hcsplt5: crate::Reg<hcsplt5::HCSPLT5_SPEC>,
    #[doc = "0x1a8 - OTG_HS host channel-5 interrupt register"]
    pub hcint5: crate::Reg<hcint5::HCINT5_SPEC>,
    #[doc = "0x1ac - OTG_HS host channel-5 interrupt mask register"]
    pub hcintmsk5: crate::Reg<hcintmsk5::HCINTMSK5_SPEC>,
    #[doc = "0x1b0 - OTG_HS host channel-5 transfer size register"]
    pub hctsiz5: crate::Reg<hctsiz5::HCTSIZ5_SPEC>,
    #[doc = "0x1b4 - OTG_HS host channel-5 DMA address register"]
    pub hcdma5: crate::Reg<hcdma5::HCDMA5_SPEC>,
    _reserved43: [u8; 0x08],
    #[doc = "0x1c0 - OTG_HS host channel-6 characteristics register"]
    pub hcchar6: crate::Reg<hcchar6::HCCHAR6_SPEC>,
    #[doc = "0x1c4 - OTG_HS host channel-6 split control register"]
    pub hcsplt6: crate::Reg<hcsplt6::HCSPLT6_SPEC>,
    #[doc = "0x1c8 - OTG_HS host channel-6 interrupt register"]
    pub hcint6: crate::Reg<hcint6::HCINT6_SPEC>,
    #[doc = "0x1cc - OTG_HS host channel-6 interrupt mask register"]
    pub hcintmsk6: crate::Reg<hcintmsk6::HCINTMSK6_SPEC>,
    #[doc = "0x1d0 - OTG_HS host channel-6 transfer size register"]
    pub hctsiz6: crate::Reg<hctsiz6::HCTSIZ6_SPEC>,
    #[doc = "0x1d4 - OTG_HS host channel-6 DMA address register"]
    pub hcdma6: crate::Reg<hcdma6::HCDMA6_SPEC>,
    _reserved49: [u8; 0x08],
    #[doc = "0x1e0 - OTG_HS host channel-7 characteristics register"]
    pub hcchar7: crate::Reg<hcchar7::HCCHAR7_SPEC>,
    #[doc = "0x1e4 - OTG_HS host channel-7 split control register"]
    pub hcsplt7: crate::Reg<hcsplt7::HCSPLT7_SPEC>,
    #[doc = "0x1e8 - OTG_HS host channel-7 interrupt register"]
    pub hcint7: crate::Reg<hcint7::HCINT7_SPEC>,
    #[doc = "0x1ec - OTG_HS host channel-7 interrupt mask register"]
    pub hcintmsk7: crate::Reg<hcintmsk7::HCINTMSK7_SPEC>,
    #[doc = "0x1f0 - OTG_HS host channel-7 transfer size register"]
    pub hctsiz7: crate::Reg<hctsiz7::HCTSIZ7_SPEC>,
    #[doc = "0x1f4 - OTG_HS host channel-7 DMA address register"]
    pub hcdma7: crate::Reg<hcdma7::HCDMA7_SPEC>,
    _reserved55: [u8; 0x08],
    #[doc = "0x200 - OTG_HS host channel-8 characteristics register"]
    pub hcchar8: crate::Reg<hcchar8::HCCHAR8_SPEC>,
    #[doc = "0x204 - OTG_HS host channel-8 split control register"]
    pub hcsplt8: crate::Reg<hcsplt8::HCSPLT8_SPEC>,
    #[doc = "0x208 - OTG_HS host channel-8 interrupt register"]
    pub hcint8: crate::Reg<hcint8::HCINT8_SPEC>,
    #[doc = "0x20c - OTG_HS host channel-8 interrupt mask register"]
    pub hcintmsk8: crate::Reg<hcintmsk8::HCINTMSK8_SPEC>,
    #[doc = "0x210 - OTG_HS host channel-8 transfer size register"]
    pub hctsiz8: crate::Reg<hctsiz8::HCTSIZ8_SPEC>,
    #[doc = "0x214 - OTG_HS host channel-8 DMA address register"]
    pub hcdma8: crate::Reg<hcdma8::HCDMA8_SPEC>,
    _reserved61: [u8; 0x08],
    #[doc = "0x220 - OTG_HS host channel-9 characteristics register"]
    pub hcchar9: crate::Reg<hcchar9::HCCHAR9_SPEC>,
    #[doc = "0x224 - OTG_HS host channel-9 split control register"]
    pub hcsplt9: crate::Reg<hcsplt9::HCSPLT9_SPEC>,
    #[doc = "0x228 - OTG_HS host channel-9 interrupt register"]
    pub hcint9: crate::Reg<hcint9::HCINT9_SPEC>,
    #[doc = "0x22c - OTG_HS host channel-9 interrupt mask register"]
    pub hcintmsk9: crate::Reg<hcintmsk9::HCINTMSK9_SPEC>,
    #[doc = "0x230 - OTG_HS host channel-9 transfer size register"]
    pub hctsiz9: crate::Reg<hctsiz9::HCTSIZ9_SPEC>,
    #[doc = "0x234 - OTG_HS host channel-9 DMA address register"]
    pub hcdma9: crate::Reg<hcdma9::HCDMA9_SPEC>,
    _reserved67: [u8; 0x08],
    #[doc = "0x240 - OTG_HS host channel-10 characteristics register"]
    pub hcchar10: crate::Reg<hcchar10::HCCHAR10_SPEC>,
    #[doc = "0x244 - OTG_HS host channel-10 split control register"]
    pub hcsplt10: crate::Reg<hcsplt10::HCSPLT10_SPEC>,
    #[doc = "0x248 - OTG_HS host channel-10 interrupt register"]
    pub hcint10: crate::Reg<hcint10::HCINT10_SPEC>,
    #[doc = "0x24c - OTG_HS host channel-10 interrupt mask register"]
    pub hcintmsk10: crate::Reg<hcintmsk10::HCINTMSK10_SPEC>,
    #[doc = "0x250 - OTG_HS host channel-10 transfer size register"]
    pub hctsiz10: crate::Reg<hctsiz10::HCTSIZ10_SPEC>,
    #[doc = "0x254 - OTG_HS host channel-10 DMA address register"]
    pub hcdma10: crate::Reg<hcdma10::HCDMA10_SPEC>,
    _reserved73: [u8; 0x08],
    #[doc = "0x260 - OTG_HS host channel-11 characteristics register"]
    pub hcchar11: crate::Reg<hcchar11::HCCHAR11_SPEC>,
    #[doc = "0x264 - OTG_HS host channel-11 split control register"]
    pub hcsplt11: crate::Reg<hcsplt11::HCSPLT11_SPEC>,
    #[doc = "0x268 - OTG_HS host channel-11 interrupt register"]
    pub hcint11: crate::Reg<hcint11::HCINT11_SPEC>,
    #[doc = "0x26c - OTG_HS host channel-11 interrupt mask register"]
    pub hcintmsk11: crate::Reg<hcintmsk11::HCINTMSK11_SPEC>,
    #[doc = "0x270 - OTG_HS host channel-11 transfer size register"]
    pub hctsiz11: crate::Reg<hctsiz11::HCTSIZ11_SPEC>,
    #[doc = "0x274 - OTG_HS host channel-11 DMA address register"]
    pub hcdma11: crate::Reg<hcdma11::HCDMA11_SPEC>,
}
#[doc = "HCFG register accessor: an alias for `Reg<HCFG_SPEC>`"]
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
#[doc = "OTG_HS host configuration register"]
pub mod hcfg;
#[doc = "HFIR register accessor: an alias for `Reg<HFIR_SPEC>`"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = "OTG_HS Host frame interval register"]
pub mod hfir;
#[doc = "HFNUM register accessor: an alias for `Reg<HFNUM_SPEC>`"]
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
#[doc = "OTG_HS host frame number/frame time remaining register"]
pub mod hfnum;
#[doc = "HPTXSTS register accessor: an alias for `Reg<HPTXSTS_SPEC>`"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
#[doc = "OTG_HS_Host periodic transmit FIFO/queue status register"]
pub mod hptxsts;
#[doc = "HAINT register accessor: an alias for `Reg<HAINT_SPEC>`"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = "OTG_HS Host all channels interrupt register"]
pub mod haint;
#[doc = "HAINTMSK register accessor: an alias for `Reg<HAINTMSK_SPEC>`"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = "OTG_HS host all channels interrupt mask register"]
pub mod haintmsk;
#[doc = "HPRT register accessor: an alias for `Reg<HPRT_SPEC>`"]
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
#[doc = "OTG_HS host port control and status register"]
pub mod hprt;
#[doc = "HCCHAR0 register accessor: an alias for `Reg<HCCHAR0_SPEC>`"]
pub type HCCHAR0 = crate::Reg<hcchar0::HCCHAR0_SPEC>;
#[doc = "OTG_HS host channel-0 characteristics register"]
pub mod hcchar0;
#[doc = "HCCHAR1 register accessor: an alias for `Reg<HCCHAR1_SPEC>`"]
pub type HCCHAR1 = crate::Reg<hcchar1::HCCHAR1_SPEC>;
#[doc = "OTG_HS host channel-1 characteristics register"]
pub mod hcchar1;
#[doc = "HCCHAR2 register accessor: an alias for `Reg<HCCHAR2_SPEC>`"]
pub type HCCHAR2 = crate::Reg<hcchar2::HCCHAR2_SPEC>;
#[doc = "OTG_HS host channel-2 characteristics register"]
pub mod hcchar2;
#[doc = "HCCHAR3 register accessor: an alias for `Reg<HCCHAR3_SPEC>`"]
pub type HCCHAR3 = crate::Reg<hcchar3::HCCHAR3_SPEC>;
#[doc = "OTG_HS host channel-3 characteristics register"]
pub mod hcchar3;
#[doc = "HCCHAR4 register accessor: an alias for `Reg<HCCHAR4_SPEC>`"]
pub type HCCHAR4 = crate::Reg<hcchar4::HCCHAR4_SPEC>;
#[doc = "OTG_HS host channel-4 characteristics register"]
pub mod hcchar4;
#[doc = "HCCHAR5 register accessor: an alias for `Reg<HCCHAR5_SPEC>`"]
pub type HCCHAR5 = crate::Reg<hcchar5::HCCHAR5_SPEC>;
#[doc = "OTG_HS host channel-5 characteristics register"]
pub mod hcchar5;
#[doc = "HCCHAR6 register accessor: an alias for `Reg<HCCHAR6_SPEC>`"]
pub type HCCHAR6 = crate::Reg<hcchar6::HCCHAR6_SPEC>;
#[doc = "OTG_HS host channel-6 characteristics register"]
pub mod hcchar6;
#[doc = "HCCHAR7 register accessor: an alias for `Reg<HCCHAR7_SPEC>`"]
pub type HCCHAR7 = crate::Reg<hcchar7::HCCHAR7_SPEC>;
#[doc = "OTG_HS host channel-7 characteristics register"]
pub mod hcchar7;
#[doc = "HCCHAR8 register accessor: an alias for `Reg<HCCHAR8_SPEC>`"]
pub type HCCHAR8 = crate::Reg<hcchar8::HCCHAR8_SPEC>;
#[doc = "OTG_HS host channel-8 characteristics register"]
pub mod hcchar8;
#[doc = "HCCHAR9 register accessor: an alias for `Reg<HCCHAR9_SPEC>`"]
pub type HCCHAR9 = crate::Reg<hcchar9::HCCHAR9_SPEC>;
#[doc = "OTG_HS host channel-9 characteristics register"]
pub mod hcchar9;
#[doc = "HCCHAR10 register accessor: an alias for `Reg<HCCHAR10_SPEC>`"]
pub type HCCHAR10 = crate::Reg<hcchar10::HCCHAR10_SPEC>;
#[doc = "OTG_HS host channel-10 characteristics register"]
pub mod hcchar10;
#[doc = "HCCHAR11 register accessor: an alias for `Reg<HCCHAR11_SPEC>`"]
pub type HCCHAR11 = crate::Reg<hcchar11::HCCHAR11_SPEC>;
#[doc = "OTG_HS host channel-11 characteristics register"]
pub mod hcchar11;
#[doc = "HCSPLT0 register accessor: an alias for `Reg<HCSPLT0_SPEC>`"]
pub type HCSPLT0 = crate::Reg<hcsplt0::HCSPLT0_SPEC>;
#[doc = "OTG_HS host channel-0 split control register"]
pub mod hcsplt0;
#[doc = "HCSPLT1 register accessor: an alias for `Reg<HCSPLT1_SPEC>`"]
pub type HCSPLT1 = crate::Reg<hcsplt1::HCSPLT1_SPEC>;
#[doc = "OTG_HS host channel-1 split control register"]
pub mod hcsplt1;
#[doc = "HCSPLT2 register accessor: an alias for `Reg<HCSPLT2_SPEC>`"]
pub type HCSPLT2 = crate::Reg<hcsplt2::HCSPLT2_SPEC>;
#[doc = "OTG_HS host channel-2 split control register"]
pub mod hcsplt2;
#[doc = "HCSPLT3 register accessor: an alias for `Reg<HCSPLT3_SPEC>`"]
pub type HCSPLT3 = crate::Reg<hcsplt3::HCSPLT3_SPEC>;
#[doc = "OTG_HS host channel-3 split control register"]
pub mod hcsplt3;
#[doc = "HCSPLT4 register accessor: an alias for `Reg<HCSPLT4_SPEC>`"]
pub type HCSPLT4 = crate::Reg<hcsplt4::HCSPLT4_SPEC>;
#[doc = "OTG_HS host channel-4 split control register"]
pub mod hcsplt4;
#[doc = "HCSPLT5 register accessor: an alias for `Reg<HCSPLT5_SPEC>`"]
pub type HCSPLT5 = crate::Reg<hcsplt5::HCSPLT5_SPEC>;
#[doc = "OTG_HS host channel-5 split control register"]
pub mod hcsplt5;
#[doc = "HCSPLT6 register accessor: an alias for `Reg<HCSPLT6_SPEC>`"]
pub type HCSPLT6 = crate::Reg<hcsplt6::HCSPLT6_SPEC>;
#[doc = "OTG_HS host channel-6 split control register"]
pub mod hcsplt6;
#[doc = "HCSPLT7 register accessor: an alias for `Reg<HCSPLT7_SPEC>`"]
pub type HCSPLT7 = crate::Reg<hcsplt7::HCSPLT7_SPEC>;
#[doc = "OTG_HS host channel-7 split control register"]
pub mod hcsplt7;
#[doc = "HCSPLT8 register accessor: an alias for `Reg<HCSPLT8_SPEC>`"]
pub type HCSPLT8 = crate::Reg<hcsplt8::HCSPLT8_SPEC>;
#[doc = "OTG_HS host channel-8 split control register"]
pub mod hcsplt8;
#[doc = "HCSPLT9 register accessor: an alias for `Reg<HCSPLT9_SPEC>`"]
pub type HCSPLT9 = crate::Reg<hcsplt9::HCSPLT9_SPEC>;
#[doc = "OTG_HS host channel-9 split control register"]
pub mod hcsplt9;
#[doc = "HCSPLT10 register accessor: an alias for `Reg<HCSPLT10_SPEC>`"]
pub type HCSPLT10 = crate::Reg<hcsplt10::HCSPLT10_SPEC>;
#[doc = "OTG_HS host channel-10 split control register"]
pub mod hcsplt10;
#[doc = "HCSPLT11 register accessor: an alias for `Reg<HCSPLT11_SPEC>`"]
pub type HCSPLT11 = crate::Reg<hcsplt11::HCSPLT11_SPEC>;
#[doc = "OTG_HS host channel-11 split control register"]
pub mod hcsplt11;
#[doc = "HCINT0 register accessor: an alias for `Reg<HCINT0_SPEC>`"]
pub type HCINT0 = crate::Reg<hcint0::HCINT0_SPEC>;
#[doc = "OTG_HS host channel-11 interrupt register"]
pub mod hcint0;
#[doc = "HCINT1 register accessor: an alias for `Reg<HCINT1_SPEC>`"]
pub type HCINT1 = crate::Reg<hcint1::HCINT1_SPEC>;
#[doc = "OTG_HS host channel-1 interrupt register"]
pub mod hcint1;
#[doc = "HCINT2 register accessor: an alias for `Reg<HCINT2_SPEC>`"]
pub type HCINT2 = crate::Reg<hcint2::HCINT2_SPEC>;
#[doc = "OTG_HS host channel-2 interrupt register"]
pub mod hcint2;
#[doc = "HCINT3 register accessor: an alias for `Reg<HCINT3_SPEC>`"]
pub type HCINT3 = crate::Reg<hcint3::HCINT3_SPEC>;
#[doc = "OTG_HS host channel-3 interrupt register"]
pub mod hcint3;
#[doc = "HCINT4 register accessor: an alias for `Reg<HCINT4_SPEC>`"]
pub type HCINT4 = crate::Reg<hcint4::HCINT4_SPEC>;
#[doc = "OTG_HS host channel-4 interrupt register"]
pub mod hcint4;
#[doc = "HCINT5 register accessor: an alias for `Reg<HCINT5_SPEC>`"]
pub type HCINT5 = crate::Reg<hcint5::HCINT5_SPEC>;
#[doc = "OTG_HS host channel-5 interrupt register"]
pub mod hcint5;
#[doc = "HCINT6 register accessor: an alias for `Reg<HCINT6_SPEC>`"]
pub type HCINT6 = crate::Reg<hcint6::HCINT6_SPEC>;
#[doc = "OTG_HS host channel-6 interrupt register"]
pub mod hcint6;
#[doc = "HCINT7 register accessor: an alias for `Reg<HCINT7_SPEC>`"]
pub type HCINT7 = crate::Reg<hcint7::HCINT7_SPEC>;
#[doc = "OTG_HS host channel-7 interrupt register"]
pub mod hcint7;
#[doc = "HCINT8 register accessor: an alias for `Reg<HCINT8_SPEC>`"]
pub type HCINT8 = crate::Reg<hcint8::HCINT8_SPEC>;
#[doc = "OTG_HS host channel-8 interrupt register"]
pub mod hcint8;
#[doc = "HCINT9 register accessor: an alias for `Reg<HCINT9_SPEC>`"]
pub type HCINT9 = crate::Reg<hcint9::HCINT9_SPEC>;
#[doc = "OTG_HS host channel-9 interrupt register"]
pub mod hcint9;
#[doc = "HCINT10 register accessor: an alias for `Reg<HCINT10_SPEC>`"]
pub type HCINT10 = crate::Reg<hcint10::HCINT10_SPEC>;
#[doc = "OTG_HS host channel-10 interrupt register"]
pub mod hcint10;
#[doc = "HCINT11 register accessor: an alias for `Reg<HCINT11_SPEC>`"]
pub type HCINT11 = crate::Reg<hcint11::HCINT11_SPEC>;
#[doc = "OTG_HS host channel-11 interrupt register"]
pub mod hcint11;
#[doc = "HCINTMSK0 register accessor: an alias for `Reg<HCINTMSK0_SPEC>`"]
pub type HCINTMSK0 = crate::Reg<hcintmsk0::HCINTMSK0_SPEC>;
#[doc = "OTG_HS host channel-11 interrupt mask register"]
pub mod hcintmsk0;
#[doc = "HCINTMSK1 register accessor: an alias for `Reg<HCINTMSK1_SPEC>`"]
pub type HCINTMSK1 = crate::Reg<hcintmsk1::HCINTMSK1_SPEC>;
#[doc = "OTG_HS host channel-1 interrupt mask register"]
pub mod hcintmsk1;
#[doc = "HCINTMSK2 register accessor: an alias for `Reg<HCINTMSK2_SPEC>`"]
pub type HCINTMSK2 = crate::Reg<hcintmsk2::HCINTMSK2_SPEC>;
#[doc = "OTG_HS host channel-2 interrupt mask register"]
pub mod hcintmsk2;
#[doc = "HCINTMSK3 register accessor: an alias for `Reg<HCINTMSK3_SPEC>`"]
pub type HCINTMSK3 = crate::Reg<hcintmsk3::HCINTMSK3_SPEC>;
#[doc = "OTG_HS host channel-3 interrupt mask register"]
pub mod hcintmsk3;
#[doc = "HCINTMSK4 register accessor: an alias for `Reg<HCINTMSK4_SPEC>`"]
pub type HCINTMSK4 = crate::Reg<hcintmsk4::HCINTMSK4_SPEC>;
#[doc = "OTG_HS host channel-4 interrupt mask register"]
pub mod hcintmsk4;
#[doc = "HCINTMSK5 register accessor: an alias for `Reg<HCINTMSK5_SPEC>`"]
pub type HCINTMSK5 = crate::Reg<hcintmsk5::HCINTMSK5_SPEC>;
#[doc = "OTG_HS host channel-5 interrupt mask register"]
pub mod hcintmsk5;
#[doc = "HCINTMSK6 register accessor: an alias for `Reg<HCINTMSK6_SPEC>`"]
pub type HCINTMSK6 = crate::Reg<hcintmsk6::HCINTMSK6_SPEC>;
#[doc = "OTG_HS host channel-6 interrupt mask register"]
pub mod hcintmsk6;
#[doc = "HCINTMSK7 register accessor: an alias for `Reg<HCINTMSK7_SPEC>`"]
pub type HCINTMSK7 = crate::Reg<hcintmsk7::HCINTMSK7_SPEC>;
#[doc = "OTG_HS host channel-7 interrupt mask register"]
pub mod hcintmsk7;
#[doc = "HCINTMSK8 register accessor: an alias for `Reg<HCINTMSK8_SPEC>`"]
pub type HCINTMSK8 = crate::Reg<hcintmsk8::HCINTMSK8_SPEC>;
#[doc = "OTG_HS host channel-8 interrupt mask register"]
pub mod hcintmsk8;
#[doc = "HCINTMSK9 register accessor: an alias for `Reg<HCINTMSK9_SPEC>`"]
pub type HCINTMSK9 = crate::Reg<hcintmsk9::HCINTMSK9_SPEC>;
#[doc = "OTG_HS host channel-9 interrupt mask register"]
pub mod hcintmsk9;
#[doc = "HCINTMSK10 register accessor: an alias for `Reg<HCINTMSK10_SPEC>`"]
pub type HCINTMSK10 = crate::Reg<hcintmsk10::HCINTMSK10_SPEC>;
#[doc = "OTG_HS host channel-10 interrupt mask register"]
pub mod hcintmsk10;
#[doc = "HCINTMSK11 register accessor: an alias for `Reg<HCINTMSK11_SPEC>`"]
pub type HCINTMSK11 = crate::Reg<hcintmsk11::HCINTMSK11_SPEC>;
#[doc = "OTG_HS host channel-11 interrupt mask register"]
pub mod hcintmsk11;
#[doc = "HCTSIZ0 register accessor: an alias for `Reg<HCTSIZ0_SPEC>`"]
pub type HCTSIZ0 = crate::Reg<hctsiz0::HCTSIZ0_SPEC>;
#[doc = "OTG_HS host channel-11 transfer size register"]
pub mod hctsiz0;
#[doc = "HCTSIZ1 register accessor: an alias for `Reg<HCTSIZ1_SPEC>`"]
pub type HCTSIZ1 = crate::Reg<hctsiz1::HCTSIZ1_SPEC>;
#[doc = "OTG_HS host channel-1 transfer size register"]
pub mod hctsiz1;
#[doc = "HCTSIZ2 register accessor: an alias for `Reg<HCTSIZ2_SPEC>`"]
pub type HCTSIZ2 = crate::Reg<hctsiz2::HCTSIZ2_SPEC>;
#[doc = "OTG_HS host channel-2 transfer size register"]
pub mod hctsiz2;
#[doc = "HCTSIZ3 register accessor: an alias for `Reg<HCTSIZ3_SPEC>`"]
pub type HCTSIZ3 = crate::Reg<hctsiz3::HCTSIZ3_SPEC>;
#[doc = "OTG_HS host channel-3 transfer size register"]
pub mod hctsiz3;
#[doc = "HCTSIZ4 register accessor: an alias for `Reg<HCTSIZ4_SPEC>`"]
pub type HCTSIZ4 = crate::Reg<hctsiz4::HCTSIZ4_SPEC>;
#[doc = "OTG_HS host channel-4 transfer size register"]
pub mod hctsiz4;
#[doc = "HCTSIZ5 register accessor: an alias for `Reg<HCTSIZ5_SPEC>`"]
pub type HCTSIZ5 = crate::Reg<hctsiz5::HCTSIZ5_SPEC>;
#[doc = "OTG_HS host channel-5 transfer size register"]
pub mod hctsiz5;
#[doc = "HCTSIZ6 register accessor: an alias for `Reg<HCTSIZ6_SPEC>`"]
pub type HCTSIZ6 = crate::Reg<hctsiz6::HCTSIZ6_SPEC>;
#[doc = "OTG_HS host channel-6 transfer size register"]
pub mod hctsiz6;
#[doc = "HCTSIZ7 register accessor: an alias for `Reg<HCTSIZ7_SPEC>`"]
pub type HCTSIZ7 = crate::Reg<hctsiz7::HCTSIZ7_SPEC>;
#[doc = "OTG_HS host channel-7 transfer size register"]
pub mod hctsiz7;
#[doc = "HCTSIZ8 register accessor: an alias for `Reg<HCTSIZ8_SPEC>`"]
pub type HCTSIZ8 = crate::Reg<hctsiz8::HCTSIZ8_SPEC>;
#[doc = "OTG_HS host channel-8 transfer size register"]
pub mod hctsiz8;
#[doc = "HCTSIZ9 register accessor: an alias for `Reg<HCTSIZ9_SPEC>`"]
pub type HCTSIZ9 = crate::Reg<hctsiz9::HCTSIZ9_SPEC>;
#[doc = "OTG_HS host channel-9 transfer size register"]
pub mod hctsiz9;
#[doc = "HCTSIZ10 register accessor: an alias for `Reg<HCTSIZ10_SPEC>`"]
pub type HCTSIZ10 = crate::Reg<hctsiz10::HCTSIZ10_SPEC>;
#[doc = "OTG_HS host channel-10 transfer size register"]
pub mod hctsiz10;
#[doc = "HCTSIZ11 register accessor: an alias for `Reg<HCTSIZ11_SPEC>`"]
pub type HCTSIZ11 = crate::Reg<hctsiz11::HCTSIZ11_SPEC>;
#[doc = "OTG_HS host channel-11 transfer size register"]
pub mod hctsiz11;
#[doc = "HCDMA0 register accessor: an alias for `Reg<HCDMA0_SPEC>`"]
pub type HCDMA0 = crate::Reg<hcdma0::HCDMA0_SPEC>;
#[doc = "OTG_HS host channel-0 DMA address register"]
pub mod hcdma0;
#[doc = "HCDMA1 register accessor: an alias for `Reg<HCDMA1_SPEC>`"]
pub type HCDMA1 = crate::Reg<hcdma1::HCDMA1_SPEC>;
#[doc = "OTG_HS host channel-1 DMA address register"]
pub mod hcdma1;
#[doc = "HCDMA2 register accessor: an alias for `Reg<HCDMA2_SPEC>`"]
pub type HCDMA2 = crate::Reg<hcdma2::HCDMA2_SPEC>;
#[doc = "OTG_HS host channel-2 DMA address register"]
pub mod hcdma2;
#[doc = "HCDMA3 register accessor: an alias for `Reg<HCDMA3_SPEC>`"]
pub type HCDMA3 = crate::Reg<hcdma3::HCDMA3_SPEC>;
#[doc = "OTG_HS host channel-3 DMA address register"]
pub mod hcdma3;
#[doc = "HCDMA4 register accessor: an alias for `Reg<HCDMA4_SPEC>`"]
pub type HCDMA4 = crate::Reg<hcdma4::HCDMA4_SPEC>;
#[doc = "OTG_HS host channel-4 DMA address register"]
pub mod hcdma4;
#[doc = "HCDMA5 register accessor: an alias for `Reg<HCDMA5_SPEC>`"]
pub type HCDMA5 = crate::Reg<hcdma5::HCDMA5_SPEC>;
#[doc = "OTG_HS host channel-5 DMA address register"]
pub mod hcdma5;
#[doc = "HCDMA6 register accessor: an alias for `Reg<HCDMA6_SPEC>`"]
pub type HCDMA6 = crate::Reg<hcdma6::HCDMA6_SPEC>;
#[doc = "OTG_HS host channel-6 DMA address register"]
pub mod hcdma6;
#[doc = "HCDMA7 register accessor: an alias for `Reg<HCDMA7_SPEC>`"]
pub type HCDMA7 = crate::Reg<hcdma7::HCDMA7_SPEC>;
#[doc = "OTG_HS host channel-7 DMA address register"]
pub mod hcdma7;
#[doc = "HCDMA8 register accessor: an alias for `Reg<HCDMA8_SPEC>`"]
pub type HCDMA8 = crate::Reg<hcdma8::HCDMA8_SPEC>;
#[doc = "OTG_HS host channel-8 DMA address register"]
pub mod hcdma8;
#[doc = "HCDMA9 register accessor: an alias for `Reg<HCDMA9_SPEC>`"]
pub type HCDMA9 = crate::Reg<hcdma9::HCDMA9_SPEC>;
#[doc = "OTG_HS host channel-9 DMA address register"]
pub mod hcdma9;
#[doc = "HCDMA10 register accessor: an alias for `Reg<HCDMA10_SPEC>`"]
pub type HCDMA10 = crate::Reg<hcdma10::HCDMA10_SPEC>;
#[doc = "OTG_HS host channel-10 DMA address register"]
pub mod hcdma10;
#[doc = "HCDMA11 register accessor: an alias for `Reg<HCDMA11_SPEC>`"]
pub type HCDMA11 = crate::Reg<hcdma11::HCDMA11_SPEC>;
#[doc = "OTG_HS host channel-11 DMA address register"]
pub mod hcdma11;
