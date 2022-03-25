#[doc = "Register `BGPFCCR` reader"]
pub struct R(crate::R<BGPFCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGPFCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGPFCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGPFCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGPFCCR` writer"]
pub struct W(crate::W<BGPFCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGPFCCR_SPEC>;
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
impl From<crate::W<BGPFCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGPFCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALPHA` reader - Alpha value"]
pub struct ALPHA_R(crate::FieldReader<u8, u8>);
impl ALPHA_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALPHA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALPHA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALPHA` writer - Alpha value"]
pub struct ALPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> ALPHA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `AM` reader - Alpha mode"]
pub struct AM_R(crate::FieldReader<u8, u8>);
impl AM_R {
    pub(crate) fn new(bits: u8) -> Self {
        AM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AM` writer - Alpha mode"]
pub struct AM_W<'a> {
    w: &'a mut W,
}
impl<'a> AM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `CS` reader - CLUT size"]
pub struct CS_R(crate::FieldReader<u8, u8>);
impl CS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS` writer - CLUT size"]
pub struct CS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `START` reader - Start"]
pub struct START_R(crate::FieldReader<bool, bool>);
impl START_R {
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - Start"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Field `CCM` reader - CLUT Color mode"]
pub struct CCM_R(crate::FieldReader<bool, bool>);
impl CCM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCM` writer - CLUT Color mode"]
pub struct CCM_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_W<'a> {
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
#[doc = "Field `CM` reader - Color mode"]
pub struct CM_R(crate::FieldReader<u8, u8>);
impl CM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM` writer - Color mode"]
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Alpha value"]
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Alpha mode"]
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - CLUT size"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 5 - Start"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CLUT Color mode"]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - Color mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Alpha value"]
    #[inline(always)]
    pub fn alpha(&mut self) -> ALPHA_W {
        ALPHA_W { w: self }
    }
    #[doc = "Bits 16:17 - Alpha mode"]
    #[inline(always)]
    pub fn am(&mut self) -> AM_W {
        AM_W { w: self }
    }
    #[doc = "Bits 8:15 - CLUT size"]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W {
        CS_W { w: self }
    }
    #[doc = "Bit 5 - Start"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 4 - CLUT Color mode"]
    #[inline(always)]
    pub fn ccm(&mut self) -> CCM_W {
        CCM_W { w: self }
    }
    #[doc = "Bits 0:3 - Color mode"]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "background PFC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgpfccr](index.html) module"]
pub struct BGPFCCR_SPEC;
impl crate::RegisterSpec for BGPFCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgpfccr::R](R) reader structure"]
impl crate::Readable for BGPFCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgpfccr::W](W) writer structure"]
impl crate::Writable for BGPFCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BGPFCCR to value 0"]
impl crate::Resettable for BGPFCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
