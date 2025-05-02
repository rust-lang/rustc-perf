#[doc = "Register `BGMAR` reader"]
pub struct R(crate::R<BGMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGMAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGMAR` writer"]
pub struct W(crate::W<BGMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGMAR_SPEC>;
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
impl From<crate::W<BGMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGMAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MA` reader - Memory address Address of the data used for the background image. This register can only be written when data transfers are disabled. Once a data transfer has started, this register is read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned."]
pub type MA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MA` writer - Memory address Address of the data used for the background image. This register can only be written when data transfers are disabled. Once a data transfer has started, this register is read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned."]
pub type MA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGMAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Memory address Address of the data used for the background image. This register can only be written when data transfers are disabled. Once a data transfer has started, this register is read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned."]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address Address of the data used for the background image. This register can only be written when data transfers are disabled. Once a data transfer has started, this register is read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned."]
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W<0> {
        MA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA2D background memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgmar](index.html) module"]
pub struct BGMAR_SPEC;
impl crate::RegisterSpec for BGMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgmar::R](R) reader structure"]
impl crate::Readable for BGMAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgmar::W](W) writer structure"]
impl crate::Writable for BGMAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BGMAR to value 0"]
impl crate::Resettable for BGMAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
