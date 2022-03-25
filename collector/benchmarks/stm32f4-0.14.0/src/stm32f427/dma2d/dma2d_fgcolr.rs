#[doc = "Register `DMA2D_FGCOLR` reader"]
pub struct R(crate::R<DMA2D_FGCOLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA2D_FGCOLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA2D_FGCOLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA2D_FGCOLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA2D_FGCOLR` writer"]
pub struct W(crate::W<DMA2D_FGCOLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA2D_FGCOLR_SPEC>;
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
impl From<crate::W<DMA2D_FGCOLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA2D_FGCOLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLUE` reader - Blue Value These bits defines the blue value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
pub struct BLUE_R(crate::FieldReader<u8, u8>);
impl BLUE_R {
    pub(crate) fn new(bits: u8) -> Self {
        BLUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLUE` writer - Blue Value These bits defines the blue value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
pub struct BLUE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `GREEN` reader - Green Value These bits defines the green value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
pub struct GREEN_R(crate::FieldReader<u8, u8>);
impl GREEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        GREEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GREEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GREEN` writer - Green Value These bits defines the green value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
pub struct GREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GREEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `RED` reader - Red Value These bits defines the red value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub struct RED_R(crate::FieldReader<u8, u8>);
impl RED_R {
    pub(crate) fn new(bits: u8) -> Self {
        RED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RED` writer - Red Value These bits defines the red value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub struct RED_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Blue Value These bits defines the blue value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Green Value These bits defines the green value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Red Value These bits defines the red value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Blue Value These bits defines the blue value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W {
        BLUE_W { w: self }
    }
    #[doc = "Bits 8:15 - Green Value These bits defines the green value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, They are read-only."]
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W {
        GREEN_W { w: self }
    }
    #[doc = "Bits 16:23 - Red Value These bits defines the red value for the A4 or A8 mode of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn red(&mut self) -> RED_W {
        RED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA2D foreground color register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_fgcolr](index.html) module"]
pub struct DMA2D_FGCOLR_SPEC;
impl crate::RegisterSpec for DMA2D_FGCOLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma2d_fgcolr::R](R) reader structure"]
impl crate::Readable for DMA2D_FGCOLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma2d_fgcolr::W](W) writer structure"]
impl crate::Writable for DMA2D_FGCOLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA2D_FGCOLR to value 0"]
impl crate::Resettable for DMA2D_FGCOLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
