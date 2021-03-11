#[doc = "Layerx Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Layerx Control Register"]
pub mod cr;
#[doc = "Layerx Window Horizontal Position Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [whpcr](whpcr) module"]
pub type WHPCR = crate::Reg<u32, _WHPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WHPCR;
#[doc = "`read()` method returns [whpcr::R](whpcr::R) reader structure"]
impl crate::Readable for WHPCR {}
#[doc = "`write(|w| ..)` method takes [whpcr::W](whpcr::W) writer structure"]
impl crate::Writable for WHPCR {}
#[doc = "Layerx Window Horizontal Position Configuration Register"]
pub mod whpcr;
#[doc = "Layerx Window Vertical Position Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wvpcr](wvpcr) module"]
pub type WVPCR = crate::Reg<u32, _WVPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WVPCR;
#[doc = "`read()` method returns [wvpcr::R](wvpcr::R) reader structure"]
impl crate::Readable for WVPCR {}
#[doc = "`write(|w| ..)` method takes [wvpcr::W](wvpcr::W) writer structure"]
impl crate::Writable for WVPCR {}
#[doc = "Layerx Window Vertical Position Configuration Register"]
pub mod wvpcr;
#[doc = "Layerx Color Keying Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcr](ckcr) module"]
pub type CKCR = crate::Reg<u32, _CKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCR;
#[doc = "`read()` method returns [ckcr::R](ckcr::R) reader structure"]
impl crate::Readable for CKCR {}
#[doc = "`write(|w| ..)` method takes [ckcr::W](ckcr::W) writer structure"]
impl crate::Writable for CKCR {}
#[doc = "Layerx Color Keying Configuration Register"]
pub mod ckcr;
#[doc = "Layerx Pixel Format Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfcr](pfcr) module"]
pub type PFCR = crate::Reg<u32, _PFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFCR;
#[doc = "`read()` method returns [pfcr::R](pfcr::R) reader structure"]
impl crate::Readable for PFCR {}
#[doc = "`write(|w| ..)` method takes [pfcr::W](pfcr::W) writer structure"]
impl crate::Writable for PFCR {}
#[doc = "Layerx Pixel Format Configuration Register"]
pub mod pfcr;
#[doc = "Layerx Constant Alpha Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cacr](cacr) module"]
pub type CACR = crate::Reg<u32, _CACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CACR;
#[doc = "`read()` method returns [cacr::R](cacr::R) reader structure"]
impl crate::Readable for CACR {}
#[doc = "`write(|w| ..)` method takes [cacr::W](cacr::W) writer structure"]
impl crate::Writable for CACR {}
#[doc = "Layerx Constant Alpha Configuration Register"]
pub mod cacr;
#[doc = "Layerx Default Color Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccr](dccr) module"]
pub type DCCR = crate::Reg<u32, _DCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCR;
#[doc = "`read()` method returns [dccr::R](dccr::R) reader structure"]
impl crate::Readable for DCCR {}
#[doc = "`write(|w| ..)` method takes [dccr::W](dccr::W) writer structure"]
impl crate::Writable for DCCR {}
#[doc = "Layerx Default Color Configuration Register"]
pub mod dccr;
#[doc = "Layerx Blending Factors Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfcr](bfcr) module"]
pub type BFCR = crate::Reg<u32, _BFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFCR;
#[doc = "`read()` method returns [bfcr::R](bfcr::R) reader structure"]
impl crate::Readable for BFCR {}
#[doc = "`write(|w| ..)` method takes [bfcr::W](bfcr::W) writer structure"]
impl crate::Writable for BFCR {}
#[doc = "Layerx Blending Factors Configuration Register"]
pub mod bfcr;
#[doc = "Layerx Color Frame Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfbar](cfbar) module"]
pub type CFBAR = crate::Reg<u32, _CFBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFBAR;
#[doc = "`read()` method returns [cfbar::R](cfbar::R) reader structure"]
impl crate::Readable for CFBAR {}
#[doc = "`write(|w| ..)` method takes [cfbar::W](cfbar::W) writer structure"]
impl crate::Writable for CFBAR {}
#[doc = "Layerx Color Frame Buffer Address Register"]
pub mod cfbar;
#[doc = "Layerx Color Frame Buffer Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfblr](cfblr) module"]
pub type CFBLR = crate::Reg<u32, _CFBLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFBLR;
#[doc = "`read()` method returns [cfblr::R](cfblr::R) reader structure"]
impl crate::Readable for CFBLR {}
#[doc = "`write(|w| ..)` method takes [cfblr::W](cfblr::W) writer structure"]
impl crate::Writable for CFBLR {}
#[doc = "Layerx Color Frame Buffer Length Register"]
pub mod cfblr;
#[doc = "Layerx ColorFrame Buffer Line Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfblnr](cfblnr) module"]
pub type CFBLNR = crate::Reg<u32, _CFBLNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFBLNR;
#[doc = "`read()` method returns [cfblnr::R](cfblnr::R) reader structure"]
impl crate::Readable for CFBLNR {}
#[doc = "`write(|w| ..)` method takes [cfblnr::W](cfblnr::W) writer structure"]
impl crate::Writable for CFBLNR {}
#[doc = "Layerx ColorFrame Buffer Line Number Register"]
pub mod cfblnr;
#[doc = "Layerx CLUT Write Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clutwr](clutwr) module"]
pub type CLUTWR = crate::Reg<u32, _CLUTWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLUTWR;
#[doc = "`write(|w| ..)` method takes [clutwr::W](clutwr::W) writer structure"]
impl crate::Writable for CLUTWR {}
#[doc = "Layerx CLUT Write Register"]
pub mod clutwr;
