#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - software trigger register"]
    pub swtrigr: SWTRIGR,
    #[doc = "0x08 - channel1 12-bit right-aligned data holding register"]
    pub dhr12r1: DHR12R1,
    #[doc = "0x0c - channel1 12-bit left aligned data holding register"]
    pub dhr12l1: DHR12L1,
    #[doc = "0x10 - channel1 8-bit right aligned data holding register"]
    pub dhr8r1: DHR8R1,
    #[doc = "0x14 - channel2 12-bit right aligned data holding register"]
    pub dhr12r2: DHR12R2,
    #[doc = "0x18 - channel2 12-bit left aligned data holding register"]
    pub dhr12l2: DHR12L2,
    #[doc = "0x1c - channel2 8-bit right-aligned data holding register"]
    pub dhr8r2: DHR8R2,
    #[doc = "0x20 - Dual DAC 12-bit right-aligned data holding register"]
    pub dhr12rd: DHR12RD,
    #[doc = "0x24 - DUAL DAC 12-bit left aligned data holding register"]
    pub dhr12ld: DHR12LD,
    #[doc = "0x28 - DUAL DAC 8-bit right aligned data holding register"]
    pub dhr8rd: DHR8RD,
    #[doc = "0x2c - channel1 data output register"]
    pub dor1: DOR1,
    #[doc = "0x30 - channel2 data output register"]
    pub dor2: DOR2,
    #[doc = "0x34 - status register"]
    pub sr: SR,
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "control register"]
pub mod cr;
#[doc = "software trigger register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swtrigr](swtrigr) module"]
pub type SWTRIGR = crate::Reg<u32, _SWTRIGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWTRIGR;
#[doc = "`write(|w| ..)` method takes [swtrigr::W](swtrigr::W) writer structure"]
impl crate::Writable for SWTRIGR {}
#[doc = "software trigger register"]
pub mod swtrigr;
#[doc = "channel1 12-bit right-aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhr12r1](dhr12r1) module"]
pub type DHR12R1 = crate::Reg<u32, _DHR12R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHR12R1;
#[doc = "`read()` method returns [dhr12r1::R](dhr12r1::R) reader structure"]
impl crate::Readable for DHR12R1 {}
#[doc = "`write(|w| ..)` method takes [dhr12r1::W](dhr12r1::W) writer structure"]
impl crate::Writable for DHR12R1 {}
#[doc = "channel1 12-bit right-aligned data holding register"]
pub mod dhr12r1;
#[doc = "channel1 12-bit left aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhr12l1](dhr12l1) module"]
pub type DHR12L1 = crate::Reg<u32, _DHR12L1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHR12L1;
#[doc = "`read()` method returns [dhr12l1::R](dhr12l1::R) reader structure"]
impl crate::Readable for DHR12L1 {}
#[doc = "`write(|w| ..)` method takes [dhr12l1::W](dhr12l1::W) writer structure"]
impl crate::Writable for DHR12L1 {}
#[doc = "channel1 12-bit left aligned data holding register"]
pub mod dhr12l1;
#[doc = "channel1 8-bit right aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhr8r1](dhr8r1) module"]
pub type DHR8R1 = crate::Reg<u32, _DHR8R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHR8R1;
#[doc = "`read()` method returns [dhr8r1::R](dhr8r1::R) reader structure"]
impl crate::Readable for DHR8R1 {}
#[doc = "`write(|w| ..)` method takes [dhr8r1::W](dhr8r1::W) writer structure"]
impl crate::Writable for DHR8R1 {}
#[doc = "channel1 8-bit right aligned data holding register"]
pub mod dhr8r1;
#[doc = "channel2 12-bit right aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhr12r2](dhr12r2) module"]
pub type DHR12R2 = crate::Reg<u32, _DHR12R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHR12R2;
#[doc = "`read()` method returns [dhr12r2::R](dhr12r2::R) reader structure"]
impl crate::Readable for DHR12R2 {}
#[doc = "`write(|w| ..)` method takes [dhr12r2::W](dhr12r2::W) writer structure"]
impl crate::Writable for DHR12R2 {}
#[doc = "channel2 12-bit right aligned data holding register"]
pub mod dhr12r2;
#[doc = "channel2 12-bit left aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhr12l2](dhr12l2) module"]
pub type DHR12L2 = crate::Reg<u32, _DHR12L2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHR12L2;
#[doc = "`read()` method returns [dhr12l2::R](dhr12l2::R) reader structure"]
impl crate::Readable for DHR12L2 {}
#[doc = "`write(|w| ..)` method takes [dhr12l2::W](dhr12l2::W) writer structure"]
impl crate::Writable for DHR12L2 {}
#[doc = "channel2 12-bit left aligned data holding register"]
pub mod dhr12l2;
#[doc = "channel2 8-bit right-aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhr8r2](dhr8r2) module"]
pub type DHR8R2 = crate::Reg<u32, _DHR8R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHR8R2;
#[doc = "`read()` method returns [dhr8r2::R](dhr8r2::R) reader structure"]
impl crate::Readable for DHR8R2 {}
#[doc = "`write(|w| ..)` method takes [dhr8r2::W](dhr8r2::W) writer structure"]
impl crate::Writable for DHR8R2 {}
#[doc = "channel2 8-bit right-aligned data holding register"]
pub mod dhr8r2;
#[doc = "Dual DAC 12-bit right-aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhr12rd](dhr12rd) module"]
pub type DHR12RD = crate::Reg<u32, _DHR12RD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHR12RD;
#[doc = "`read()` method returns [dhr12rd::R](dhr12rd::R) reader structure"]
impl crate::Readable for DHR12RD {}
#[doc = "`write(|w| ..)` method takes [dhr12rd::W](dhr12rd::W) writer structure"]
impl crate::Writable for DHR12RD {}
#[doc = "Dual DAC 12-bit right-aligned data holding register"]
pub mod dhr12rd;
#[doc = "DUAL DAC 12-bit left aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhr12ld](dhr12ld) module"]
pub type DHR12LD = crate::Reg<u32, _DHR12LD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHR12LD;
#[doc = "`read()` method returns [dhr12ld::R](dhr12ld::R) reader structure"]
impl crate::Readable for DHR12LD {}
#[doc = "`write(|w| ..)` method takes [dhr12ld::W](dhr12ld::W) writer structure"]
impl crate::Writable for DHR12LD {}
#[doc = "DUAL DAC 12-bit left aligned data holding register"]
pub mod dhr12ld;
#[doc = "DUAL DAC 8-bit right aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhr8rd](dhr8rd) module"]
pub type DHR8RD = crate::Reg<u32, _DHR8RD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHR8RD;
#[doc = "`read()` method returns [dhr8rd::R](dhr8rd::R) reader structure"]
impl crate::Readable for DHR8RD {}
#[doc = "`write(|w| ..)` method takes [dhr8rd::W](dhr8rd::W) writer structure"]
impl crate::Writable for DHR8RD {}
#[doc = "DUAL DAC 8-bit right aligned data holding register"]
pub mod dhr8rd;
#[doc = "channel1 data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dor1](dor1) module"]
pub type DOR1 = crate::Reg<u32, _DOR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOR1;
#[doc = "`read()` method returns [dor1::R](dor1::R) reader structure"]
impl crate::Readable for DOR1 {}
#[doc = "channel1 data output register"]
pub mod dor1;
#[doc = "channel2 data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dor2](dor2) module"]
pub type DOR2 = crate::Reg<u32, _DOR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOR2;
#[doc = "`read()` method returns [dor2::R](dor2::R) reader structure"]
impl crate::Readable for DOR2 {}
#[doc = "channel2 data output register"]
pub mod dor2;
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "status register"]
pub mod sr;
