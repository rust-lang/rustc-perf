#[doc = "Register `SSCR` reader"]
pub struct R(crate::R<SSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSCR` writer"]
pub struct W(crate::W<SSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSCR_SPEC>;
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
impl From<crate::W<SSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSW` reader - Horizontal Synchronization Width (in units of pixel clock period)"]
pub struct HSW_R(crate::FieldReader<u16, u16>);
impl HSW_R {
    pub(crate) fn new(bits: u16) -> Self {
        HSW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSW` writer - Horizontal Synchronization Width (in units of pixel clock period)"]
pub struct HSW_W<'a> {
    w: &'a mut W,
}
impl<'a> HSW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `VSH` reader - Vertical Synchronization Height (in units of horizontal scan line)"]
pub struct VSH_R(crate::FieldReader<u16, u16>);
impl VSH_R {
    pub(crate) fn new(bits: u16) -> Self {
        VSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSH` writer - Vertical Synchronization Height (in units of horizontal scan line)"]
pub struct VSH_W<'a> {
    w: &'a mut W,
}
impl<'a> VSH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27 - Horizontal Synchronization Width (in units of pixel clock period)"]
    #[inline(always)]
    pub fn hsw(&self) -> HSW_R {
        HSW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:10 - Vertical Synchronization Height (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn vsh(&self) -> VSH_R {
        VSH_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - Horizontal Synchronization Width (in units of pixel clock period)"]
    #[inline(always)]
    pub fn hsw(&mut self) -> HSW_W {
        HSW_W { w: self }
    }
    #[doc = "Bits 0:10 - Vertical Synchronization Height (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn vsh(&mut self) -> VSH_W {
        VSH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronization Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sscr](index.html) module"]
pub struct SSCR_SPEC;
impl crate::RegisterSpec for SSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sscr::R](R) reader structure"]
impl crate::Readable for SSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sscr::W](W) writer structure"]
impl crate::Writable for SSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSCR to value 0"]
impl crate::Resettable for SSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
