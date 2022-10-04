#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINE_IE` reader - Line interrupt enable"]
pub struct LINE_IE_R(crate::FieldReader<bool, bool>);
impl LINE_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINE_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINE_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINE_IE` writer - Line interrupt enable"]
pub struct LINE_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_IE_W<'a> {
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
#[doc = "Field `VSYNC_IE` reader - VSYNC interrupt enable"]
pub struct VSYNC_IE_R(crate::FieldReader<bool, bool>);
impl VSYNC_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VSYNC_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSYNC_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSYNC_IE` writer - VSYNC interrupt enable"]
pub struct VSYNC_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_IE_W<'a> {
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
#[doc = "Field `ERR_IE` reader - Synchronization error interrupt enable"]
pub struct ERR_IE_R(crate::FieldReader<bool, bool>);
impl ERR_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERR_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_IE` writer - Synchronization error interrupt enable"]
pub struct ERR_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_IE_W<'a> {
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
#[doc = "Field `OVR_IE` reader - Overrun interrupt enable"]
pub struct OVR_IE_R(crate::FieldReader<bool, bool>);
impl OVR_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVR_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_IE` writer - Overrun interrupt enable"]
pub struct OVR_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_IE_W<'a> {
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
#[doc = "Field `FRAME_IE` reader - Capture complete interrupt enable"]
pub struct FRAME_IE_R(crate::FieldReader<bool, bool>);
impl FRAME_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRAME_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAME_IE` writer - Capture complete interrupt enable"]
pub struct FRAME_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_IE_W<'a> {
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
    #[doc = "Bit 4 - Line interrupt enable"]
    #[inline(always)]
    pub fn line_ie(&self) -> LINE_IE_R {
        LINE_IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VSYNC interrupt enable"]
    #[inline(always)]
    pub fn vsync_ie(&self) -> VSYNC_IE_R {
        VSYNC_IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Synchronization error interrupt enable"]
    #[inline(always)]
    pub fn err_ie(&self) -> ERR_IE_R {
        ERR_IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overrun interrupt enable"]
    #[inline(always)]
    pub fn ovr_ie(&self) -> OVR_IE_R {
        OVR_IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Capture complete interrupt enable"]
    #[inline(always)]
    pub fn frame_ie(&self) -> FRAME_IE_R {
        FRAME_IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Line interrupt enable"]
    #[inline(always)]
    pub fn line_ie(&mut self) -> LINE_IE_W {
        LINE_IE_W { w: self }
    }
    #[doc = "Bit 3 - VSYNC interrupt enable"]
    #[inline(always)]
    pub fn vsync_ie(&mut self) -> VSYNC_IE_W {
        VSYNC_IE_W { w: self }
    }
    #[doc = "Bit 2 - Synchronization error interrupt enable"]
    #[inline(always)]
    pub fn err_ie(&mut self) -> ERR_IE_W {
        ERR_IE_W { w: self }
    }
    #[doc = "Bit 1 - Overrun interrupt enable"]
    #[inline(always)]
    pub fn ovr_ie(&mut self) -> OVR_IE_W {
        OVR_IE_W { w: self }
    }
    #[doc = "Bit 0 - Capture complete interrupt enable"]
    #[inline(always)]
    pub fn frame_ie(&mut self) -> FRAME_IE_W {
        FRAME_IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
