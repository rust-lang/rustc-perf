#[doc = "Register `AWCR` reader"]
pub struct R(crate::R<AWCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AWCR` writer"]
pub struct W(crate::W<AWCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWCR_SPEC>;
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
impl From<crate::W<AWCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AAW` reader - Accumulated Active Width (in units of pixel clock period)"]
pub struct AAW_R(crate::FieldReader<u16, u16>);
impl AAW_R {
    pub(crate) fn new(bits: u16) -> Self {
        AAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AAW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AAW` writer - Accumulated Active Width (in units of pixel clock period)"]
pub struct AAW_W<'a> {
    w: &'a mut W,
}
impl<'a> AAW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `AAH` reader - Accumulated Active Height (in units of horizontal scan line)"]
pub struct AAH_R(crate::FieldReader<u16, u16>);
impl AAH_R {
    pub(crate) fn new(bits: u16) -> Self {
        AAH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AAH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AAH` writer - Accumulated Active Height (in units of horizontal scan line)"]
pub struct AAH_W<'a> {
    w: &'a mut W,
}
impl<'a> AAH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27 - Accumulated Active Width (in units of pixel clock period)"]
    #[inline(always)]
    pub fn aaw(&self) -> AAW_R {
        AAW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:10 - Accumulated Active Height (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn aah(&self) -> AAH_R {
        AAH_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - Accumulated Active Width (in units of pixel clock period)"]
    #[inline(always)]
    pub fn aaw(&mut self) -> AAW_W {
        AAW_W { w: self }
    }
    #[doc = "Bits 0:10 - Accumulated Active Height (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn aah(&mut self) -> AAH_W {
        AAH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Active Width Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awcr](index.html) module"]
pub struct AWCR_SPEC;
impl crate::RegisterSpec for AWCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [awcr::R](R) reader structure"]
impl crate::Readable for AWCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [awcr::W](W) writer structure"]
impl crate::Writable for AWCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AWCR to value 0"]
impl crate::Resettable for AWCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
