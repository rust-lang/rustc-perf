#[doc = "Register `WPCR3` reader"]
pub struct R(crate::R<WPCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPCR3` writer"]
pub struct W(crate::W<WPCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR3_SPEC>;
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
impl From<crate::W<WPCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THSTRAIL` reader - tHSTRAIL"]
pub struct THSTRAIL_R(crate::FieldReader<u8, u8>);
impl THSTRAIL_R {
    pub(crate) fn new(bits: u8) -> Self {
        THSTRAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THSTRAIL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THSTRAIL` writer - tHSTRAIL"]
pub struct THSTRAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> THSTRAIL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `THSPREP` reader - tHS-PREPARE"]
pub struct THSPREP_R(crate::FieldReader<u8, u8>);
impl THSPREP_R {
    pub(crate) fn new(bits: u8) -> Self {
        THSPREP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THSPREP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THSPREP` writer - tHS-PREPARE"]
pub struct THSPREP_W<'a> {
    w: &'a mut W,
}
impl<'a> THSPREP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `TCLKZEO` reader - tCLK-ZERO"]
pub struct TCLKZEO_R(crate::FieldReader<u8, u8>);
impl TCLKZEO_R {
    pub(crate) fn new(bits: u8) -> Self {
        TCLKZEO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCLKZEO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCLKZEO` writer - tCLK-ZERO"]
pub struct TCLKZEO_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLKZEO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `TCLKPREP` reader - tCLK-PREPARE"]
pub struct TCLKPREP_R(crate::FieldReader<u8, u8>);
impl TCLKPREP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TCLKPREP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCLKPREP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCLKPREP` writer - tCLK-PREPARE"]
pub struct TCLKPREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLKPREP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - tHSTRAIL"]
    #[inline(always)]
    pub fn thstrail(&self) -> THSTRAIL_R {
        THSTRAIL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - tHS-PREPARE"]
    #[inline(always)]
    pub fn thsprep(&self) -> THSPREP_R {
        THSPREP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - tCLK-ZERO"]
    #[inline(always)]
    pub fn tclkzeo(&self) -> TCLKZEO_R {
        TCLKZEO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - tCLK-PREPARE"]
    #[inline(always)]
    pub fn tclkprep(&self) -> TCLKPREP_R {
        TCLKPREP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - tHSTRAIL"]
    #[inline(always)]
    pub fn thstrail(&mut self) -> THSTRAIL_W {
        THSTRAIL_W { w: self }
    }
    #[doc = "Bits 16:23 - tHS-PREPARE"]
    #[inline(always)]
    pub fn thsprep(&mut self) -> THSPREP_W {
        THSPREP_W { w: self }
    }
    #[doc = "Bits 8:15 - tCLK-ZERO"]
    #[inline(always)]
    pub fn tclkzeo(&mut self) -> TCLKZEO_W {
        TCLKZEO_W { w: self }
    }
    #[doc = "Bits 0:7 - tCLK-PREPARE"]
    #[inline(always)]
    pub fn tclkprep(&mut self) -> TCLKPREP_W {
        TCLKPREP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Wrapper PHY Configuration Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr3](index.html) module"]
pub struct WPCR3_SPEC;
impl crate::RegisterSpec for WPCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpcr3::R](R) reader structure"]
impl crate::Readable for WPCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpcr3::W](W) writer structure"]
impl crate::Writable for WPCR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WPCR3 to value 0"]
impl crate::Resettable for WPCR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
