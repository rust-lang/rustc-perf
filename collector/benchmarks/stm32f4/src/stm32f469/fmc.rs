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
    _reserved8: [u8; 96usize],
    #[doc = "0x80 - PC Card/NAND Flash control register 3"]
    pub pcr: PCR,
    #[doc = "0x84 - FIFO status and interrupt register 3"]
    pub sr: SR,
    #[doc = "0x88 - Common memory space timing register 3"]
    pub pmem: PMEM,
    #[doc = "0x8c - Attribute memory space timing register 3"]
    pub patt: PATT,
    _reserved12: [u8; 4usize],
    #[doc = "0x94 - ECC result register 3"]
    pub eccr: ECCR,
    _reserved13: [u8; 108usize],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr1: BWTR,
    _reserved14: [u8; 4usize],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr2: BWTR,
    _reserved15: [u8; 4usize],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr3: BWTR,
    _reserved16: [u8; 4usize],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr4: BWTR,
    _reserved17: [u8; 32usize],
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
#[doc = "PC Card/NAND Flash control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr](pcr) module"]
pub type PCR = crate::Reg<u32, _PCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR;
#[doc = "`read()` method returns [pcr::R](pcr::R) reader structure"]
impl crate::Readable for PCR {}
#[doc = "`write(|w| ..)` method takes [pcr::W](pcr::W) writer structure"]
impl crate::Writable for PCR {}
#[doc = "PC Card/NAND Flash control register 3"]
pub mod pcr;
#[doc = "FIFO status and interrupt register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "FIFO status and interrupt register 3"]
pub mod sr;
#[doc = "Common memory space timing register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmem](pmem) module"]
pub type PMEM = crate::Reg<u32, _PMEM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMEM;
#[doc = "`read()` method returns [pmem::R](pmem::R) reader structure"]
impl crate::Readable for PMEM {}
#[doc = "`write(|w| ..)` method takes [pmem::W](pmem::W) writer structure"]
impl crate::Writable for PMEM {}
#[doc = "Common memory space timing register 3"]
pub mod pmem;
#[doc = "Attribute memory space timing register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patt](patt) module"]
pub type PATT = crate::Reg<u32, _PATT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PATT;
#[doc = "`read()` method returns [patt::R](patt::R) reader structure"]
impl crate::Readable for PATT {}
#[doc = "`write(|w| ..)` method takes [patt::W](patt::W) writer structure"]
impl crate::Writable for PATT {}
#[doc = "Attribute memory space timing register 3"]
pub mod patt;
#[doc = "ECC result register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccr](eccr) module"]
pub type ECCR = crate::Reg<u32, _ECCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECCR;
#[doc = "`read()` method returns [eccr::R](eccr::R) reader structure"]
impl crate::Readable for ECCR {}
#[doc = "ECC result register 3"]
pub mod eccr;
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
