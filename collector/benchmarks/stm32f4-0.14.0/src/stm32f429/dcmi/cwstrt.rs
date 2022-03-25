#[doc = "Register `CWSTRT` reader"]
pub struct R(crate::R<CWSTRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CWSTRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CWSTRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CWSTRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CWSTRT` writer"]
pub struct W(crate::W<CWSTRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CWSTRT_SPEC>;
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
impl From<crate::W<CWSTRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CWSTRT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VST` reader - Vertical start line count"]
pub struct VST_R(crate::FieldReader<u16, u16>);
impl VST_R {
    pub(crate) fn new(bits: u16) -> Self {
        VST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VST_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VST` writer - Vertical start line count"]
pub struct VST_W<'a> {
    w: &'a mut W,
}
impl<'a> VST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 16)) | ((value as u32 & 0x1fff) << 16);
        self.w
    }
}
#[doc = "Field `HOFFCNT` reader - Horizontal offset count"]
pub struct HOFFCNT_R(crate::FieldReader<u16, u16>);
impl HOFFCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        HOFFCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOFFCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOFFCNT` writer - Horizontal offset count"]
pub struct HOFFCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HOFFCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:28 - Vertical start line count"]
    #[inline(always)]
    pub fn vst(&self) -> VST_R {
        VST_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bits 0:13 - Horizontal offset count"]
    #[inline(always)]
    pub fn hoffcnt(&self) -> HOFFCNT_R {
        HOFFCNT_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:28 - Vertical start line count"]
    #[inline(always)]
    pub fn vst(&mut self) -> VST_W {
        VST_W { w: self }
    }
    #[doc = "Bits 0:13 - Horizontal offset count"]
    #[inline(always)]
    pub fn hoffcnt(&mut self) -> HOFFCNT_W {
        HOFFCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "crop window start\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwstrt](index.html) module"]
pub struct CWSTRT_SPEC;
impl crate::RegisterSpec for CWSTRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cwstrt::R](R) reader structure"]
impl crate::Readable for CWSTRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cwstrt::W](W) writer structure"]
impl crate::Writable for CWSTRT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CWSTRT to value 0"]
impl crate::Resettable for CWSTRT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
