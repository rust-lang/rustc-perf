#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Synchronization Size Configuration Register"]
    pub sscr: SSCR,
    #[doc = "0x0c - Back Porch Configuration Register"]
    pub bpcr: BPCR,
    #[doc = "0x10 - Active Width Configuration Register"]
    pub awcr: AWCR,
    #[doc = "0x14 - Total Width Configuration Register"]
    pub twcr: TWCR,
    #[doc = "0x18 - Global Control Register"]
    pub gcr: GCR,
    _reserved5: [u8; 8usize],
    #[doc = "0x24 - Shadow Reload Configuration Register"]
    pub srcr: SRCR,
    _reserved6: [u8; 4usize],
    #[doc = "0x2c - Background Color Configuration Register"]
    pub bccr: BCCR,
    _reserved7: [u8; 4usize],
    #[doc = "0x34 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x38 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x3c - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x40 - Line Interrupt Position Configuration Register"]
    pub lipcr: LIPCR,
    #[doc = "0x44 - Current Position Status Register"]
    pub cpsr: CPSR,
    #[doc = "0x48 - Current Display Status Register"]
    pub cdsr: CDSR,
    _reserved13: [u8; 56usize],
    #[doc = "0x84 - Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
    pub layer1: LAYER,
    _reserved14: [u8; 60usize],
    #[doc = "0x104 - Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
    pub layer2: LAYER,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct LAYER {
    #[doc = "0x00 - Layerx Control Register"]
    pub cr: self::layer::CR,
    #[doc = "0x04 - Layerx Window Horizontal Position Configuration Register"]
    pub whpcr: self::layer::WHPCR,
    #[doc = "0x08 - Layerx Window Vertical Position Configuration Register"]
    pub wvpcr: self::layer::WVPCR,
    #[doc = "0x0c - Layerx Color Keying Configuration Register"]
    pub ckcr: self::layer::CKCR,
    #[doc = "0x10 - Layerx Pixel Format Configuration Register"]
    pub pfcr: self::layer::PFCR,
    #[doc = "0x14 - Layerx Constant Alpha Configuration Register"]
    pub cacr: self::layer::CACR,
    #[doc = "0x18 - Layerx Default Color Configuration Register"]
    pub dccr: self::layer::DCCR,
    #[doc = "0x1c - Layerx Blending Factors Configuration Register"]
    pub bfcr: self::layer::BFCR,
    _reserved8: [u8; 8usize],
    #[doc = "0x28 - Layerx Color Frame Buffer Address Register"]
    pub cfbar: self::layer::CFBAR,
    #[doc = "0x2c - Layerx Color Frame Buffer Length Register"]
    pub cfblr: self::layer::CFBLR,
    #[doc = "0x30 - Layerx ColorFrame Buffer Line Number Register"]
    pub cfblnr: self::layer::CFBLNR,
    _reserved11: [u8; 12usize],
    #[doc = "0x40 - Layerx CLUT Write Register"]
    pub clutwr: self::layer::CLUTWR,
}
#[doc = r"Register block"]
#[doc = "Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
pub mod layer;
#[doc = "Synchronization Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sscr](sscr) module"]
pub type SSCR = crate::Reg<u32, _SSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSCR;
#[doc = "`read()` method returns [sscr::R](sscr::R) reader structure"]
impl crate::Readable for SSCR {}
#[doc = "`write(|w| ..)` method takes [sscr::W](sscr::W) writer structure"]
impl crate::Writable for SSCR {}
#[doc = "Synchronization Size Configuration Register"]
pub mod sscr;
#[doc = "Back Porch Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpcr](bpcr) module"]
pub type BPCR = crate::Reg<u32, _BPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BPCR;
#[doc = "`read()` method returns [bpcr::R](bpcr::R) reader structure"]
impl crate::Readable for BPCR {}
#[doc = "`write(|w| ..)` method takes [bpcr::W](bpcr::W) writer structure"]
impl crate::Writable for BPCR {}
#[doc = "Back Porch Configuration Register"]
pub mod bpcr;
#[doc = "Active Width Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awcr](awcr) module"]
pub type AWCR = crate::Reg<u32, _AWCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWCR;
#[doc = "`read()` method returns [awcr::R](awcr::R) reader structure"]
impl crate::Readable for AWCR {}
#[doc = "`write(|w| ..)` method takes [awcr::W](awcr::W) writer structure"]
impl crate::Writable for AWCR {}
#[doc = "Active Width Configuration Register"]
pub mod awcr;
#[doc = "Total Width Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twcr](twcr) module"]
pub type TWCR = crate::Reg<u32, _TWCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWCR;
#[doc = "`read()` method returns [twcr::R](twcr::R) reader structure"]
impl crate::Readable for TWCR {}
#[doc = "`write(|w| ..)` method takes [twcr::W](twcr::W) writer structure"]
impl crate::Writable for TWCR {}
#[doc = "Total Width Configuration Register"]
pub mod twcr;
#[doc = "Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcr](gcr) module"]
pub type GCR = crate::Reg<u32, _GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCR;
#[doc = "`read()` method returns [gcr::R](gcr::R) reader structure"]
impl crate::Readable for GCR {}
#[doc = "`write(|w| ..)` method takes [gcr::W](gcr::W) writer structure"]
impl crate::Writable for GCR {}
#[doc = "Global Control Register"]
pub mod gcr;
#[doc = "Shadow Reload Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcr](srcr) module"]
pub type SRCR = crate::Reg<u32, _SRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCR;
#[doc = "`read()` method returns [srcr::R](srcr::R) reader structure"]
impl crate::Readable for SRCR {}
#[doc = "`write(|w| ..)` method takes [srcr::W](srcr::W) writer structure"]
impl crate::Writable for SRCR {}
#[doc = "Shadow Reload Configuration Register"]
pub mod srcr;
#[doc = "Background Color Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bccr](bccr) module"]
pub type BCCR = crate::Reg<u32, _BCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCCR;
#[doc = "`read()` method returns [bccr::R](bccr::R) reader structure"]
impl crate::Readable for BCCR {}
#[doc = "`write(|w| ..)` method takes [bccr::W](bccr::W) writer structure"]
impl crate::Writable for BCCR {}
#[doc = "Background Color Configuration Register"]
pub mod bccr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "Line Interrupt Position Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lipcr](lipcr) module"]
pub type LIPCR = crate::Reg<u32, _LIPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LIPCR;
#[doc = "`read()` method returns [lipcr::R](lipcr::R) reader structure"]
impl crate::Readable for LIPCR {}
#[doc = "`write(|w| ..)` method takes [lipcr::W](lipcr::W) writer structure"]
impl crate::Writable for LIPCR {}
#[doc = "Line Interrupt Position Configuration Register"]
pub mod lipcr;
#[doc = "Current Position Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpsr](cpsr) module"]
pub type CPSR = crate::Reg<u32, _CPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPSR;
#[doc = "`read()` method returns [cpsr::R](cpsr::R) reader structure"]
impl crate::Readable for CPSR {}
#[doc = "Current Position Status Register"]
pub mod cpsr;
#[doc = "Current Display Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdsr](cdsr) module"]
pub type CDSR = crate::Reg<u32, _CDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDSR;
#[doc = "`read()` method returns [cdsr::R](cdsr::R) reader structure"]
impl crate::Readable for CDSR {}
#[doc = "Current Display Status Register"]
pub mod cdsr;
