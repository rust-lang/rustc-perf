#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet PTP time stamp control register"]
    pub ptptscr: PTPTSCR,
    #[doc = "0x04 - Ethernet PTP subsecond increment register"]
    pub ptpssir: PTPSSIR,
    #[doc = "0x08 - Ethernet PTP time stamp high register"]
    pub ptptshr: PTPTSHR,
    #[doc = "0x0c - Ethernet PTP time stamp low register"]
    pub ptptslr: PTPTSLR,
    #[doc = "0x10 - Ethernet PTP time stamp high update register"]
    pub ptptshur: PTPTSHUR,
    #[doc = "0x14 - Ethernet PTP time stamp low update register"]
    pub ptptslur: PTPTSLUR,
    #[doc = "0x18 - Ethernet PTP time stamp addend register"]
    pub ptptsar: PTPTSAR,
    #[doc = "0x1c - Ethernet PTP target time high register"]
    pub ptptthr: PTPTTHR,
    #[doc = "0x20 - Ethernet PTP target time low register"]
    pub ptpttlr: PTPTTLR,
    _reserved9: [u8; 4usize],
    #[doc = "0x28 - Ethernet PTP time stamp status register"]
    pub ptptssr: PTPTSSR,
    #[doc = "0x2c - Ethernet PTP PPS control register"]
    pub ptpppscr: PTPPPSCR,
}
#[doc = "Ethernet PTP time stamp control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptptscr](ptptscr) module"]
pub type PTPTSCR = crate::Reg<u32, _PTPTSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPTSCR;
#[doc = "`read()` method returns [ptptscr::R](ptptscr::R) reader structure"]
impl crate::Readable for PTPTSCR {}
#[doc = "`write(|w| ..)` method takes [ptptscr::W](ptptscr::W) writer structure"]
impl crate::Writable for PTPTSCR {}
#[doc = "Ethernet PTP time stamp control register"]
pub mod ptptscr;
#[doc = "Ethernet PTP subsecond increment register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptpssir](ptpssir) module"]
pub type PTPSSIR = crate::Reg<u32, _PTPSSIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPSSIR;
#[doc = "`read()` method returns [ptpssir::R](ptpssir::R) reader structure"]
impl crate::Readable for PTPSSIR {}
#[doc = "`write(|w| ..)` method takes [ptpssir::W](ptpssir::W) writer structure"]
impl crate::Writable for PTPSSIR {}
#[doc = "Ethernet PTP subsecond increment register"]
pub mod ptpssir;
#[doc = "Ethernet PTP time stamp high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptptshr](ptptshr) module"]
pub type PTPTSHR = crate::Reg<u32, _PTPTSHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPTSHR;
#[doc = "`read()` method returns [ptptshr::R](ptptshr::R) reader structure"]
impl crate::Readable for PTPTSHR {}
#[doc = "Ethernet PTP time stamp high register"]
pub mod ptptshr;
#[doc = "Ethernet PTP time stamp low register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptptslr](ptptslr) module"]
pub type PTPTSLR = crate::Reg<u32, _PTPTSLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPTSLR;
#[doc = "`read()` method returns [ptptslr::R](ptptslr::R) reader structure"]
impl crate::Readable for PTPTSLR {}
#[doc = "Ethernet PTP time stamp low register"]
pub mod ptptslr;
#[doc = "Ethernet PTP time stamp high update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptptshur](ptptshur) module"]
pub type PTPTSHUR = crate::Reg<u32, _PTPTSHUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPTSHUR;
#[doc = "`read()` method returns [ptptshur::R](ptptshur::R) reader structure"]
impl crate::Readable for PTPTSHUR {}
#[doc = "`write(|w| ..)` method takes [ptptshur::W](ptptshur::W) writer structure"]
impl crate::Writable for PTPTSHUR {}
#[doc = "Ethernet PTP time stamp high update register"]
pub mod ptptshur;
#[doc = "Ethernet PTP time stamp low update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptptslur](ptptslur) module"]
pub type PTPTSLUR = crate::Reg<u32, _PTPTSLUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPTSLUR;
#[doc = "`read()` method returns [ptptslur::R](ptptslur::R) reader structure"]
impl crate::Readable for PTPTSLUR {}
#[doc = "`write(|w| ..)` method takes [ptptslur::W](ptptslur::W) writer structure"]
impl crate::Writable for PTPTSLUR {}
#[doc = "Ethernet PTP time stamp low update register"]
pub mod ptptslur;
#[doc = "Ethernet PTP time stamp addend register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptptsar](ptptsar) module"]
pub type PTPTSAR = crate::Reg<u32, _PTPTSAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPTSAR;
#[doc = "`read()` method returns [ptptsar::R](ptptsar::R) reader structure"]
impl crate::Readable for PTPTSAR {}
#[doc = "`write(|w| ..)` method takes [ptptsar::W](ptptsar::W) writer structure"]
impl crate::Writable for PTPTSAR {}
#[doc = "Ethernet PTP time stamp addend register"]
pub mod ptptsar;
#[doc = "Ethernet PTP target time high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptptthr](ptptthr) module"]
pub type PTPTTHR = crate::Reg<u32, _PTPTTHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPTTHR;
#[doc = "`read()` method returns [ptptthr::R](ptptthr::R) reader structure"]
impl crate::Readable for PTPTTHR {}
#[doc = "`write(|w| ..)` method takes [ptptthr::W](ptptthr::W) writer structure"]
impl crate::Writable for PTPTTHR {}
#[doc = "Ethernet PTP target time high register"]
pub mod ptptthr;
#[doc = "Ethernet PTP target time low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptpttlr](ptpttlr) module"]
pub type PTPTTLR = crate::Reg<u32, _PTPTTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPTTLR;
#[doc = "`read()` method returns [ptpttlr::R](ptpttlr::R) reader structure"]
impl crate::Readable for PTPTTLR {}
#[doc = "`write(|w| ..)` method takes [ptpttlr::W](ptpttlr::W) writer structure"]
impl crate::Writable for PTPTTLR {}
#[doc = "Ethernet PTP target time low register"]
pub mod ptpttlr;
#[doc = "Ethernet PTP time stamp status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptptssr](ptptssr) module"]
pub type PTPTSSR = crate::Reg<u32, _PTPTSSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPTSSR;
#[doc = "`read()` method returns [ptptssr::R](ptptssr::R) reader structure"]
impl crate::Readable for PTPTSSR {}
#[doc = "Ethernet PTP time stamp status register"]
pub mod ptptssr;
#[doc = "Ethernet PTP PPS control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptpppscr](ptpppscr) module"]
pub type PTPPPSCR = crate::Reg<u32, _PTPPPSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTPPPSCR;
#[doc = "`read()` method returns [ptpppscr::R](ptpppscr::R) reader structure"]
impl crate::Readable for PTPPPSCR {}
#[doc = "Ethernet PTP PPS control register"]
pub mod ptpppscr;
