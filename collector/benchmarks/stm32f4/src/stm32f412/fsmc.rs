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
    _reserved8: [u8; 228usize],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr1: BWTR,
    _reserved9: [u8; 4usize],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr2: BWTR,
    _reserved10: [u8; 4usize],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr3: BWTR,
    _reserved11: [u8; 4usize],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr4: BWTR,
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
