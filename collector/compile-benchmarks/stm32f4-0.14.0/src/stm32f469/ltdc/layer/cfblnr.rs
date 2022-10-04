#[doc = "Register `CFBLNR` reader"]
pub struct R(crate::R<CFBLNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFBLNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFBLNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFBLNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFBLNR` writer"]
pub struct W(crate::W<CFBLNR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFBLNR_SPEC>;
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
impl From<crate::W<CFBLNR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFBLNR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFBLNBR` reader - Frame Buffer Line Number"]
pub struct CFBLNBR_R(crate::FieldReader<u16, u16>);
impl CFBLNBR_R {
    pub(crate) fn new(bits: u16) -> Self {
        CFBLNBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFBLNBR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFBLNBR` writer - Frame Buffer Line Number"]
pub struct CFBLNBR_W<'a> {
    w: &'a mut W,
}
impl<'a> CFBLNBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Frame Buffer Line Number"]
    #[inline(always)]
    pub fn cfblnbr(&self) -> CFBLNBR_R {
        CFBLNBR_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Frame Buffer Line Number"]
    #[inline(always)]
    pub fn cfblnbr(&mut self) -> CFBLNBR_W {
        CFBLNBR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layerx ColorFrame Buffer Line Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfblnr](index.html) module"]
pub struct CFBLNR_SPEC;
impl crate::RegisterSpec for CFBLNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfblnr::R](R) reader structure"]
impl crate::Readable for CFBLNR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfblnr::W](W) writer structure"]
impl crate::Writable for CFBLNR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFBLNR to value 0"]
impl crate::Resettable for CFBLNR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
