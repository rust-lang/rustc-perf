#[doc = "Register `WIFCR` reader"]
pub struct R(crate::R<WIFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIFCR` writer"]
pub struct W(crate::W<WIFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFCR_SPEC>;
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
impl From<crate::W<WIFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRRIF` reader - Clear Regulator Ready Interrupt Flag"]
pub struct CRRIF_R(crate::FieldReader<bool, bool>);
impl CRRIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRRIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRRIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRRIF` writer - Clear Regulator Ready Interrupt Flag"]
pub struct CRRIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CRRIF_W<'a> {
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
#[doc = "Field `CPLLUIF` reader - Clear PLL Unlock Interrupt Flag"]
pub struct CPLLUIF_R(crate::FieldReader<bool, bool>);
impl CPLLUIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPLLUIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPLLUIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPLLUIF` writer - Clear PLL Unlock Interrupt Flag"]
pub struct CPLLUIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CPLLUIF_W<'a> {
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
#[doc = "Field `CPLLLIF` reader - Clear PLL Lock Interrupt Flag"]
pub struct CPLLLIF_R(crate::FieldReader<bool, bool>);
impl CPLLLIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPLLLIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPLLLIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPLLLIF` writer - Clear PLL Lock Interrupt Flag"]
pub struct CPLLLIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CPLLLIF_W<'a> {
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
#[doc = "Field `CERIF` reader - Clear End of Refresh Interrupt Flag"]
pub struct CERIF_R(crate::FieldReader<bool, bool>);
impl CERIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CERIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CERIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CERIF` writer - Clear End of Refresh Interrupt Flag"]
pub struct CERIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CERIF_W<'a> {
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
#[doc = "Field `CTEIF` reader - Clear Tearing Effect Interrupt Flag"]
pub struct CTEIF_R(crate::FieldReader<bool, bool>);
impl CTEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTEIF` writer - Clear Tearing Effect Interrupt Flag"]
pub struct CTEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF_W<'a> {
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
    #[doc = "Bit 13 - Clear Regulator Ready Interrupt Flag"]
    #[inline(always)]
    pub fn crrif(&self) -> CRRIF_R {
        CRRIF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Clear PLL Unlock Interrupt Flag"]
    #[inline(always)]
    pub fn cplluif(&self) -> CPLLUIF_R {
        CPLLUIF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clear PLL Lock Interrupt Flag"]
    #[inline(always)]
    pub fn cplllif(&self) -> CPLLLIF_R {
        CPLLLIF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clear End of Refresh Interrupt Flag"]
    #[inline(always)]
    pub fn cerif(&self) -> CERIF_R {
        CERIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Clear Tearing Effect Interrupt Flag"]
    #[inline(always)]
    pub fn cteif(&self) -> CTEIF_R {
        CTEIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Clear Regulator Ready Interrupt Flag"]
    #[inline(always)]
    pub fn crrif(&mut self) -> CRRIF_W {
        CRRIF_W { w: self }
    }
    #[doc = "Bit 10 - Clear PLL Unlock Interrupt Flag"]
    #[inline(always)]
    pub fn cplluif(&mut self) -> CPLLUIF_W {
        CPLLUIF_W { w: self }
    }
    #[doc = "Bit 9 - Clear PLL Lock Interrupt Flag"]
    #[inline(always)]
    pub fn cplllif(&mut self) -> CPLLLIF_W {
        CPLLLIF_W { w: self }
    }
    #[doc = "Bit 1 - Clear End of Refresh Interrupt Flag"]
    #[inline(always)]
    pub fn cerif(&mut self) -> CERIF_W {
        CERIF_W { w: self }
    }
    #[doc = "Bit 0 - Clear Tearing Effect Interrupt Flag"]
    #[inline(always)]
    pub fn cteif(&mut self) -> CTEIF_W {
        CTEIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Wrapper Interrupt Flag Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifcr](index.html) module"]
pub struct WIFCR_SPEC;
impl crate::RegisterSpec for WIFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifcr::R](R) reader structure"]
impl crate::Readable for WIFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifcr::W](W) writer structure"]
impl crate::Writable for WIFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WIFCR to value 0"]
impl crate::Resettable for WIFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
