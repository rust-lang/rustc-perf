#[doc = "Register `SDRTR` reader"]
pub struct R(crate::R<SDRTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDRTR` writer"]
pub struct W(crate::W<SDRTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDRTR_SPEC>;
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
impl From<crate::W<SDRTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDRTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear Refresh error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRE_AW {
    #[doc = "1: Refresh Error Flag is cleared"]
    CLEAR = 1,
}
impl From<CRE_AW> for bool {
    #[inline(always)]
    fn from(variant: CRE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRE` writer - Clear Refresh error flag"]
pub struct CRE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRE_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Refresh Error Flag is cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CRE_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `COUNT` reader - Refresh Timer Count"]
pub struct COUNT_R(crate::FieldReader<u16, u16>);
impl COUNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNT` writer - Refresh Timer Count"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 1)) | ((value as u32 & 0x1fff) << 1);
        self.w
    }
}
#[doc = "RES Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REIE_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is generated if RE = 1"]
    ENABLED = 1,
}
impl From<REIE_A> for bool {
    #[inline(always)]
    fn from(variant: REIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REIE` reader - RES Interrupt Enable"]
pub struct REIE_R(crate::FieldReader<bool, REIE_A>);
impl REIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REIE_A {
        match self.bits {
            false => REIE_A::DISABLED,
            true => REIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REIE_A::ENABLED
    }
}
impl core::ops::Deref for REIE_R {
    type Target = crate::FieldReader<bool, REIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REIE` writer - RES Interrupt Enable"]
pub struct REIE_W<'a> {
    w: &'a mut W,
}
impl<'a> REIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REIE_A::DISABLED)
    }
    #[doc = "Interrupt is generated if RE = 1"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:13 - Refresh Timer Count"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 1) & 0x1fff) as u16)
    }
    #[doc = "Bit 14 - RES Interrupt Enable"]
    #[inline(always)]
    pub fn reie(&self) -> REIE_R {
        REIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Refresh error flag"]
    #[inline(always)]
    pub fn cre(&mut self) -> CRE_W {
        CRE_W { w: self }
    }
    #[doc = "Bits 1:13 - Refresh Timer Count"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
    #[doc = "Bit 14 - RES Interrupt Enable"]
    #[inline(always)]
    pub fn reie(&mut self) -> REIE_W {
        REIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Refresh Timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdrtr](index.html) module"]
pub struct SDRTR_SPEC;
impl crate::RegisterSpec for SDRTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdrtr::R](R) reader structure"]
impl crate::Readable for SDRTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdrtr::W](W) writer structure"]
impl crate::Writable for SDRTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDRTR to value 0"]
impl crate::Resettable for SDRTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
