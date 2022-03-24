#[doc = "Register `FCR` reader"]
pub struct R(crate::R<FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCR` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTOF` reader - Clear timeout flag"]
pub struct CTOF_R(crate::FieldReader<bool, bool>);
impl CTOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTOF` writer - Clear timeout flag"]
pub struct CTOF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTOF_W<'a> {
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
#[doc = "Field `CSMF` reader - Clear status match flag"]
pub struct CSMF_R(crate::FieldReader<bool, bool>);
impl CSMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSMF` writer - Clear status match flag"]
pub struct CSMF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSMF_W<'a> {
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
#[doc = "Field `CTCF` reader - Clear transfer complete flag"]
pub struct CTCF_R(crate::FieldReader<bool, bool>);
impl CTCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCF` writer - Clear transfer complete flag"]
pub struct CTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCF_W<'a> {
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
#[doc = "Field `CTEF` reader - Clear transfer error flag"]
pub struct CTEF_R(crate::FieldReader<bool, bool>);
impl CTEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTEF` writer - Clear transfer error flag"]
pub struct CTEF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEF_W<'a> {
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
    #[doc = "Bit 4 - Clear timeout flag"]
    #[inline(always)]
    pub fn ctof(&self) -> CTOF_R {
        CTOF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear status match flag"]
    #[inline(always)]
    pub fn csmf(&self) -> CSMF_R {
        CSMF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clear transfer complete flag"]
    #[inline(always)]
    pub fn ctcf(&self) -> CTCF_R {
        CTCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Clear transfer error flag"]
    #[inline(always)]
    pub fn ctef(&self) -> CTEF_R {
        CTEF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Clear timeout flag"]
    #[inline(always)]
    pub fn ctof(&mut self) -> CTOF_W {
        CTOF_W { w: self }
    }
    #[doc = "Bit 3 - Clear status match flag"]
    #[inline(always)]
    pub fn csmf(&mut self) -> CSMF_W {
        CSMF_W { w: self }
    }
    #[doc = "Bit 1 - Clear transfer complete flag"]
    #[inline(always)]
    pub fn ctcf(&mut self) -> CTCF_W {
        CTCF_W { w: self }
    }
    #[doc = "Bit 0 - Clear transfer error flag"]
    #[inline(always)]
    pub fn ctef(&mut self) -> CTEF_W {
        CTEF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcr::R](R) reader structure"]
impl crate::Readable for FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
