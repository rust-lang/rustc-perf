#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - master control register"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x04 - master status register"]
    pub msr: crate::Reg<msr::MSR_SPEC>,
    #[doc = "0x08 - transmit status register"]
    pub tsr: crate::Reg<tsr::TSR_SPEC>,
    #[doc = "0x0c..0x14 - receive FIFO %s register"]
    pub rfr: [crate::Reg<rfr::RFR_SPEC>; 2],
    #[doc = "0x14 - interrupt enable register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x18 - interrupt enable register"]
    pub esr: crate::Reg<esr::ESR_SPEC>,
    #[doc = "0x1c - bit timing register"]
    pub btr: crate::Reg<btr::BTR_SPEC>,
    _reserved7: [u8; 0x0160],
    #[doc = "0x180..0x1b0 - CAN Transmit cluster"]
    pub tx: [TX; 3],
    #[doc = "0x1b0..0x1d0 - CAN Receive cluster"]
    pub rx: [RX; 2],
    _reserved9: [u8; 0x30],
    #[doc = "0x200 - filter master register"]
    pub fmr: crate::Reg<fmr::FMR_SPEC>,
    #[doc = "0x204 - filter mode register"]
    pub fm1r: crate::Reg<fm1r::FM1R_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x20c - filter scale register"]
    pub fs1r: crate::Reg<fs1r::FS1R_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x214 - filter FIFO assignment register"]
    pub ffa1r: crate::Reg<ffa1r::FFA1R_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x21c - filter activation register"]
    pub fa1r: crate::Reg<fa1r::FA1R_SPEC>,
    _reserved14: [u8; 0x20],
    #[doc = "0x240..0x320 - CAN Filter Bank cluster"]
    pub fb: [FB; 28],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct TX {
    #[doc = "0x00 - TX mailbox identifier register"]
    pub tir: crate::Reg<self::tx::tir::TIR_SPEC>,
    #[doc = "0x04 - mailbox data length control and time stamp register"]
    pub tdtr: crate::Reg<self::tx::tdtr::TDTR_SPEC>,
    #[doc = "0x08 - mailbox data low register"]
    pub tdlr: crate::Reg<self::tx::tdlr::TDLR_SPEC>,
    #[doc = "0x0c - mailbox data high register"]
    pub tdhr: crate::Reg<self::tx::tdhr::TDHR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CAN Transmit cluster"]
pub mod tx;
#[doc = r"Register block"]
#[repr(C)]
pub struct RX {
    #[doc = "0x00 - receive FIFO mailbox identifier register"]
    pub rir: crate::Reg<self::rx::rir::RIR_SPEC>,
    #[doc = "0x04 - mailbox data high register"]
    pub rdtr: crate::Reg<self::rx::rdtr::RDTR_SPEC>,
    #[doc = "0x08 - mailbox data high register"]
    pub rdlr: crate::Reg<self::rx::rdlr::RDLR_SPEC>,
    #[doc = "0x0c - receive FIFO mailbox data high register"]
    pub rdhr: crate::Reg<self::rx::rdhr::RDHR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CAN Receive cluster"]
pub mod rx;
#[doc = r"Register block"]
#[repr(C)]
pub struct FB {
    #[doc = "0x00 - Filter bank 0 register 1"]
    pub fr1: crate::Reg<self::fb::fr1::FR1_SPEC>,
    #[doc = "0x04 - Filter bank 0 register 2"]
    pub fr2: crate::Reg<self::fb::fr2::FR2_SPEC>,
}
#[doc = r"Register block"]
#[doc = "CAN Filter Bank cluster"]
pub mod fb;
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "master control register"]
pub mod mcr;
#[doc = "MSR register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "master status register"]
pub mod msr;
#[doc = "TSR register accessor: an alias for `Reg<TSR_SPEC>`"]
pub type TSR = crate::Reg<tsr::TSR_SPEC>;
#[doc = "transmit status register"]
pub mod tsr;
#[doc = "RFR register accessor: an alias for `Reg<RFR_SPEC>`"]
pub type RFR = crate::Reg<rfr::RFR_SPEC>;
#[doc = "receive FIFO %s register"]
pub mod rfr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "ESR register accessor: an alias for `Reg<ESR_SPEC>`"]
pub type ESR = crate::Reg<esr::ESR_SPEC>;
#[doc = "interrupt enable register"]
pub mod esr;
#[doc = "BTR register accessor: an alias for `Reg<BTR_SPEC>`"]
pub type BTR = crate::Reg<btr::BTR_SPEC>;
#[doc = "bit timing register"]
pub mod btr;
#[doc = "FMR register accessor: an alias for `Reg<FMR_SPEC>`"]
pub type FMR = crate::Reg<fmr::FMR_SPEC>;
#[doc = "filter master register"]
pub mod fmr;
#[doc = "FM1R register accessor: an alias for `Reg<FM1R_SPEC>`"]
pub type FM1R = crate::Reg<fm1r::FM1R_SPEC>;
#[doc = "filter mode register"]
pub mod fm1r;
#[doc = "FS1R register accessor: an alias for `Reg<FS1R_SPEC>`"]
pub type FS1R = crate::Reg<fs1r::FS1R_SPEC>;
#[doc = "filter scale register"]
pub mod fs1r;
#[doc = "FFA1R register accessor: an alias for `Reg<FFA1R_SPEC>`"]
pub type FFA1R = crate::Reg<ffa1r::FFA1R_SPEC>;
#[doc = "filter FIFO assignment register"]
pub mod ffa1r;
#[doc = "FA1R register accessor: an alias for `Reg<FA1R_SPEC>`"]
pub type FA1R = crate::Reg<fa1r::FA1R_SPEC>;
#[doc = "filter activation register"]
pub mod fa1r;
