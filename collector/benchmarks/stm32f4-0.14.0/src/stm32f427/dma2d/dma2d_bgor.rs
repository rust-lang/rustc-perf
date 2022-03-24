#[doc = "Register `DMA2D_BGOR` reader"]
pub struct R(crate::R<DMA2D_BGOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA2D_BGOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA2D_BGOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA2D_BGOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA2D_BGOR` writer"]
pub struct W(crate::W<DMA2D_BGOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA2D_BGOR_SPEC>;
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
impl From<crate::W<DMA2D_BGOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA2D_BGOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LO` reader - Line offset Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
pub struct LO_R(crate::FieldReader<u16, u16>);
impl LO_R {
    pub(crate) fn new(bits: u16) -> Self {
        LO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LO` writer - Line offset Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
pub struct LO_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Line offset Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Line offset Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
    #[inline(always)]
    pub fn lo(&mut self) -> LO_W {
        LO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA2D background offset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_bgor](index.html) module"]
pub struct DMA2D_BGOR_SPEC;
impl crate::RegisterSpec for DMA2D_BGOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma2d_bgor::R](R) reader structure"]
impl crate::Readable for DMA2D_BGOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma2d_bgor::W](W) writer structure"]
impl crate::Writable for DMA2D_BGOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA2D_BGOR to value 0"]
impl crate::Resettable for DMA2D_BGOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
