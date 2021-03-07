#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet DMA bus mode register"]
    pub dmabmr: DMABMR,
    #[doc = "0x04 - Ethernet DMA transmit poll demand register"]
    pub dmatpdr: DMATPDR,
    #[doc = "0x08 - EHERNET DMA receive poll demand register"]
    pub dmarpdr: DMARPDR,
    #[doc = "0x0c - Ethernet DMA receive descriptor list address register"]
    pub dmardlar: DMARDLAR,
    #[doc = "0x10 - Ethernet DMA transmit descriptor list address register"]
    pub dmatdlar: DMATDLAR,
    #[doc = "0x14 - Ethernet DMA status register"]
    pub dmasr: DMASR,
    #[doc = "0x18 - Ethernet DMA operation mode register"]
    pub dmaomr: DMAOMR,
    #[doc = "0x1c - Ethernet DMA interrupt enable register"]
    pub dmaier: DMAIER,
    #[doc = "0x20 - Ethernet DMA missed frame and buffer overflow counter register"]
    pub dmamfbocr: DMAMFBOCR,
    #[doc = "0x24 - Ethernet DMA receive status watchdog timer register"]
    pub dmarswtr: DMARSWTR,
    _reserved10: [u8; 32usize],
    #[doc = "0x48 - Ethernet DMA current host transmit descriptor register"]
    pub dmachtdr: DMACHTDR,
    #[doc = "0x4c - Ethernet DMA current host receive descriptor register"]
    pub dmachrdr: DMACHRDR,
    #[doc = "0x50 - Ethernet DMA current host transmit buffer address register"]
    pub dmachtbar: DMACHTBAR,
    #[doc = "0x54 - Ethernet DMA current host receive buffer address register"]
    pub dmachrbar: DMACHRBAR,
}
#[doc = "Ethernet DMA bus mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmabmr](dmabmr) module"]
pub type DMABMR = crate::Reg<u32, _DMABMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMABMR;
#[doc = "`read()` method returns [dmabmr::R](dmabmr::R) reader structure"]
impl crate::Readable for DMABMR {}
#[doc = "`write(|w| ..)` method takes [dmabmr::W](dmabmr::W) writer structure"]
impl crate::Writable for DMABMR {}
#[doc = "Ethernet DMA bus mode register"]
pub mod dmabmr;
#[doc = "Ethernet DMA transmit poll demand register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatpdr](dmatpdr) module"]
pub type DMATPDR = crate::Reg<u32, _DMATPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMATPDR;
#[doc = "`read()` method returns [dmatpdr::R](dmatpdr::R) reader structure"]
impl crate::Readable for DMATPDR {}
#[doc = "`write(|w| ..)` method takes [dmatpdr::W](dmatpdr::W) writer structure"]
impl crate::Writable for DMATPDR {}
#[doc = "Ethernet DMA transmit poll demand register"]
pub mod dmatpdr;
#[doc = "EHERNET DMA receive poll demand register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmarpdr](dmarpdr) module"]
pub type DMARPDR = crate::Reg<u32, _DMARPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMARPDR;
#[doc = "`read()` method returns [dmarpdr::R](dmarpdr::R) reader structure"]
impl crate::Readable for DMARPDR {}
#[doc = "`write(|w| ..)` method takes [dmarpdr::W](dmarpdr::W) writer structure"]
impl crate::Writable for DMARPDR {}
#[doc = "EHERNET DMA receive poll demand register"]
pub mod dmarpdr;
#[doc = "Ethernet DMA receive descriptor list address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmardlar](dmardlar) module"]
pub type DMARDLAR = crate::Reg<u32, _DMARDLAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMARDLAR;
#[doc = "`read()` method returns [dmardlar::R](dmardlar::R) reader structure"]
impl crate::Readable for DMARDLAR {}
#[doc = "`write(|w| ..)` method takes [dmardlar::W](dmardlar::W) writer structure"]
impl crate::Writable for DMARDLAR {}
#[doc = "Ethernet DMA receive descriptor list address register"]
pub mod dmardlar;
#[doc = "Ethernet DMA transmit descriptor list address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatdlar](dmatdlar) module"]
pub type DMATDLAR = crate::Reg<u32, _DMATDLAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMATDLAR;
#[doc = "`read()` method returns [dmatdlar::R](dmatdlar::R) reader structure"]
impl crate::Readable for DMATDLAR {}
#[doc = "`write(|w| ..)` method takes [dmatdlar::W](dmatdlar::W) writer structure"]
impl crate::Writable for DMATDLAR {}
#[doc = "Ethernet DMA transmit descriptor list address register"]
pub mod dmatdlar;
#[doc = "Ethernet DMA status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasr](dmasr) module"]
pub type DMASR = crate::Reg<u32, _DMASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASR;
#[doc = "`read()` method returns [dmasr::R](dmasr::R) reader structure"]
impl crate::Readable for DMASR {}
#[doc = "`write(|w| ..)` method takes [dmasr::W](dmasr::W) writer structure"]
impl crate::Writable for DMASR {}
#[doc = "Ethernet DMA status register"]
pub mod dmasr;
#[doc = "Ethernet DMA operation mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaomr](dmaomr) module"]
pub type DMAOMR = crate::Reg<u32, _DMAOMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAOMR;
#[doc = "`read()` method returns [dmaomr::R](dmaomr::R) reader structure"]
impl crate::Readable for DMAOMR {}
#[doc = "`write(|w| ..)` method takes [dmaomr::W](dmaomr::W) writer structure"]
impl crate::Writable for DMAOMR {}
#[doc = "Ethernet DMA operation mode register"]
pub mod dmaomr;
#[doc = "Ethernet DMA interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaier](dmaier) module"]
pub type DMAIER = crate::Reg<u32, _DMAIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAIER;
#[doc = "`read()` method returns [dmaier::R](dmaier::R) reader structure"]
impl crate::Readable for DMAIER {}
#[doc = "`write(|w| ..)` method takes [dmaier::W](dmaier::W) writer structure"]
impl crate::Writable for DMAIER {}
#[doc = "Ethernet DMA interrupt enable register"]
pub mod dmaier;
#[doc = "Ethernet DMA missed frame and buffer overflow counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamfbocr](dmamfbocr) module"]
pub type DMAMFBOCR = crate::Reg<u32, _DMAMFBOCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMFBOCR;
#[doc = "`read()` method returns [dmamfbocr::R](dmamfbocr::R) reader structure"]
impl crate::Readable for DMAMFBOCR {}
#[doc = "`write(|w| ..)` method takes [dmamfbocr::W](dmamfbocr::W) writer structure"]
impl crate::Writable for DMAMFBOCR {}
#[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
pub mod dmamfbocr;
#[doc = "Ethernet DMA receive status watchdog timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmarswtr](dmarswtr) module"]
pub type DMARSWTR = crate::Reg<u32, _DMARSWTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMARSWTR;
#[doc = "`read()` method returns [dmarswtr::R](dmarswtr::R) reader structure"]
impl crate::Readable for DMARSWTR {}
#[doc = "`write(|w| ..)` method takes [dmarswtr::W](dmarswtr::W) writer structure"]
impl crate::Writable for DMARSWTR {}
#[doc = "Ethernet DMA receive status watchdog timer register"]
pub mod dmarswtr;
#[doc = "Ethernet DMA current host transmit descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmachtdr](dmachtdr) module"]
pub type DMACHTDR = crate::Reg<u32, _DMACHTDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACHTDR;
#[doc = "`read()` method returns [dmachtdr::R](dmachtdr::R) reader structure"]
impl crate::Readable for DMACHTDR {}
#[doc = "Ethernet DMA current host transmit descriptor register"]
pub mod dmachtdr;
#[doc = "Ethernet DMA current host receive descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmachrdr](dmachrdr) module"]
pub type DMACHRDR = crate::Reg<u32, _DMACHRDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACHRDR;
#[doc = "`read()` method returns [dmachrdr::R](dmachrdr::R) reader structure"]
impl crate::Readable for DMACHRDR {}
#[doc = "Ethernet DMA current host receive descriptor register"]
pub mod dmachrdr;
#[doc = "Ethernet DMA current host transmit buffer address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmachtbar](dmachtbar) module"]
pub type DMACHTBAR = crate::Reg<u32, _DMACHTBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACHTBAR;
#[doc = "`read()` method returns [dmachtbar::R](dmachtbar::R) reader structure"]
impl crate::Readable for DMACHTBAR {}
#[doc = "Ethernet DMA current host transmit buffer address register"]
pub mod dmachtbar;
#[doc = "Ethernet DMA current host receive buffer address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmachrbar](dmachrbar) module"]
pub type DMACHRBAR = crate::Reg<u32, _DMACHRBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACHRBAR;
#[doc = "`read()` method returns [dmachrbar::R](dmachrbar::R) reader structure"]
impl crate::Readable for DMACHRBAR {}
#[doc = "Ethernet DMA current host receive buffer address register"]
pub mod dmachrbar;
