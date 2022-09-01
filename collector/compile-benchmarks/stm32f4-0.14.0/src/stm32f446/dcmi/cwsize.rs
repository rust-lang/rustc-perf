#[doc = "Register `CWSIZE` reader"]
pub struct R(crate::R<CWSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CWSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CWSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CWSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CWSIZE` writer"]
pub struct W(crate::W<CWSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CWSIZE_SPEC>;
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
impl From<crate::W<CWSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CWSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLINE` reader - Vertical line count"]
pub struct VLINE_R(crate::FieldReader<u16, u16>);
impl VLINE_R {
    pub(crate) fn new(bits: u16) -> Self {
        VLINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLINE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLINE` writer - Vertical line count"]
pub struct VLINE_W<'a> {
    w: &'a mut W,
}
impl<'a> VLINE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | ((value as u32 & 0x3fff) << 16);
        self.w
    }
}
#[doc = "Field `CAPCNT` reader - Capture count"]
pub struct CAPCNT_R(crate::FieldReader<u16, u16>);
impl CAPCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        CAPCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPCNT` writer - Capture count"]
pub struct CAPCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:29 - Vertical line count"]
    #[inline(always)]
    pub fn vline(&self) -> VLINE_R {
        VLINE_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bits 0:13 - Capture count"]
    #[inline(always)]
    pub fn capcnt(&self) -> CAPCNT_R {
        CAPCNT_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:29 - Vertical line count"]
    #[inline(always)]
    pub fn vline(&mut self) -> VLINE_W {
        VLINE_W { w: self }
    }
    #[doc = "Bits 0:13 - Capture count"]
    #[inline(always)]
    pub fn capcnt(&mut self) -> CAPCNT_W {
        CAPCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "crop window size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwsize](index.html) module"]
pub struct CWSIZE_SPEC;
impl crate::RegisterSpec for CWSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cwsize::R](R) reader structure"]
impl crate::Readable for CWSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cwsize::W](W) writer structure"]
impl crate::Writable for CWSIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CWSIZE to value 0"]
impl crate::Resettable for CWSIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
