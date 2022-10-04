#[doc = "Register `WIER` reader"]
pub struct R(crate::R<WIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIER` writer"]
pub struct W(crate::W<WIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIER_SPEC>;
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
impl From<crate::W<WIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RRIE` reader - Regulator Ready Interrupt Enable"]
pub struct RRIE_R(crate::FieldReader<bool, bool>);
impl RRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRIE` writer - Regulator Ready Interrupt Enable"]
pub struct RRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RRIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `PLLUIE` reader - PLL Unlock Interrupt Enable"]
pub struct PLLUIE_R(crate::FieldReader<bool, bool>);
impl PLLUIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLUIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLUIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLUIE` writer - PLL Unlock Interrupt Enable"]
pub struct PLLUIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLUIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `PLLLIE` reader - PLL Lock Interrupt Enable"]
pub struct PLLLIE_R(crate::FieldReader<bool, bool>);
impl PLLLIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLLIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLLIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLLIE` writer - PLL Lock Interrupt Enable"]
pub struct PLLLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLLIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `ERIE` reader - End of Refresh Interrupt Enable"]
pub struct ERIE_R(crate::FieldReader<bool, bool>);
impl ERIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERIE` writer - End of Refresh Interrupt Enable"]
pub struct ERIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TEIE` reader - Tearing Effect Interrupt Enable"]
pub struct TEIE_R(crate::FieldReader<bool, bool>);
impl TEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIE` writer - Tearing Effect Interrupt Enable"]
pub struct TEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE_W<'a> {
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
impl R {
    #[doc = "Bit 13 - Regulator Ready Interrupt Enable"]
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PLL Unlock Interrupt Enable"]
    #[inline(always)]
    pub fn plluie(&self) -> PLLUIE_R {
        PLLUIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PLL Lock Interrupt Enable"]
    #[inline(always)]
    pub fn plllie(&self) -> PLLLIE_R {
        PLLLIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of Refresh Interrupt Enable"]
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Tearing Effect Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Regulator Ready Interrupt Enable"]
    #[inline(always)]
    pub fn rrie(&mut self) -> RRIE_W {
        RRIE_W { w: self }
    }
    #[doc = "Bit 10 - PLL Unlock Interrupt Enable"]
    #[inline(always)]
    pub fn plluie(&mut self) -> PLLUIE_W {
        PLLUIE_W { w: self }
    }
    #[doc = "Bit 9 - PLL Lock Interrupt Enable"]
    #[inline(always)]
    pub fn plllie(&mut self) -> PLLLIE_W {
        PLLLIE_W { w: self }
    }
    #[doc = "Bit 1 - End of Refresh Interrupt Enable"]
    #[inline(always)]
    pub fn erie(&mut self) -> ERIE_W {
        ERIE_W { w: self }
    }
    #[doc = "Bit 0 - Tearing Effect Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W {
        TEIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Wrapper Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wier](index.html) module"]
pub struct WIER_SPEC;
impl crate::RegisterSpec for WIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wier::R](R) reader structure"]
impl crate::Readable for WIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wier::W](W) writer structure"]
impl crate::Writable for WIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WIER to value 0"]
impl crate::Resettable for WIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
