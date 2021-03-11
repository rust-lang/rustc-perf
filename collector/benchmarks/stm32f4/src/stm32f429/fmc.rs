#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    pub bcr1: BCR1,
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr1: BTR,
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr2: BCR,
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr2: BTR,
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr3: BCR,
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr3: BTR,
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr4: BCR,
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr4: BTR,
    _reserved8: [u8; 64usize],
    #[doc = "0x60 - PC Card/NAND Flash control register 2"]
    pub pcr2: PCR,
    #[doc = "0x64 - FIFO status and interrupt register 2"]
    pub sr2: SR,
    #[doc = "0x68 - Common memory space timing register 2"]
    pub pmem2: PMEM2,
    #[doc = "0x6c - Attribute memory space timing register 2"]
    pub patt2: PATT2,
    _reserved12: [u8; 4usize],
    #[doc = "0x74 - ECC result register 2"]
    pub eccr2: ECCR2,
    _reserved13: [u8; 8usize],
    #[doc = "0x80 - PC Card/NAND Flash control register 2"]
    pub pcr3: PCR,
    #[doc = "0x84 - FIFO status and interrupt register 2"]
    pub sr3: SR,
    #[doc = "0x88 - Common memory space timing register 3"]
    pub pmem3: PMEM3,
    #[doc = "0x8c - Attribute memory space timing register 3"]
    pub patt3: PATT3,
    _reserved17: [u8; 4usize],
    #[doc = "0x94 - ECC result register 3"]
    pub eccr3: ECCR3,
    _reserved18: [u8; 8usize],
    #[doc = "0xa0 - PC Card/NAND Flash control register 2"]
    pub pcr4: PCR,
    #[doc = "0xa4 - FIFO status and interrupt register 2"]
    pub sr4: SR,
    #[doc = "0xa8 - Common memory space timing register 4"]
    pub pmem4: PMEM4,
    #[doc = "0xac - Attribute memory space timing register 4"]
    pub patt4: PATT4,
    #[doc = "0xb0 - I/O space timing register 4"]
    pub pio4: PIO4,
    _reserved23: [u8; 80usize],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr1: BWTR,
    _reserved24: [u8; 4usize],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr2: BWTR,
    _reserved25: [u8; 4usize],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr3: BWTR,
    _reserved26: [u8; 4usize],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr4: BWTR,
    _reserved27: [u8; 32usize],
    #[doc = "0x140 - SDRAM Control Register 1"]
    pub sdcr1: SDCR,
    #[doc = "0x144 - SDRAM Control Register 1"]
    pub sdcr2: SDCR,
    #[doc = "0x148 - SDRAM Timing register 1"]
    pub sdtr1: SDTR,
    #[doc = "0x14c - SDRAM Timing register 1"]
    pub sdtr2: SDTR,
    #[doc = "0x150 - SDRAM Command Mode register"]
    pub sdcmr: SDCMR,
    #[doc = "0x154 - SDRAM Refresh Timer register"]
    pub sdrtr: SDRTR,
    #[doc = "0x158 - SDRAM Status register"]
    pub sdsr: SDSR,
}
#[doc = "SRAM/NOR-Flash chip-select control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr1](bcr1) module"]
pub type BCR1 = crate::Reg<u32, _BCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR1;
#[doc = "`read()` method returns [bcr1::R](bcr1::R) reader structure"]
impl crate::Readable for BCR1 {}
#[doc = "`write(|w| ..)` method takes [bcr1::W](bcr1::W) writer structure"]
impl crate::Writable for BCR1 {}
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bcr1;
#[doc = "SRAM/NOR-Flash chip-select timing register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btr](btr) module"]
pub type BTR = crate::Reg<u32, _BTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTR;
#[doc = "`read()` method returns [btr::R](btr::R) reader structure"]
impl crate::Readable for BTR {}
#[doc = "`write(|w| ..)` method takes [btr::W](btr::W) writer structure"]
impl crate::Writable for BTR {}
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod btr;
#[doc = "SRAM/NOR-Flash chip-select control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr](bcr) module"]
pub type BCR = crate::Reg<u32, _BCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR;
#[doc = "`read()` method returns [bcr::R](bcr::R) reader structure"]
impl crate::Readable for BCR {}
#[doc = "`write(|w| ..)` method takes [bcr::W](bcr::W) writer structure"]
impl crate::Writable for BCR {}
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub mod bcr;
#[doc = "PC Card/NAND Flash control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr](pcr) module"]
pub type PCR = crate::Reg<u32, _PCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR;
#[doc = "`read()` method returns [pcr::R](pcr::R) reader structure"]
impl crate::Readable for PCR {}
#[doc = "`write(|w| ..)` method takes [pcr::W](pcr::W) writer structure"]
impl crate::Writable for PCR {}
#[doc = "PC Card/NAND Flash control register 2"]
pub mod pcr;
#[doc = "FIFO status and interrupt register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "FIFO status and interrupt register 2"]
pub mod sr;
#[doc = "Common memory space timing register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmem2](pmem2) module"]
pub type PMEM2 = crate::Reg<u32, _PMEM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMEM2;
#[doc = "`read()` method returns [pmem2::R](pmem2::R) reader structure"]
impl crate::Readable for PMEM2 {}
#[doc = "`write(|w| ..)` method takes [pmem2::W](pmem2::W) writer structure"]
impl crate::Writable for PMEM2 {}
#[doc = "Common memory space timing register 2"]
pub mod pmem2;
#[doc = "Attribute memory space timing register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patt2](patt2) module"]
pub type PATT2 = crate::Reg<u32, _PATT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PATT2;
#[doc = "`read()` method returns [patt2::R](patt2::R) reader structure"]
impl crate::Readable for PATT2 {}
#[doc = "`write(|w| ..)` method takes [patt2::W](patt2::W) writer structure"]
impl crate::Writable for PATT2 {}
#[doc = "Attribute memory space timing register 2"]
pub mod patt2;
#[doc = "ECC result register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccr2](eccr2) module"]
pub type ECCR2 = crate::Reg<u32, _ECCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECCR2;
#[doc = "`read()` method returns [eccr2::R](eccr2::R) reader structure"]
impl crate::Readable for ECCR2 {}
#[doc = "ECC result register 2"]
pub mod eccr2;
#[doc = "Common memory space timing register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmem3](pmem3) module"]
pub type PMEM3 = crate::Reg<u32, _PMEM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMEM3;
#[doc = "`read()` method returns [pmem3::R](pmem3::R) reader structure"]
impl crate::Readable for PMEM3 {}
#[doc = "`write(|w| ..)` method takes [pmem3::W](pmem3::W) writer structure"]
impl crate::Writable for PMEM3 {}
#[doc = "Common memory space timing register 3"]
pub mod pmem3;
#[doc = "Attribute memory space timing register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patt3](patt3) module"]
pub type PATT3 = crate::Reg<u32, _PATT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PATT3;
#[doc = "`read()` method returns [patt3::R](patt3::R) reader structure"]
impl crate::Readable for PATT3 {}
#[doc = "`write(|w| ..)` method takes [patt3::W](patt3::W) writer structure"]
impl crate::Writable for PATT3 {}
#[doc = "Attribute memory space timing register 3"]
pub mod patt3;
#[doc = "ECC result register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccr3](eccr3) module"]
pub type ECCR3 = crate::Reg<u32, _ECCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECCR3;
#[doc = "`read()` method returns [eccr3::R](eccr3::R) reader structure"]
impl crate::Readable for ECCR3 {}
#[doc = "ECC result register 3"]
pub mod eccr3;
#[doc = "Common memory space timing register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmem4](pmem4) module"]
pub type PMEM4 = crate::Reg<u32, _PMEM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMEM4;
#[doc = "`read()` method returns [pmem4::R](pmem4::R) reader structure"]
impl crate::Readable for PMEM4 {}
#[doc = "`write(|w| ..)` method takes [pmem4::W](pmem4::W) writer structure"]
impl crate::Writable for PMEM4 {}
#[doc = "Common memory space timing register 4"]
pub mod pmem4;
#[doc = "Attribute memory space timing register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patt4](patt4) module"]
pub type PATT4 = crate::Reg<u32, _PATT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PATT4;
#[doc = "`read()` method returns [patt4::R](patt4::R) reader structure"]
impl crate::Readable for PATT4 {}
#[doc = "`write(|w| ..)` method takes [patt4::W](patt4::W) writer structure"]
impl crate::Writable for PATT4 {}
#[doc = "Attribute memory space timing register 4"]
pub mod patt4;
#[doc = "I/O space timing register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio4](pio4) module"]
pub type PIO4 = crate::Reg<u32, _PIO4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO4;
#[doc = "`read()` method returns [pio4::R](pio4::R) reader structure"]
impl crate::Readable for PIO4 {}
#[doc = "`write(|w| ..)` method takes [pio4::W](pio4::W) writer structure"]
impl crate::Writable for PIO4 {}
#[doc = "I/O space timing register 4"]
pub mod pio4;
#[doc = "SRAM/NOR-Flash write timing registers 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bwtr](bwtr) module"]
pub type BWTR = crate::Reg<u32, _BWTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BWTR;
#[doc = "`read()` method returns [bwtr::R](bwtr::R) reader structure"]
impl crate::Readable for BWTR {}
#[doc = "`write(|w| ..)` method takes [bwtr::W](bwtr::W) writer structure"]
impl crate::Writable for BWTR {}
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bwtr;
#[doc = "SDRAM Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdcr](sdcr) module"]
pub type SDCR = crate::Reg<u32, _SDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDCR;
#[doc = "`read()` method returns [sdcr::R](sdcr::R) reader structure"]
impl crate::Readable for SDCR {}
#[doc = "`write(|w| ..)` method takes [sdcr::W](sdcr::W) writer structure"]
impl crate::Writable for SDCR {}
#[doc = "SDRAM Control Register 1"]
pub mod sdcr;
#[doc = "SDRAM Timing register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdtr](sdtr) module"]
pub type SDTR = crate::Reg<u32, _SDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDTR;
#[doc = "`read()` method returns [sdtr::R](sdtr::R) reader structure"]
impl crate::Readable for SDTR {}
#[doc = "`write(|w| ..)` method takes [sdtr::W](sdtr::W) writer structure"]
impl crate::Writable for SDTR {}
#[doc = "SDRAM Timing register 1"]
pub mod sdtr;
#[doc = "SDRAM Command Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdcmr](sdcmr) module"]
pub type SDCMR = crate::Reg<u32, _SDCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDCMR;
#[doc = "`read()` method returns [sdcmr::R](sdcmr::R) reader structure"]
impl crate::Readable for SDCMR {}
#[doc = "`write(|w| ..)` method takes [sdcmr::W](sdcmr::W) writer structure"]
impl crate::Writable for SDCMR {}
#[doc = "SDRAM Command Mode register"]
pub mod sdcmr;
#[doc = "SDRAM Refresh Timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdrtr](sdrtr) module"]
pub type SDRTR = crate::Reg<u32, _SDRTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRTR;
#[doc = "`read()` method returns [sdrtr::R](sdrtr::R) reader structure"]
impl crate::Readable for SDRTR {}
#[doc = "`write(|w| ..)` method takes [sdrtr::W](sdrtr::W) writer structure"]
impl crate::Writable for SDRTR {}
#[doc = "SDRAM Refresh Timer register"]
pub mod sdrtr;
#[doc = "SDRAM Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdsr](sdsr) module"]
pub type SDSR = crate::Reg<u32, _SDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDSR;
#[doc = "`read()` method returns [sdsr::R](sdsr::R) reader structure"]
impl crate::Readable for SDSR {}
#[doc = "SDRAM Status register"]
pub mod sdsr;
