#[doc = "Register `DMA2D_NLR` reader"]
pub struct R(crate::R<DMA2D_NLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA2D_NLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA2D_NLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA2D_NLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA2D_NLR` writer"]
pub struct W(crate::W<DMA2D_NLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA2D_NLR_SPEC>;
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
impl From<crate::W<DMA2D_NLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA2D_NLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NL` reader - Number of lines Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub struct NL_R(crate::FieldReader<u16, u16>);
impl NL_R {
    pub(crate) fn new(bits: u16) -> Self {
        NL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NL` writer - Number of lines Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub struct NL_W<'a> {
    w: &'a mut W,
}
impl<'a> NL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `PL` reader - Pixel per lines Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. If any of the input image format is 4-bit per pixel, pixel per lines must be even."]
pub struct PL_R(crate::FieldReader<u16, u16>);
impl PL_R {
    pub(crate) fn new(bits: u16) -> Self {
        PL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PL` writer - Pixel per lines Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. If any of the input image format is 4-bit per pixel, pixel per lines must be even."]
pub struct PL_W<'a> {
    w: &'a mut W,
}
impl<'a> PL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | ((value as u32 & 0x3fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Number of lines Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn nl(&self) -> NL_R {
        NL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:29 - Pixel per lines Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. If any of the input image format is 4-bit per pixel, pixel per lines must be even."]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of lines Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn nl(&mut self) -> NL_W {
        NL_W { w: self }
    }
    #[doc = "Bits 16:29 - Pixel per lines Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. If any of the input image format is 4-bit per pixel, pixel per lines must be even."]
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W {
        PL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA2D number of line register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_nlr](index.html) module"]
pub struct DMA2D_NLR_SPEC;
impl crate::RegisterSpec for DMA2D_NLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma2d_nlr::R](R) reader structure"]
impl crate::Readable for DMA2D_NLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma2d_nlr::W](W) writer structure"]
impl crate::Writable for DMA2D_NLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA2D_NLR to value 0"]
impl crate::Resettable for DMA2D_NLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
