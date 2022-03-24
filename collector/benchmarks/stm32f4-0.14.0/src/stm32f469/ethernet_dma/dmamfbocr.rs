#[doc = "Register `DMAMFBOCR` reader"]
pub struct R(crate::R<DMAMFBOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAMFBOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAMFBOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAMFBOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAMFBOCR` writer"]
pub struct W(crate::W<DMAMFBOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAMFBOCR_SPEC>;
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
impl From<crate::W<DMAMFBOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAMFBOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MFC` reader - Missed frames by the controller"]
pub struct MFC_R(crate::FieldReader<u16, u16>);
impl MFC_R {
    pub(crate) fn new(bits: u16) -> Self {
        MFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MFC` writer - Missed frames by the controller"]
pub struct MFC_W<'a> {
    w: &'a mut W,
}
impl<'a> MFC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `OMFC` reader - Overflow bit for missed frame counter"]
pub struct OMFC_R(crate::FieldReader<bool, bool>);
impl OMFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        OMFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OMFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OMFC` writer - Overflow bit for missed frame counter"]
pub struct OMFC_W<'a> {
    w: &'a mut W,
}
impl<'a> OMFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `MFA` reader - Missed frames by the application"]
pub struct MFA_R(crate::FieldReader<u16, u16>);
impl MFA_R {
    pub(crate) fn new(bits: u16) -> Self {
        MFA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MFA` writer - Missed frames by the application"]
pub struct MFA_W<'a> {
    w: &'a mut W,
}
impl<'a> MFA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 17)) | ((value as u32 & 0x07ff) << 17);
        self.w
    }
}
#[doc = "Field `OFOC` reader - Overflow bit for FIFO overflow counter"]
pub struct OFOC_R(crate::FieldReader<bool, bool>);
impl OFOC_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFOC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFOC` writer - Overflow bit for FIFO overflow counter"]
pub struct OFOC_W<'a> {
    w: &'a mut W,
}
impl<'a> OFOC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Missed frames by the controller"]
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overflow bit for missed frame counter"]
    #[inline(always)]
    pub fn omfc(&self) -> OMFC_R {
        OMFC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:27 - Missed frames by the application"]
    #[inline(always)]
    pub fn mfa(&self) -> MFA_R {
        MFA_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow bit for FIFO overflow counter"]
    #[inline(always)]
    pub fn ofoc(&self) -> OFOC_R {
        OFOC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Missed frames by the controller"]
    #[inline(always)]
    pub fn mfc(&mut self) -> MFC_W {
        MFC_W { w: self }
    }
    #[doc = "Bit 16 - Overflow bit for missed frame counter"]
    #[inline(always)]
    pub fn omfc(&mut self) -> OMFC_W {
        OMFC_W { w: self }
    }
    #[doc = "Bits 17:27 - Missed frames by the application"]
    #[inline(always)]
    pub fn mfa(&mut self) -> MFA_W {
        MFA_W { w: self }
    }
    #[doc = "Bit 28 - Overflow bit for FIFO overflow counter"]
    #[inline(always)]
    pub fn ofoc(&mut self) -> OFOC_W {
        OFOC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA missed frame and buffer overflow counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamfbocr](index.html) module"]
pub struct DMAMFBOCR_SPEC;
impl crate::RegisterSpec for DMAMFBOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmamfbocr::R](R) reader structure"]
impl crate::Readable for DMAMFBOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmamfbocr::W](W) writer structure"]
impl crate::Writable for DMAMFBOCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAMFBOCR to value 0"]
impl crate::Resettable for DMAMFBOCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
