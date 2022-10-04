#[doc = "Register `PLLCFGR` reader"]
pub struct R(crate::R<PLLCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCFGR` writer"]
pub struct W(crate::W<PLLCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PLLCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Main PLL(PLL) and audio PLL (PLLI2S) entry clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSRC_A {
    #[doc = "0: HSI clock selected as PLL and PLLI2S clock entry"]
    HSI = 0,
    #[doc = "1: HSE oscillator clock selected as PLL and PLLI2S clock entry"]
    HSE = 1,
}
impl From<PLLSRC_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSRC` reader - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
pub struct PLLSRC_R(crate::FieldReader<bool, PLLSRC_A>);
impl PLLSRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSRC_A {
        match self.bits {
            false => PLLSRC_A::HSI,
            true => PLLSRC_A::HSE,
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        **self == PLLSRC_A::HSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == PLLSRC_A::HSE
    }
}
impl core::ops::Deref for PLLSRC_R {
    type Target = crate::FieldReader<bool, PLLSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSRC` writer - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
pub struct PLLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HSI clock selected as PLL and PLLI2S clock entry"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSI)
    }
    #[doc = "HSE oscillator clock selected as PLL and PLLI2S clock entry"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `PLLR` reader - Main PLL division factor for I2S, DFSDM clocks"]
pub struct PLLR_R(crate::FieldReader<u8, u8>);
impl PLLR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLR` writer - Main PLL division factor for I2S, DFSDM clocks"]
pub struct PLLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `PLLM` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub struct PLLM_R(crate::FieldReader<u8, u8>);
impl PLLM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLM` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub struct PLLM_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `PLLN` reader - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN_R(crate::FieldReader<u16, u16>);
impl PLLN_R {
    pub(crate) fn new(bits: u16) -> Self {
        PLLN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLN` writer - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 6)) | ((value as u32 & 0x01ff) << 6);
        self.w
    }
}
#[doc = "Main PLL (PLL) division factor for main system clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLP_A {
    #[doc = "0: PLLP=2"]
    DIV2 = 0,
    #[doc = "1: PLLP=4"]
    DIV4 = 1,
    #[doc = "2: PLLP=6"]
    DIV6 = 2,
    #[doc = "3: PLLP=8"]
    DIV8 = 3,
}
impl From<PLLP_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLP` reader - Main PLL (PLL) division factor for main system clock"]
pub struct PLLP_R(crate::FieldReader<u8, PLLP_A>);
impl PLLP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLP_A {
        match self.bits {
            0 => PLLP_A::DIV2,
            1 => PLLP_A::DIV4,
            2 => PLLP_A::DIV6,
            3 => PLLP_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PLLP_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PLLP_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        **self == PLLP_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PLLP_A::DIV8
    }
}
impl core::ops::Deref for PLLP_R {
    type Target = crate::FieldReader<u8, PLLP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLP` writer - Main PLL (PLL) division factor for main system clock"]
pub struct PLLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PLLP=2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLP_A::DIV2)
    }
    #[doc = "PLLP=4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLP_A::DIV4)
    }
    #[doc = "PLLP=6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLP_A::DIV6)
    }
    #[doc = "PLLP=8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLP_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `PLLQ` reader - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub struct PLLQ_R(crate::FieldReader<u8, u8>);
impl PLLQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLQ` writer - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub struct PLLQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - Main PLL division factor for I2S, DFSDM clocks"]
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 0:5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:14 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W {
        PLLSRC_W { w: self }
    }
    #[doc = "Bits 28:30 - Main PLL division factor for I2S, DFSDM clocks"]
    #[inline(always)]
    pub fn pllr(&mut self) -> PLLR_W {
        PLLR_W { w: self }
    }
    #[doc = "Bits 0:5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm(&mut self) -> PLLM_W {
        PLLM_W { w: self }
    }
    #[doc = "Bits 6:14 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln(&mut self) -> PLLN_W {
        PLLN_W { w: self }
    }
    #[doc = "Bits 16:17 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    pub fn pllp(&mut self) -> PLLP_W {
        PLLP_W { w: self }
    }
    #[doc = "Bits 24:27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq(&mut self) -> PLLQ_W {
        PLLQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfgr](index.html) module"]
pub struct PLLCFGR_SPEC;
impl crate::RegisterSpec for PLLCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcfgr::R](R) reader structure"]
impl crate::Readable for PLLCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcfgr::W](W) writer structure"]
impl crate::Writable for PLLCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x2400_3010"]
impl crate::Resettable for PLLCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2400_3010
    }
}
