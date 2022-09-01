#[doc = "Register `RF%sR` reader"]
pub struct R(crate::R<RFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF%sR` writer"]
pub struct W(crate::W<RFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFR_SPEC>;
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
impl From<crate::W<RFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RFOM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFOM_A {
    #[doc = "1: Set by software to release the output mailbox of the FIFO"]
    RELEASE = 1,
}
impl From<RFOM_A> for bool {
    #[inline(always)]
    fn from(variant: RFOM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFOM` reader - RFOM0"]
pub struct RFOM_R(crate::FieldReader<bool, RFOM_A>);
impl RFOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFOM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RFOM_A> {
        match self.bits {
            true => Some(RFOM_A::RELEASE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASE`"]
    #[inline(always)]
    pub fn is_release(&self) -> bool {
        **self == RFOM_A::RELEASE
    }
}
impl core::ops::Deref for RFOM_R {
    type Target = crate::FieldReader<bool, RFOM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFOM` writer - RFOM0"]
pub struct RFOM_W<'a> {
    w: &'a mut W,
}
impl<'a> RFOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFOM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set by software to release the output mailbox of the FIFO"]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(RFOM_A::RELEASE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "FOVR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOVR_A {
    #[doc = "0: No FIFO x overrun"]
    NOOVERRUN = 0,
    #[doc = "1: FIFO x overrun"]
    OVERRUN = 1,
}
impl From<FOVR_A> for bool {
    #[inline(always)]
    fn from(variant: FOVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOVR` reader - FOVR0"]
pub struct FOVR_R(crate::FieldReader<bool, FOVR_A>);
impl FOVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FOVR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOVR_A {
        match self.bits {
            false => FOVR_A::NOOVERRUN,
            true => FOVR_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOOVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        **self == FOVR_A::NOOVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        **self == FOVR_A::OVERRUN
    }
}
impl core::ops::Deref for FOVR_R {
    type Target = crate::FieldReader<bool, FOVR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "FOVR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOVR_AW {
    #[doc = "1: Clear flag"]
    CLEAR = 1,
}
impl From<FOVR_AW> for bool {
    #[inline(always)]
    fn from(variant: FOVR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOVR` writer - FOVR0"]
pub struct FOVR_W<'a> {
    w: &'a mut W,
}
impl<'a> FOVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOVR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FOVR_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "FULL0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FULL_A {
    #[doc = "0: FIFO x is not full"]
    NOTFULL = 0,
    #[doc = "1: FIFO x is full"]
    FULL = 1,
}
impl From<FULL_A> for bool {
    #[inline(always)]
    fn from(variant: FULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FULL` reader - FULL0"]
pub struct FULL_R(crate::FieldReader<bool, FULL_A>);
impl FULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FULL_A {
        match self.bits {
            false => FULL_A::NOTFULL,
            true => FULL_A::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `NOTFULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        **self == FULL_A::NOTFULL
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        **self == FULL_A::FULL
    }
}
impl core::ops::Deref for FULL_R {
    type Target = crate::FieldReader<bool, FULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "FULL0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FULL_AW {
    #[doc = "1: Clear flag"]
    CLEAR = 1,
}
impl From<FULL_AW> for bool {
    #[inline(always)]
    fn from(variant: FULL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FULL` writer - FULL0"]
pub struct FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FULL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FULL_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `FMP` reader - FMP0"]
pub struct FMP_R(crate::FieldReader<u8, u8>);
impl FMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 5 - RFOM0"]
    #[inline(always)]
    pub fn rfom(&self) -> RFOM_R {
        RFOM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FOVR0"]
    #[inline(always)]
    pub fn fovr(&self) -> FOVR_R {
        FOVR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FULL0"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - FMP0"]
    #[inline(always)]
    pub fn fmp(&self) -> FMP_R {
        FMP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - RFOM0"]
    #[inline(always)]
    pub fn rfom(&mut self) -> RFOM_W {
        RFOM_W { w: self }
    }
    #[doc = "Bit 4 - FOVR0"]
    #[inline(always)]
    pub fn fovr(&mut self) -> FOVR_W {
        FOVR_W { w: self }
    }
    #[doc = "Bit 3 - FULL0"]
    #[inline(always)]
    pub fn full(&mut self) -> FULL_W {
        FULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "receive FIFO %s register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfr](index.html) module"]
pub struct RFR_SPEC;
impl crate::RegisterSpec for RFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfr::R](R) reader structure"]
impl crate::Readable for RFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfr::W](W) writer structure"]
impl crate::Writable for RFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF%sR to value 0"]
impl crate::Resettable for RFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
