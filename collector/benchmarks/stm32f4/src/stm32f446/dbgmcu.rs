#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IDCODE"]
    pub idcode: IDCODE,
    #[doc = "0x04 - Control Register"]
    pub cr: CR,
    #[doc = "0x08 - Debug MCU APB1 Freeze registe"]
    pub apb1_fz: APB1_FZ,
    #[doc = "0x0c - Debug MCU APB2 Freeze registe"]
    pub apb2_fz: APB2_FZ,
}
#[doc = "IDCODE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idcode](idcode) module"]
pub type IDCODE = crate::Reg<u32, _IDCODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDCODE;
#[doc = "`read()` method returns [idcode::R](idcode::R) reader structure"]
impl crate::Readable for IDCODE {}
#[doc = "IDCODE"]
pub mod idcode;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Debug MCU APB1 Freeze registe\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1_fz](apb1_fz) module"]
pub type APB1_FZ = crate::Reg<u32, _APB1_FZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1_FZ;
#[doc = "`read()` method returns [apb1_fz::R](apb1_fz::R) reader structure"]
impl crate::Readable for APB1_FZ {}
#[doc = "`write(|w| ..)` method takes [apb1_fz::W](apb1_fz::W) writer structure"]
impl crate::Writable for APB1_FZ {}
#[doc = "Debug MCU APB1 Freeze registe"]
pub mod apb1_fz;
#[doc = "Debug MCU APB2 Freeze registe\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2_fz](apb2_fz) module"]
pub type APB2_FZ = crate::Reg<u32, _APB2_FZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2_FZ;
#[doc = "`read()` method returns [apb2_fz::R](apb2_fz::R) reader structure"]
impl crate::Readable for APB2_FZ {}
#[doc = "`write(|w| ..)` method takes [apb2_fz::W](apb2_fz::W) writer structure"]
impl crate::Writable for APB2_FZ {}
#[doc = "Debug MCU APB2 Freeze registe"]
pub mod apb2_fz;
