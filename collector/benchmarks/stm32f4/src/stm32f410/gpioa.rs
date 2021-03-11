#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub moder: MODER,
    #[doc = "0x04 - GPIO port output type register"]
    pub otyper: OTYPER,
    #[doc = "0x08 - GPIO port output speed register"]
    pub ospeedr: OSPEEDR,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub pupdr: PUPDR,
    #[doc = "0x10 - GPIO port input data register"]
    pub idr: IDR,
    #[doc = "0x14 - GPIO port output data register"]
    pub odr: ODR,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub bsrr: BSRR,
    #[doc = "0x1c - GPIO port configuration lock register"]
    pub lckr: LCKR,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub afrl: AFRL,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub afrh: AFRH,
}
#[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [moder](moder) module"]
pub type MODER = crate::Reg<u32, _MODER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODER;
#[doc = "`read()` method returns [moder::R](moder::R) reader structure"]
impl crate::Readable for MODER {}
#[doc = "`write(|w| ..)` method takes [moder::W](moder::W) writer structure"]
impl crate::Writable for MODER {}
#[doc = "GPIO port mode register"]
pub mod moder;
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otyper](otyper) module"]
pub type OTYPER = crate::Reg<u32, _OTYPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTYPER;
#[doc = "`read()` method returns [otyper::R](otyper::R) reader structure"]
impl crate::Readable for OTYPER {}
#[doc = "`write(|w| ..)` method takes [otyper::W](otyper::W) writer structure"]
impl crate::Writable for OTYPER {}
#[doc = "GPIO port output type register"]
pub mod otyper;
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ospeedr](ospeedr) module"]
pub type OSPEEDR = crate::Reg<u32, _OSPEEDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSPEEDR;
#[doc = "`read()` method returns [ospeedr::R](ospeedr::R) reader structure"]
impl crate::Readable for OSPEEDR {}
#[doc = "`write(|w| ..)` method takes [ospeedr::W](ospeedr::W) writer structure"]
impl crate::Writable for OSPEEDR {}
#[doc = "GPIO port output speed register"]
pub mod ospeedr;
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pupdr](pupdr) module"]
pub type PUPDR = crate::Reg<u32, _PUPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUPDR;
#[doc = "`read()` method returns [pupdr::R](pupdr::R) reader structure"]
impl crate::Readable for PUPDR {}
#[doc = "`write(|w| ..)` method takes [pupdr::W](pupdr::W) writer structure"]
impl crate::Writable for PUPDR {}
#[doc = "GPIO port pull-up/pull-down register"]
pub mod pupdr;
#[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`read()` method returns [idr::R](idr::R) reader structure"]
impl crate::Readable for IDR {}
#[doc = "GPIO port input data register"]
pub mod idr;
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odr](odr) module"]
pub type ODR = crate::Reg<u32, _ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODR;
#[doc = "`read()` method returns [odr::R](odr::R) reader structure"]
impl crate::Readable for ODR {}
#[doc = "`write(|w| ..)` method takes [odr::W](odr::W) writer structure"]
impl crate::Writable for ODR {}
#[doc = "GPIO port output data register"]
pub mod odr;
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsrr](bsrr) module"]
pub type BSRR = crate::Reg<u32, _BSRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSRR;
#[doc = "`write(|w| ..)` method takes [bsrr::W](bsrr::W) writer structure"]
impl crate::Writable for BSRR {}
#[doc = "GPIO port bit set/reset register"]
pub mod bsrr;
#[doc = "GPIO port configuration lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lckr](lckr) module"]
pub type LCKR = crate::Reg<u32, _LCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCKR;
#[doc = "`read()` method returns [lckr::R](lckr::R) reader structure"]
impl crate::Readable for LCKR {}
#[doc = "`write(|w| ..)` method takes [lckr::W](lckr::W) writer structure"]
impl crate::Writable for LCKR {}
#[doc = "GPIO port configuration lock register"]
pub mod lckr;
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrl](afrl) module"]
pub type AFRL = crate::Reg<u32, _AFRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFRL;
#[doc = "`read()` method returns [afrl::R](afrl::R) reader structure"]
impl crate::Readable for AFRL {}
#[doc = "`write(|w| ..)` method takes [afrl::W](afrl::W) writer structure"]
impl crate::Writable for AFRL {}
#[doc = "GPIO alternate function low register"]
pub mod afrl;
#[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrh](afrh) module"]
pub type AFRH = crate::Reg<u32, _AFRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFRH;
#[doc = "`read()` method returns [afrh::R](afrh::R) reader structure"]
impl crate::Readable for AFRH {}
#[doc = "`write(|w| ..)` method takes [afrh::W](afrh::W) writer structure"]
impl crate::Writable for AFRH {}
#[doc = "GPIO alternate function high register"]
pub mod afrh;
