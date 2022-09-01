#[doc = "Register `PIO4` reader"]
pub struct R(crate::R<PIO4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIO4` writer"]
pub struct W(crate::W<PIO4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO4_SPEC>;
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
impl From<crate::W<PIO4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOHIZx` reader - IOHIZx"]
pub struct IOHIZX_R(crate::FieldReader<u8, u8>);
impl IOHIZX_R {
    pub(crate) fn new(bits: u8) -> Self {
        IOHIZX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOHIZX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOHIZx` writer - IOHIZx"]
pub struct IOHIZX_W<'a> {
    w: &'a mut W,
}
impl<'a> IOHIZX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `IOHOLDx` reader - IOHOLDx"]
pub struct IOHOLDX_R(crate::FieldReader<u8, u8>);
impl IOHOLDX_R {
    pub(crate) fn new(bits: u8) -> Self {
        IOHOLDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOHOLDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOHOLDx` writer - IOHOLDx"]
pub struct IOHOLDX_W<'a> {
    w: &'a mut W,
}
impl<'a> IOHOLDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `IOWAITx` reader - IOWAITx"]
pub struct IOWAITX_R(crate::FieldReader<u8, u8>);
impl IOWAITX_R {
    pub(crate) fn new(bits: u8) -> Self {
        IOWAITX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOWAITX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOWAITx` writer - IOWAITx"]
pub struct IOWAITX_W<'a> {
    w: &'a mut W,
}
impl<'a> IOWAITX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `IOSETx` reader - IOSETx"]
pub struct IOSETX_R(crate::FieldReader<u8, u8>);
impl IOSETX_R {
    pub(crate) fn new(bits: u8) -> Self {
        IOSETX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOSETX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOSETx` writer - IOSETx"]
pub struct IOSETX_W<'a> {
    w: &'a mut W,
}
impl<'a> IOSETX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - IOHIZx"]
    #[inline(always)]
    pub fn iohizx(&self) -> IOHIZX_R {
        IOHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - IOHOLDx"]
    #[inline(always)]
    pub fn ioholdx(&self) -> IOHOLDX_R {
        IOHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - IOWAITx"]
    #[inline(always)]
    pub fn iowaitx(&self) -> IOWAITX_R {
        IOWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - IOSETx"]
    #[inline(always)]
    pub fn iosetx(&self) -> IOSETX_R {
        IOSETX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - IOHIZx"]
    #[inline(always)]
    pub fn iohizx(&mut self) -> IOHIZX_W {
        IOHIZX_W { w: self }
    }
    #[doc = "Bits 16:23 - IOHOLDx"]
    #[inline(always)]
    pub fn ioholdx(&mut self) -> IOHOLDX_W {
        IOHOLDX_W { w: self }
    }
    #[doc = "Bits 8:15 - IOWAITx"]
    #[inline(always)]
    pub fn iowaitx(&mut self) -> IOWAITX_W {
        IOWAITX_W { w: self }
    }
    #[doc = "Bits 0:7 - IOSETx"]
    #[inline(always)]
    pub fn iosetx(&mut self) -> IOSETX_W {
        IOSETX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O space timing register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio4](index.html) module"]
pub struct PIO4_SPEC;
impl crate::RegisterSpec for PIO4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio4::R](R) reader structure"]
impl crate::Readable for PIO4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio4::W](W) writer structure"]
impl crate::Writable for PIO4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIO4 to value 0xfcfc_fcfc"]
impl crate::Resettable for PIO4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfcfc_fcfc
    }
}
