#[doc = "Register `GHCR` reader"]
pub struct R(crate::R<GHCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GHCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GHCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GHCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GHCR` writer"]
pub struct W(crate::W<GHCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GHCR_SPEC>;
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
impl From<crate::W<GHCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GHCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WCMSB` reader - WordCount MSB"]
pub struct WCMSB_R(crate::FieldReader<u8, u8>);
impl WCMSB_R {
    pub(crate) fn new(bits: u8) -> Self {
        WCMSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WCMSB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WCMSB` writer - WordCount MSB"]
pub struct WCMSB_W<'a> {
    w: &'a mut W,
}
impl<'a> WCMSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `WCLSB` reader - WordCount LSB"]
pub struct WCLSB_R(crate::FieldReader<u8, u8>);
impl WCLSB_R {
    pub(crate) fn new(bits: u8) -> Self {
        WCLSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WCLSB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WCLSB` writer - WordCount LSB"]
pub struct WCLSB_W<'a> {
    w: &'a mut W,
}
impl<'a> WCLSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `VCID` reader - Channel"]
pub struct VCID_R(crate::FieldReader<u8, u8>);
impl VCID_R {
    pub(crate) fn new(bits: u8) -> Self {
        VCID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCID` writer - Channel"]
pub struct VCID_W<'a> {
    w: &'a mut W,
}
impl<'a> VCID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `DT` reader - Type"]
pub struct DT_R(crate::FieldReader<u8, u8>);
impl DT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT` writer - Type"]
pub struct DT_W<'a> {
    w: &'a mut W,
}
impl<'a> DT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - WordCount MSB"]
    #[inline(always)]
    pub fn wcmsb(&self) -> WCMSB_R {
        WCMSB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - WordCount LSB"]
    #[inline(always)]
    pub fn wclsb(&self) -> WCLSB_R {
        WCLSB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 6:7 - Channel"]
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 0:5 - Type"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - WordCount MSB"]
    #[inline(always)]
    pub fn wcmsb(&mut self) -> WCMSB_W {
        WCMSB_W { w: self }
    }
    #[doc = "Bits 8:15 - WordCount LSB"]
    #[inline(always)]
    pub fn wclsb(&mut self) -> WCLSB_W {
        WCLSB_W { w: self }
    }
    #[doc = "Bits 6:7 - Channel"]
    #[inline(always)]
    pub fn vcid(&mut self) -> VCID_W {
        VCID_W { w: self }
    }
    #[doc = "Bits 0:5 - Type"]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W {
        DT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Generic Header Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ghcr](index.html) module"]
pub struct GHCR_SPEC;
impl crate::RegisterSpec for GHCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ghcr::R](R) reader structure"]
impl crate::Readable for GHCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ghcr::W](W) writer structure"]
impl crate::Writable for GHCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GHCR to value 0"]
impl crate::Resettable for GHCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
