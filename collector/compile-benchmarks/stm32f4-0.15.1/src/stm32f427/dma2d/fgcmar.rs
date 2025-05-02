#[doc = "Register `FGCMAR` reader"]
pub struct R(crate::R<FGCMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGCMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGCMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGCMAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGCMAR` writer"]
pub struct W(crate::W<FGCMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGCMAR_SPEC>;
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
impl From<crate::W<FGCMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGCMAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MA` reader - Memory Address Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned."]
pub type MA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MA` writer - Memory Address Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned."]
pub type MA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGCMAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Memory Address Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned."]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory Address Address of the data used for the CLUT address dedicated to the foreground image. This register can only be written when no transfer is ongoing. Once the CLUT transfer has started, this register is read-only. If the foreground CLUT format is 32-bit, the address must be 32-bit aligned."]
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
#[doc = "DMA2D foreground CLUT memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgcmar](index.html) module"]
pub struct FGCMAR_SPEC;
impl crate::RegisterSpec for FGCMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgcmar::R](R) reader structure"]
impl crate::Readable for FGCMAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fgcmar::W](W) writer structure"]
impl crate::Writable for FGCMAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FGCMAR to value 0"]
impl crate::Resettable for FGCMAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
