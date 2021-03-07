#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - status register"]
    pub sr: SR,
    #[doc = "0x08 - data input register"]
    pub dinr: DINR,
    #[doc = "0x0c - data output register"]
    pub doutr: DOUTR,
    #[doc = "0x10 - key register 0"]
    pub keyr0: KEYR0,
    #[doc = "0x14 - key register 1"]
    pub keyr1: KEYR1,
    #[doc = "0x18 - key register 2"]
    pub keyr2: KEYR2,
    #[doc = "0x1c - key register 3"]
    pub keyr3: KEYR3,
    #[doc = "0x20 - initialization vector register 0"]
    pub ivr0: IVR0,
    #[doc = "0x24 - initialization vector register 1"]
    pub ivr1: IVR1,
    #[doc = "0x28 - initialization vector register 2"]
    pub ivr2: IVR2,
    #[doc = "0x2c - initialization vector register 3"]
    pub ivr3: IVR3,
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
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "status register"]
pub mod sr;
#[doc = "data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr](dinr) module"]
pub type DINR = crate::Reg<u32, _DINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR;
#[doc = "`read()` method returns [dinr::R](dinr::R) reader structure"]
impl crate::Readable for DINR {}
#[doc = "`write(|w| ..)` method takes [dinr::W](dinr::W) writer structure"]
impl crate::Writable for DINR {}
#[doc = "data input register"]
pub mod dinr;
#[doc = "data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr](doutr) module"]
pub type DOUTR = crate::Reg<u32, _DOUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR;
#[doc = "`read()` method returns [doutr::R](doutr::R) reader structure"]
impl crate::Readable for DOUTR {}
#[doc = "data output register"]
pub mod doutr;
#[doc = "key register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr0](keyr0) module"]
pub type KEYR0 = crate::Reg<u32, _KEYR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYR0;
#[doc = "`read()` method returns [keyr0::R](keyr0::R) reader structure"]
impl crate::Readable for KEYR0 {}
#[doc = "`write(|w| ..)` method takes [keyr0::W](keyr0::W) writer structure"]
impl crate::Writable for KEYR0 {}
#[doc = "key register 0"]
pub mod keyr0;
#[doc = "key register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr1](keyr1) module"]
pub type KEYR1 = crate::Reg<u32, _KEYR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYR1;
#[doc = "`read()` method returns [keyr1::R](keyr1::R) reader structure"]
impl crate::Readable for KEYR1 {}
#[doc = "`write(|w| ..)` method takes [keyr1::W](keyr1::W) writer structure"]
impl crate::Writable for KEYR1 {}
#[doc = "key register 1"]
pub mod keyr1;
#[doc = "key register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr2](keyr2) module"]
pub type KEYR2 = crate::Reg<u32, _KEYR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYR2;
#[doc = "`read()` method returns [keyr2::R](keyr2::R) reader structure"]
impl crate::Readable for KEYR2 {}
#[doc = "`write(|w| ..)` method takes [keyr2::W](keyr2::W) writer structure"]
impl crate::Writable for KEYR2 {}
#[doc = "key register 2"]
pub mod keyr2;
#[doc = "key register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr3](keyr3) module"]
pub type KEYR3 = crate::Reg<u32, _KEYR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYR3;
#[doc = "`read()` method returns [keyr3::R](keyr3::R) reader structure"]
impl crate::Readable for KEYR3 {}
#[doc = "`write(|w| ..)` method takes [keyr3::W](keyr3::W) writer structure"]
impl crate::Writable for KEYR3 {}
#[doc = "key register 3"]
pub mod keyr3;
#[doc = "initialization vector register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivr0](ivr0) module"]
pub type IVR0 = crate::Reg<u32, _IVR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IVR0;
#[doc = "`read()` method returns [ivr0::R](ivr0::R) reader structure"]
impl crate::Readable for IVR0 {}
#[doc = "`write(|w| ..)` method takes [ivr0::W](ivr0::W) writer structure"]
impl crate::Writable for IVR0 {}
#[doc = "initialization vector register 0"]
pub mod ivr0;
#[doc = "initialization vector register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivr1](ivr1) module"]
pub type IVR1 = crate::Reg<u32, _IVR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IVR1;
#[doc = "`read()` method returns [ivr1::R](ivr1::R) reader structure"]
impl crate::Readable for IVR1 {}
#[doc = "`write(|w| ..)` method takes [ivr1::W](ivr1::W) writer structure"]
impl crate::Writable for IVR1 {}
#[doc = "initialization vector register 1"]
pub mod ivr1;
#[doc = "initialization vector register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivr2](ivr2) module"]
pub type IVR2 = crate::Reg<u32, _IVR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IVR2;
#[doc = "`read()` method returns [ivr2::R](ivr2::R) reader structure"]
impl crate::Readable for IVR2 {}
#[doc = "`write(|w| ..)` method takes [ivr2::W](ivr2::W) writer structure"]
impl crate::Writable for IVR2 {}
#[doc = "initialization vector register 2"]
pub mod ivr2;
#[doc = "initialization vector register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivr3](ivr3) module"]
pub type IVR3 = crate::Reg<u32, _IVR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IVR3;
#[doc = "`read()` method returns [ivr3::R](ivr3::R) reader structure"]
impl crate::Readable for IVR3 {}
#[doc = "`write(|w| ..)` method takes [ivr3::W](ivr3::W) writer structure"]
impl crate::Writable for IVR3 {}
#[doc = "initialization vector register 3"]
pub mod ivr3;
