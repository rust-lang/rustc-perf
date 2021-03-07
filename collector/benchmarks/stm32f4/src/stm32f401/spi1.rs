#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - status register"]
    pub sr: SR,
    #[doc = "0x0c - data register"]
    pub dr: DR,
    #[doc = "0x10 - CRC polynomial register"]
    pub crcpr: CRCPR,
    #[doc = "0x14 - RX CRC register"]
    pub rxcrcr: RXCRCR,
    #[doc = "0x18 - TX CRC register"]
    pub txcrcr: TXCRCR,
    #[doc = "0x1c - I2S configuration register"]
    pub i2scfgr: I2SCFGR,
    #[doc = "0x20 - I2S prescaler register"]
    pub i2spr: I2SPR,
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure"]
impl crate::Writable for CR1 {}
#[doc = "control register 1"]
pub mod cr1;
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](cr2) module"]
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
#[doc = "`read()` method returns [cr2::R](cr2::R) reader structure"]
impl crate::Readable for CR2 {}
#[doc = "`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure"]
impl crate::Writable for CR2 {}
#[doc = "control register 2"]
pub mod cr2;
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
#[doc = "data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "data register"]
pub mod dr;
#[doc = "CRC polynomial register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcpr](crcpr) module"]
pub type CRCPR = crate::Reg<u32, _CRCPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCPR;
#[doc = "`read()` method returns [crcpr::R](crcpr::R) reader structure"]
impl crate::Readable for CRCPR {}
#[doc = "`write(|w| ..)` method takes [crcpr::W](crcpr::W) writer structure"]
impl crate::Writable for CRCPR {}
#[doc = "CRC polynomial register"]
pub mod crcpr;
#[doc = "RX CRC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcrcr](rxcrcr) module"]
pub type RXCRCR = crate::Reg<u32, _RXCRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCRCR;
#[doc = "`read()` method returns [rxcrcr::R](rxcrcr::R) reader structure"]
impl crate::Readable for RXCRCR {}
#[doc = "RX CRC register"]
pub mod rxcrcr;
#[doc = "TX CRC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcrcr](txcrcr) module"]
pub type TXCRCR = crate::Reg<u32, _TXCRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCRCR;
#[doc = "`read()` method returns [txcrcr::R](txcrcr::R) reader structure"]
impl crate::Readable for TXCRCR {}
#[doc = "TX CRC register"]
pub mod txcrcr;
#[doc = "I2S configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2scfgr](i2scfgr) module"]
pub type I2SCFGR = crate::Reg<u32, _I2SCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SCFGR;
#[doc = "`read()` method returns [i2scfgr::R](i2scfgr::R) reader structure"]
impl crate::Readable for I2SCFGR {}
#[doc = "`write(|w| ..)` method takes [i2scfgr::W](i2scfgr::W) writer structure"]
impl crate::Writable for I2SCFGR {}
#[doc = "I2S configuration register"]
pub mod i2scfgr;
#[doc = "I2S prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2spr](i2spr) module"]
pub type I2SPR = crate::Reg<u32, _I2SPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SPR;
#[doc = "`read()` method returns [i2spr::R](i2spr::R) reader structure"]
impl crate::Readable for I2SPR {}
#[doc = "`write(|w| ..)` method takes [i2spr::W](i2spr::W) writer structure"]
impl crate::Writable for I2SPR {}
#[doc = "I2S prescaler register"]
pub mod i2spr;
