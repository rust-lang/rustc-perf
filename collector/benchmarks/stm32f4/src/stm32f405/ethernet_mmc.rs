#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MMC control register"]
    pub mmccr: MMCCR,
    #[doc = "0x04 - Ethernet MMC receive interrupt register"]
    pub mmcrir: MMCRIR,
    #[doc = "0x08 - Ethernet MMC transmit interrupt register"]
    pub mmctir: MMCTIR,
    #[doc = "0x0c - Ethernet MMC receive interrupt mask register"]
    pub mmcrimr: MMCRIMR,
    #[doc = "0x10 - Ethernet MMC transmit interrupt mask register"]
    pub mmctimr: MMCTIMR,
    _reserved5: [u8; 56usize],
    #[doc = "0x4c - Ethernet MMC transmitted good frames after a single collision counter"]
    pub mmctgfsccr: MMCTGFSCCR,
    #[doc = "0x50 - Ethernet MMC transmitted good frames after more than a single collision"]
    pub mmctgfmsccr: MMCTGFMSCCR,
    _reserved7: [u8; 20usize],
    #[doc = "0x68 - Ethernet MMC transmitted good frames counter register"]
    pub mmctgfcr: MMCTGFCR,
    _reserved8: [u8; 40usize],
    #[doc = "0x94 - Ethernet MMC received frames with CRC error counter register"]
    pub mmcrfcecr: MMCRFCECR,
    #[doc = "0x98 - Ethernet MMC received frames with alignment error counter register"]
    pub mmcrfaecr: MMCRFAECR,
    _reserved10: [u8; 40usize],
    #[doc = "0xc4 - MMC received good unicast frames counter register"]
    pub mmcrgufcr: MMCRGUFCR,
}
#[doc = "Ethernet MMC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmccr](mmccr) module"]
pub type MMCCR = crate::Reg<u32, _MMCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMCCR;
#[doc = "`read()` method returns [mmccr::R](mmccr::R) reader structure"]
impl crate::Readable for MMCCR {}
#[doc = "`write(|w| ..)` method takes [mmccr::W](mmccr::W) writer structure"]
impl crate::Writable for MMCCR {}
#[doc = "Ethernet MMC control register"]
pub mod mmccr;
#[doc = "Ethernet MMC receive interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmcrir](mmcrir) module"]
pub type MMCRIR = crate::Reg<u32, _MMCRIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMCRIR;
#[doc = "`read()` method returns [mmcrir::R](mmcrir::R) reader structure"]
impl crate::Readable for MMCRIR {}
#[doc = "`write(|w| ..)` method takes [mmcrir::W](mmcrir::W) writer structure"]
impl crate::Writable for MMCRIR {}
#[doc = "Ethernet MMC receive interrupt register"]
pub mod mmcrir;
#[doc = "Ethernet MMC transmit interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmctir](mmctir) module"]
pub type MMCTIR = crate::Reg<u32, _MMCTIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMCTIR;
#[doc = "`read()` method returns [mmctir::R](mmctir::R) reader structure"]
impl crate::Readable for MMCTIR {}
#[doc = "Ethernet MMC transmit interrupt register"]
pub mod mmctir;
#[doc = "Ethernet MMC receive interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmcrimr](mmcrimr) module"]
pub type MMCRIMR = crate::Reg<u32, _MMCRIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMCRIMR;
#[doc = "`read()` method returns [mmcrimr::R](mmcrimr::R) reader structure"]
impl crate::Readable for MMCRIMR {}
#[doc = "`write(|w| ..)` method takes [mmcrimr::W](mmcrimr::W) writer structure"]
impl crate::Writable for MMCRIMR {}
#[doc = "Ethernet MMC receive interrupt mask register"]
pub mod mmcrimr;
#[doc = "Ethernet MMC transmit interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmctimr](mmctimr) module"]
pub type MMCTIMR = crate::Reg<u32, _MMCTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMCTIMR;
#[doc = "`read()` method returns [mmctimr::R](mmctimr::R) reader structure"]
impl crate::Readable for MMCTIMR {}
#[doc = "`write(|w| ..)` method takes [mmctimr::W](mmctimr::W) writer structure"]
impl crate::Writable for MMCTIMR {}
#[doc = "Ethernet MMC transmit interrupt mask register"]
pub mod mmctimr;
#[doc = "Ethernet MMC transmitted good frames after a single collision counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmctgfsccr](mmctgfsccr) module"]
pub type MMCTGFSCCR = crate::Reg<u32, _MMCTGFSCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMCTGFSCCR;
#[doc = "`read()` method returns [mmctgfsccr::R](mmctgfsccr::R) reader structure"]
impl crate::Readable for MMCTGFSCCR {}
#[doc = "Ethernet MMC transmitted good frames after a single collision counter"]
pub mod mmctgfsccr;
#[doc = "Ethernet MMC transmitted good frames after more than a single collision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmctgfmsccr](mmctgfmsccr) module"]
pub type MMCTGFMSCCR = crate::Reg<u32, _MMCTGFMSCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMCTGFMSCCR;
#[doc = "`read()` method returns [mmctgfmsccr::R](mmctgfmsccr::R) reader structure"]
impl crate::Readable for MMCTGFMSCCR {}
#[doc = "Ethernet MMC transmitted good frames after more than a single collision"]
pub mod mmctgfmsccr;
#[doc = "Ethernet MMC transmitted good frames counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmctgfcr](mmctgfcr) module"]
pub type MMCTGFCR = crate::Reg<u32, _MMCTGFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMCTGFCR;
#[doc = "`read()` method returns [mmctgfcr::R](mmctgfcr::R) reader structure"]
impl crate::Readable for MMCTGFCR {}
#[doc = "Ethernet MMC transmitted good frames counter register"]
pub mod mmctgfcr;
#[doc = "Ethernet MMC received frames with CRC error counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmcrfcecr](mmcrfcecr) module"]
pub type MMCRFCECR = crate::Reg<u32, _MMCRFCECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMCRFCECR;
#[doc = "`read()` method returns [mmcrfcecr::R](mmcrfcecr::R) reader structure"]
impl crate::Readable for MMCRFCECR {}
#[doc = "Ethernet MMC received frames with CRC error counter register"]
pub mod mmcrfcecr;
#[doc = "Ethernet MMC received frames with alignment error counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmcrfaecr](mmcrfaecr) module"]
pub type MMCRFAECR = crate::Reg<u32, _MMCRFAECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMCRFAECR;
#[doc = "`read()` method returns [mmcrfaecr::R](mmcrfaecr::R) reader structure"]
impl crate::Readable for MMCRFAECR {}
#[doc = "Ethernet MMC received frames with alignment error counter register"]
pub mod mmcrfaecr;
#[doc = "MMC received good unicast frames counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmcrgufcr](mmcrgufcr) module"]
pub type MMCRGUFCR = crate::Reg<u32, _MMCRGUFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMCRGUFCR;
#[doc = "`read()` method returns [mmcrgufcr::R](mmcrgufcr::R) reader structure"]
impl crate::Readable for MMCRGUFCR {}
#[doc = "MMC received good unicast frames counter register"]
pub mod mmcrgufcr;
