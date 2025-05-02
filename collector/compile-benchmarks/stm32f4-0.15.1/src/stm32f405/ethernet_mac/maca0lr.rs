#[doc = "Register `MACA0LR` reader"]
pub struct R(crate::R<MACA0LR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACA0LR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACA0LR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACA0LR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACA0LR` writer"]
pub struct W(crate::W<MACA0LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACA0LR_SPEC>;
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
impl From<crate::W<MACA0LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACA0LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MACA0L` reader - 0"]
pub type MACA0L_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MACA0L` writer - 0"]
pub type MACA0L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACA0LR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 0"]
    #[inline(always)]
    pub fn maca0l(&self) -> MACA0L_R {
        MACA0L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 0"]
    #[inline(always)]
    pub fn maca0l(&mut self) -> MACA0L_W<0> {
        MACA0L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC address 0 low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca0lr](index.html) module"]
pub struct MACA0LR_SPEC;
impl crate::RegisterSpec for MACA0LR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maca0lr::R](R) reader structure"]
impl crate::Readable for MACA0LR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maca0lr::W](W) writer structure"]
impl crate::Writable for MACA0LR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACA0LR to value 0xffff_ffff"]
impl crate::Resettable for MACA0LR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
