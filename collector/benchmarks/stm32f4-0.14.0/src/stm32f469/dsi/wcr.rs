#[doc = "Register `WCR` reader"]
pub struct R(crate::R<WCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WCR` writer"]
pub struct W(crate::W<WCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WCR_SPEC>;
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
impl From<crate::W<WCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSIEN` reader - DSI Enable"]
pub struct DSIEN_R(crate::FieldReader<bool, bool>);
impl DSIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSIEN` writer - DSI Enable"]
pub struct DSIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIEN_W<'a> {
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
#[doc = "Field `LTDCEN` reader - LTDC Enable"]
pub struct LTDCEN_R(crate::FieldReader<bool, bool>);
impl LTDCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LTDCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LTDCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LTDCEN` writer - LTDC Enable"]
pub struct LTDCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LTDCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SHTDN` reader - Shutdown"]
pub struct SHTDN_R(crate::FieldReader<bool, bool>);
impl SHTDN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHTDN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHTDN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHTDN` writer - Shutdown"]
pub struct SHTDN_W<'a> {
    w: &'a mut W,
}
impl<'a> SHTDN_W<'a> {
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
#[doc = "Field `COLM` reader - Color Mode"]
pub struct COLM_R(crate::FieldReader<bool, bool>);
impl COLM_R {
    pub(crate) fn new(bits: bool) -> Self {
        COLM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLM` writer - Color Mode"]
pub struct COLM_W<'a> {
    w: &'a mut W,
}
impl<'a> COLM_W<'a> {
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
    #[doc = "Bit 3 - DSI Enable"]
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LTDC Enable"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shutdown"]
    #[inline(always)]
    pub fn shtdn(&self) -> SHTDN_R {
        SHTDN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Color Mode"]
    #[inline(always)]
    pub fn colm(&self) -> COLM_R {
        COLM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - DSI Enable"]
    #[inline(always)]
    pub fn dsien(&mut self) -> DSIEN_W {
        DSIEN_W { w: self }
    }
    #[doc = "Bit 2 - LTDC Enable"]
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W {
        LTDCEN_W { w: self }
    }
    #[doc = "Bit 1 - Shutdown"]
    #[inline(always)]
    pub fn shtdn(&mut self) -> SHTDN_W {
        SHTDN_W { w: self }
    }
    #[doc = "Bit 0 - Color Mode"]
    #[inline(always)]
    pub fn colm(&mut self) -> COLM_W {
        COLM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Wrapper Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcr](index.html) module"]
pub struct WCR_SPEC;
impl crate::RegisterSpec for WCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wcr::R](R) reader structure"]
impl crate::Readable for WCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wcr::W](W) writer structure"]
impl crate::Writable for WCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WCR to value 0"]
impl crate::Resettable for WCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
