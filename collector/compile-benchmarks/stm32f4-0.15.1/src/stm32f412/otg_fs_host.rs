#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS host configuration register (OTG_FS_HCFG)"]
    pub hcfg: crate::Reg<hcfg::HCFG_SPEC>,
    #[doc = "0x04 - OTG_FS Host frame interval register"]
    pub hfir: crate::Reg<hfir::HFIR_SPEC>,
    #[doc = "0x08 - OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)"]
    pub hfnum: crate::Reg<hfnum::HFNUM_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)"]
    pub hptxsts: crate::Reg<hptxsts::HPTXSTS_SPEC>,
    #[doc = "0x14 - OTG_FS Host all channels interrupt register"]
    pub haint: crate::Reg<haint::HAINT_SPEC>,
    #[doc = "0x18 - OTG_FS host all channels interrupt mask register"]
    pub haintmsk: crate::Reg<haintmsk::HAINTMSK_SPEC>,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - OTG_FS host port control and status register (OTG_FS_HPRT)"]
    pub hprt: crate::Reg<hprt::HPRT_SPEC>,
    _reserved7: [u8; 0xbc],
    #[doc = "0x100 - OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)"]
    pub hcchar0: crate::Reg<hcchar0::HCCHAR0_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x108 - OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)"]
    pub hcint0: crate::Reg<hcint0::HCINT0_SPEC>,
    #[doc = "0x10c - OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)"]
    pub hcintmsk0: crate::Reg<hcintmsk0::HCINTMSK0_SPEC>,
    #[doc = "0x110 - OTG_FS host channel-0 transfer size register"]
    pub hctsiz0: crate::Reg<hctsiz0::HCTSIZ0_SPEC>,
    _reserved11: [u8; 0x0c],
    #[doc = "0x120 - OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)"]
    pub hcchar1: crate::Reg<hcchar1::HCCHAR1_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x128 - OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)"]
    pub hcint1: crate::Reg<hcint1::HCINT1_SPEC>,
    #[doc = "0x12c - OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)"]
    pub hcintmsk1: crate::Reg<hcintmsk1::HCINTMSK1_SPEC>,
    #[doc = "0x130 - OTG_FS host channel-1 transfer size register"]
    pub hctsiz1: crate::Reg<hctsiz1::HCTSIZ1_SPEC>,
    _reserved15: [u8; 0x0c],
    #[doc = "0x140 - OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)"]
    pub hcchar2: crate::Reg<hcchar2::HCCHAR2_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x148 - OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)"]
    pub hcint2: crate::Reg<hcint2::HCINT2_SPEC>,
    #[doc = "0x14c - OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)"]
    pub hcintmsk2: crate::Reg<hcintmsk2::HCINTMSK2_SPEC>,
    #[doc = "0x150 - OTG_FS host channel-2 transfer size register"]
    pub hctsiz2: crate::Reg<hctsiz2::HCTSIZ2_SPEC>,
    _reserved19: [u8; 0x0c],
    #[doc = "0x160 - OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)"]
    pub hcchar3: crate::Reg<hcchar3::HCCHAR3_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x168 - OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)"]
    pub hcint3: crate::Reg<hcint3::HCINT3_SPEC>,
    #[doc = "0x16c - OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)"]
    pub hcintmsk3: crate::Reg<hcintmsk3::HCINTMSK3_SPEC>,
    #[doc = "0x170 - OTG_FS host channel-3 transfer size register"]
    pub hctsiz3: crate::Reg<hctsiz3::HCTSIZ3_SPEC>,
    _reserved23: [u8; 0x0c],
    #[doc = "0x180 - OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)"]
    pub hcchar4: crate::Reg<hcchar4::HCCHAR4_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0x188 - OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)"]
    pub hcint4: crate::Reg<hcint4::HCINT4_SPEC>,
    #[doc = "0x18c - OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)"]
    pub hcintmsk4: crate::Reg<hcintmsk4::HCINTMSK4_SPEC>,
    #[doc = "0x190 - OTG_FS host channel-x transfer size register"]
    pub hctsiz4: crate::Reg<hctsiz4::HCTSIZ4_SPEC>,
    _reserved27: [u8; 0x0c],
    #[doc = "0x1a0 - OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)"]
    pub hcchar5: crate::Reg<hcchar5::HCCHAR5_SPEC>,
    _reserved28: [u8; 0x04],
    #[doc = "0x1a8 - OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)"]
    pub hcint5: crate::Reg<hcint5::HCINT5_SPEC>,
    #[doc = "0x1ac - OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)"]
    pub hcintmsk5: crate::Reg<hcintmsk5::HCINTMSK5_SPEC>,
    #[doc = "0x1b0 - OTG_FS host channel-5 transfer size register"]
    pub hctsiz5: crate::Reg<hctsiz5::HCTSIZ5_SPEC>,
    _reserved31: [u8; 0x0c],
    #[doc = "0x1c0 - OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)"]
    pub hcchar6: crate::Reg<hcchar6::HCCHAR6_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x1c8 - OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)"]
    pub hcint6: crate::Reg<hcint6::HCINT6_SPEC>,
    #[doc = "0x1cc - OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)"]
    pub hcintmsk6: crate::Reg<hcintmsk6::HCINTMSK6_SPEC>,
    #[doc = "0x1d0 - OTG_FS host channel-6 transfer size register"]
    pub hctsiz6: crate::Reg<hctsiz6::HCTSIZ6_SPEC>,
    _reserved35: [u8; 0x0c],
    #[doc = "0x1e0 - OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)"]
    pub hcchar7: crate::Reg<hcchar7::HCCHAR7_SPEC>,
    _reserved36: [u8; 0x04],
    #[doc = "0x1e8 - OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)"]
    pub hcint7: crate::Reg<hcint7::HCINT7_SPEC>,
    #[doc = "0x1ec - OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)"]
    pub hcintmsk7: crate::Reg<hcintmsk7::HCINTMSK7_SPEC>,
    #[doc = "0x1f0 - OTG_FS host channel-7 transfer size register"]
    pub hctsiz7: crate::Reg<hctsiz7::HCTSIZ7_SPEC>,
}
#[doc = "HCFG register accessor: an alias for `Reg<HCFG_SPEC>`"]
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
#[doc = "OTG_FS host configuration register (OTG_FS_HCFG)"]
pub mod hcfg;
#[doc = "HFIR register accessor: an alias for `Reg<HFIR_SPEC>`"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = "OTG_FS Host frame interval register"]
pub mod hfir;
#[doc = "HFNUM register accessor: an alias for `Reg<HFNUM_SPEC>`"]
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
#[doc = "OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)"]
pub mod hfnum;
#[doc = "HPTXSTS register accessor: an alias for `Reg<HPTXSTS_SPEC>`"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
#[doc = "OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)"]
pub mod hptxsts;
#[doc = "HAINT register accessor: an alias for `Reg<HAINT_SPEC>`"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = "OTG_FS Host all channels interrupt register"]
pub mod haint;
#[doc = "HAINTMSK register accessor: an alias for `Reg<HAINTMSK_SPEC>`"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = "OTG_FS host all channels interrupt mask register"]
pub mod haintmsk;
#[doc = "HPRT register accessor: an alias for `Reg<HPRT_SPEC>`"]
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
#[doc = "OTG_FS host port control and status register (OTG_FS_HPRT)"]
pub mod hprt;
#[doc = "HCCHAR0 register accessor: an alias for `Reg<HCCHAR0_SPEC>`"]
pub type HCCHAR0 = crate::Reg<hcchar0::HCCHAR0_SPEC>;
#[doc = "OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)"]
pub mod hcchar0;
#[doc = "HCCHAR1 register accessor: an alias for `Reg<HCCHAR1_SPEC>`"]
pub type HCCHAR1 = crate::Reg<hcchar1::HCCHAR1_SPEC>;
#[doc = "OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)"]
pub mod hcchar1;
#[doc = "HCCHAR2 register accessor: an alias for `Reg<HCCHAR2_SPEC>`"]
pub type HCCHAR2 = crate::Reg<hcchar2::HCCHAR2_SPEC>;
#[doc = "OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)"]
pub mod hcchar2;
#[doc = "HCCHAR3 register accessor: an alias for `Reg<HCCHAR3_SPEC>`"]
pub type HCCHAR3 = crate::Reg<hcchar3::HCCHAR3_SPEC>;
#[doc = "OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)"]
pub mod hcchar3;
#[doc = "HCCHAR4 register accessor: an alias for `Reg<HCCHAR4_SPEC>`"]
pub type HCCHAR4 = crate::Reg<hcchar4::HCCHAR4_SPEC>;
#[doc = "OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)"]
pub mod hcchar4;
#[doc = "HCCHAR5 register accessor: an alias for `Reg<HCCHAR5_SPEC>`"]
pub type HCCHAR5 = crate::Reg<hcchar5::HCCHAR5_SPEC>;
#[doc = "OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)"]
pub mod hcchar5;
#[doc = "HCCHAR6 register accessor: an alias for `Reg<HCCHAR6_SPEC>`"]
pub type HCCHAR6 = crate::Reg<hcchar6::HCCHAR6_SPEC>;
#[doc = "OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)"]
pub mod hcchar6;
#[doc = "HCCHAR7 register accessor: an alias for `Reg<HCCHAR7_SPEC>`"]
pub type HCCHAR7 = crate::Reg<hcchar7::HCCHAR7_SPEC>;
#[doc = "OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)"]
pub mod hcchar7;
#[doc = "HCINT0 register accessor: an alias for `Reg<HCINT0_SPEC>`"]
pub type HCINT0 = crate::Reg<hcint0::HCINT0_SPEC>;
#[doc = "OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)"]
pub mod hcint0;
#[doc = "HCINT1 register accessor: an alias for `Reg<HCINT1_SPEC>`"]
pub type HCINT1 = crate::Reg<hcint1::HCINT1_SPEC>;
#[doc = "OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)"]
pub mod hcint1;
#[doc = "HCINT2 register accessor: an alias for `Reg<HCINT2_SPEC>`"]
pub type HCINT2 = crate::Reg<hcint2::HCINT2_SPEC>;
#[doc = "OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)"]
pub mod hcint2;
#[doc = "HCINT3 register accessor: an alias for `Reg<HCINT3_SPEC>`"]
pub type HCINT3 = crate::Reg<hcint3::HCINT3_SPEC>;
#[doc = "OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)"]
pub mod hcint3;
#[doc = "HCINT4 register accessor: an alias for `Reg<HCINT4_SPEC>`"]
pub type HCINT4 = crate::Reg<hcint4::HCINT4_SPEC>;
#[doc = "OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)"]
pub mod hcint4;
#[doc = "HCINT5 register accessor: an alias for `Reg<HCINT5_SPEC>`"]
pub type HCINT5 = crate::Reg<hcint5::HCINT5_SPEC>;
#[doc = "OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)"]
pub mod hcint5;
#[doc = "HCINT6 register accessor: an alias for `Reg<HCINT6_SPEC>`"]
pub type HCINT6 = crate::Reg<hcint6::HCINT6_SPEC>;
#[doc = "OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)"]
pub mod hcint6;
#[doc = "HCINT7 register accessor: an alias for `Reg<HCINT7_SPEC>`"]
pub type HCINT7 = crate::Reg<hcint7::HCINT7_SPEC>;
#[doc = "OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)"]
pub mod hcint7;
#[doc = "HCINTMSK0 register accessor: an alias for `Reg<HCINTMSK0_SPEC>`"]
pub type HCINTMSK0 = crate::Reg<hcintmsk0::HCINTMSK0_SPEC>;
#[doc = "OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)"]
pub mod hcintmsk0;
#[doc = "HCINTMSK1 register accessor: an alias for `Reg<HCINTMSK1_SPEC>`"]
pub type HCINTMSK1 = crate::Reg<hcintmsk1::HCINTMSK1_SPEC>;
#[doc = "OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)"]
pub mod hcintmsk1;
#[doc = "HCINTMSK2 register accessor: an alias for `Reg<HCINTMSK2_SPEC>`"]
pub type HCINTMSK2 = crate::Reg<hcintmsk2::HCINTMSK2_SPEC>;
#[doc = "OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)"]
pub mod hcintmsk2;
#[doc = "HCINTMSK3 register accessor: an alias for `Reg<HCINTMSK3_SPEC>`"]
pub type HCINTMSK3 = crate::Reg<hcintmsk3::HCINTMSK3_SPEC>;
#[doc = "OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)"]
pub mod hcintmsk3;
#[doc = "HCINTMSK4 register accessor: an alias for `Reg<HCINTMSK4_SPEC>`"]
pub type HCINTMSK4 = crate::Reg<hcintmsk4::HCINTMSK4_SPEC>;
#[doc = "OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)"]
pub mod hcintmsk4;
#[doc = "HCINTMSK5 register accessor: an alias for `Reg<HCINTMSK5_SPEC>`"]
pub type HCINTMSK5 = crate::Reg<hcintmsk5::HCINTMSK5_SPEC>;
#[doc = "OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)"]
pub mod hcintmsk5;
#[doc = "HCINTMSK6 register accessor: an alias for `Reg<HCINTMSK6_SPEC>`"]
pub type HCINTMSK6 = crate::Reg<hcintmsk6::HCINTMSK6_SPEC>;
#[doc = "OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)"]
pub mod hcintmsk6;
#[doc = "HCINTMSK7 register accessor: an alias for `Reg<HCINTMSK7_SPEC>`"]
pub type HCINTMSK7 = crate::Reg<hcintmsk7::HCINTMSK7_SPEC>;
#[doc = "OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)"]
pub mod hcintmsk7;
#[doc = "HCTSIZ0 register accessor: an alias for `Reg<HCTSIZ0_SPEC>`"]
pub type HCTSIZ0 = crate::Reg<hctsiz0::HCTSIZ0_SPEC>;
#[doc = "OTG_FS host channel-0 transfer size register"]
pub mod hctsiz0;
#[doc = "HCTSIZ1 register accessor: an alias for `Reg<HCTSIZ1_SPEC>`"]
pub type HCTSIZ1 = crate::Reg<hctsiz1::HCTSIZ1_SPEC>;
#[doc = "OTG_FS host channel-1 transfer size register"]
pub mod hctsiz1;
#[doc = "HCTSIZ2 register accessor: an alias for `Reg<HCTSIZ2_SPEC>`"]
pub type HCTSIZ2 = crate::Reg<hctsiz2::HCTSIZ2_SPEC>;
#[doc = "OTG_FS host channel-2 transfer size register"]
pub mod hctsiz2;
#[doc = "HCTSIZ3 register accessor: an alias for `Reg<HCTSIZ3_SPEC>`"]
pub type HCTSIZ3 = crate::Reg<hctsiz3::HCTSIZ3_SPEC>;
#[doc = "OTG_FS host channel-3 transfer size register"]
pub mod hctsiz3;
#[doc = "HCTSIZ4 register accessor: an alias for `Reg<HCTSIZ4_SPEC>`"]
pub type HCTSIZ4 = crate::Reg<hctsiz4::HCTSIZ4_SPEC>;
#[doc = "OTG_FS host channel-x transfer size register"]
pub mod hctsiz4;
#[doc = "HCTSIZ5 register accessor: an alias for `Reg<HCTSIZ5_SPEC>`"]
pub type HCTSIZ5 = crate::Reg<hctsiz5::HCTSIZ5_SPEC>;
#[doc = "OTG_FS host channel-5 transfer size register"]
pub mod hctsiz5;
#[doc = "HCTSIZ6 register accessor: an alias for `Reg<HCTSIZ6_SPEC>`"]
pub type HCTSIZ6 = crate::Reg<hctsiz6::HCTSIZ6_SPEC>;
#[doc = "OTG_FS host channel-6 transfer size register"]
pub mod hctsiz6;
#[doc = "HCTSIZ7 register accessor: an alias for `Reg<HCTSIZ7_SPEC>`"]
pub type HCTSIZ7 = crate::Reg<hctsiz7::HCTSIZ7_SPEC>;
#[doc = "OTG_FS host channel-7 transfer size register"]
pub mod hctsiz7;
