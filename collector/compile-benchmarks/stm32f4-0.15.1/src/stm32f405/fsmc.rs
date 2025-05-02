#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    pub bcr1: crate::Reg<bcr1::BCR1_SPEC>,
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr1: crate::Reg<btr::BTR_SPEC>,
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr2: crate::Reg<bcr::BCR_SPEC>,
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr2: crate::Reg<btr::BTR_SPEC>,
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr3: crate::Reg<bcr::BCR_SPEC>,
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr3: crate::Reg<btr::BTR_SPEC>,
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr4: crate::Reg<bcr::BCR_SPEC>,
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr4: crate::Reg<btr::BTR_SPEC>,
    _reserved8: [u8; 0x40],
    #[doc = "0x60 - PC Card/NAND Flash control register 2"]
    pub pcr2: crate::Reg<pcr::PCR_SPEC>,
    #[doc = "0x64 - FIFO status and interrupt register 2"]
    pub sr2: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x68 - Common memory space timing register 2"]
    pub pmem2: crate::Reg<pmem2::PMEM2_SPEC>,
    #[doc = "0x6c - Attribute memory space timing register 2"]
    pub patt2: crate::Reg<patt2::PATT2_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x74 - ECC result register 2"]
    pub eccr2: crate::Reg<eccr2::ECCR2_SPEC>,
    _reserved13: [u8; 0x08],
    #[doc = "0x80 - PC Card/NAND Flash control register 2"]
    pub pcr3: crate::Reg<pcr::PCR_SPEC>,
    #[doc = "0x84 - FIFO status and interrupt register 2"]
    pub sr3: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x88 - Common memory space timing register 3"]
    pub pmem3: crate::Reg<pmem3::PMEM3_SPEC>,
    #[doc = "0x8c - Attribute memory space timing register 3"]
    pub patt3: crate::Reg<patt3::PATT3_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x94 - ECC result register 3"]
    pub eccr3: crate::Reg<eccr3::ECCR3_SPEC>,
    _reserved18: [u8; 0x08],
    #[doc = "0xa0 - PC Card/NAND Flash control register 2"]
    pub pcr4: crate::Reg<pcr::PCR_SPEC>,
    #[doc = "0xa4 - FIFO status and interrupt register 2"]
    pub sr4: crate::Reg<sr::SR_SPEC>,
    #[doc = "0xa8 - Common memory space timing register 4"]
    pub pmem4: crate::Reg<pmem4::PMEM4_SPEC>,
    #[doc = "0xac - Attribute memory space timing register 4"]
    pub patt4: crate::Reg<patt4::PATT4_SPEC>,
    #[doc = "0xb0 - I/O space timing register 4"]
    pub pio4: crate::Reg<pio4::PIO4_SPEC>,
    _reserved23: [u8; 0x50],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr1: crate::Reg<bwtr::BWTR_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr2: crate::Reg<bwtr::BWTR_SPEC>,
    _reserved25: [u8; 0x04],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr3: crate::Reg<bwtr::BWTR_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr4: crate::Reg<bwtr::BWTR_SPEC>,
}
#[doc = "BCR1 register accessor: an alias for `Reg<BCR1_SPEC>`"]
pub type BCR1 = crate::Reg<bcr1::BCR1_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bcr1;
#[doc = "BTR register accessor: an alias for `Reg<BTR_SPEC>`"]
pub type BTR = crate::Reg<btr::BTR_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod btr;
#[doc = "BCR register accessor: an alias for `Reg<BCR_SPEC>`"]
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub mod bcr;
#[doc = "PCR register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "PC Card/NAND Flash control register 2"]
pub mod pcr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "FIFO status and interrupt register 2"]
pub mod sr;
#[doc = "PMEM2 register accessor: an alias for `Reg<PMEM2_SPEC>`"]
pub type PMEM2 = crate::Reg<pmem2::PMEM2_SPEC>;
#[doc = "Common memory space timing register 2"]
pub mod pmem2;
#[doc = "PATT2 register accessor: an alias for `Reg<PATT2_SPEC>`"]
pub type PATT2 = crate::Reg<patt2::PATT2_SPEC>;
#[doc = "Attribute memory space timing register 2"]
pub mod patt2;
#[doc = "ECCR2 register accessor: an alias for `Reg<ECCR2_SPEC>`"]
pub type ECCR2 = crate::Reg<eccr2::ECCR2_SPEC>;
#[doc = "ECC result register 2"]
pub mod eccr2;
#[doc = "PMEM3 register accessor: an alias for `Reg<PMEM3_SPEC>`"]
pub type PMEM3 = crate::Reg<pmem3::PMEM3_SPEC>;
#[doc = "Common memory space timing register 3"]
pub mod pmem3;
#[doc = "PATT3 register accessor: an alias for `Reg<PATT3_SPEC>`"]
pub type PATT3 = crate::Reg<patt3::PATT3_SPEC>;
#[doc = "Attribute memory space timing register 3"]
pub mod patt3;
#[doc = "ECCR3 register accessor: an alias for `Reg<ECCR3_SPEC>`"]
pub type ECCR3 = crate::Reg<eccr3::ECCR3_SPEC>;
#[doc = "ECC result register 3"]
pub mod eccr3;
#[doc = "PMEM4 register accessor: an alias for `Reg<PMEM4_SPEC>`"]
pub type PMEM4 = crate::Reg<pmem4::PMEM4_SPEC>;
#[doc = "Common memory space timing register 4"]
pub mod pmem4;
#[doc = "PATT4 register accessor: an alias for `Reg<PATT4_SPEC>`"]
pub type PATT4 = crate::Reg<patt4::PATT4_SPEC>;
#[doc = "Attribute memory space timing register 4"]
pub mod patt4;
#[doc = "PIO4 register accessor: an alias for `Reg<PIO4_SPEC>`"]
pub type PIO4 = crate::Reg<pio4::PIO4_SPEC>;
#[doc = "I/O space timing register 4"]
pub mod pio4;
#[doc = "BWTR register accessor: an alias for `Reg<BWTR_SPEC>`"]
pub type BWTR = crate::Reg<bwtr::BWTR_SPEC>;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bwtr;
