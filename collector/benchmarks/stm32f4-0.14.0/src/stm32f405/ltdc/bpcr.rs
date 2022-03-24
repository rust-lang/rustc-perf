#[doc = "Register `BPCR` reader"]
pub struct R(crate::R<BPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPCR` writer"]
pub struct W(crate::W<BPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPCR_SPEC>;
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
impl From<crate::W<BPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHBP` reader - Accumulated Horizontal back porch (in units of pixel clock period)"]
pub struct AHBP_R(crate::FieldReader<u16, u16>);
impl AHBP_R {
    pub(crate) fn new(bits: u16) -> Self {
        AHBP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBP` writer - Accumulated Horizontal back porch (in units of pixel clock period)"]
pub struct AHBP_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `AVBP` reader - Accumulated Vertical back porch (in units of horizontal scan line)"]
pub struct AVBP_R(crate::FieldReader<u16, u16>);
impl AVBP_R {
    pub(crate) fn new(bits: u16) -> Self {
        AVBP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVBP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVBP` writer - Accumulated Vertical back porch (in units of horizontal scan line)"]
pub struct AVBP_W<'a> {
    w: &'a mut W,
}
impl<'a> AVBP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27 - Accumulated Horizontal back porch (in units of pixel clock period)"]
    #[inline(always)]
    pub fn ahbp(&self) -> AHBP_R {
        AHBP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:10 - Accumulated Vertical back porch (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn avbp(&self) -> AVBP_R {
        AVBP_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - Accumulated Horizontal back porch (in units of pixel clock period)"]
    #[inline(always)]
    pub fn ahbp(&mut self) -> AHBP_W {
        AHBP_W { w: self }
    }
    #[doc = "Bits 0:10 - Accumulated Vertical back porch (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn avbp(&mut self) -> AVBP_W {
        AVBP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Back Porch Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpcr](index.html) module"]
pub struct BPCR_SPEC;
impl crate::RegisterSpec for BPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpcr::R](R) reader structure"]
impl crate::Readable for BPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpcr::W](W) writer structure"]
impl crate::Writable for BPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BPCR to value 0"]
impl crate::Resettable for BPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
