#[doc = "Register `HCDMA5` reader"]
pub struct R(crate::R<HCDMA5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDMA5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDMA5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDMA5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCDMA5` writer"]
pub struct W(crate::W<HCDMA5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCDMA5_SPEC>;
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
impl From<crate::W<HCDMA5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCDMA5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAADDR` reader - DMA address"]
pub struct DMAADDR_R(crate::FieldReader<u32, u32>);
impl DMAADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        DMAADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAADDR` writer - DMA address"]
pub struct DMAADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DMA address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA address"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W {
        DMAADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS host channel-5 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma5](index.html) module"]
pub struct HCDMA5_SPEC;
impl crate::RegisterSpec for HCDMA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdma5::R](R) reader structure"]
impl crate::Readable for HCDMA5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcdma5::W](W) writer structure"]
impl crate::Writable for HCDMA5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCDMA5 to value 0"]
impl crate::Resettable for HCDMA5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
