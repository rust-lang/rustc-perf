#[doc = "Register `DMARPDR` reader"]
pub struct R(crate::R<DMARPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMARPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMARPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMARPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMARPDR` writer"]
pub struct W(crate::W<DMARPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMARPDR_SPEC>;
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
impl From<crate::W<DMARPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMARPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPD` reader - RPD"]
pub struct RPD_R(crate::FieldReader<u32, u32>);
impl RPD_R {
    pub(crate) fn new(bits: u32) -> Self {
        RPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPD` writer - RPD"]
pub struct RPD_W<'a> {
    w: &'a mut W,
}
impl<'a> RPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - RPD"]
    #[inline(always)]
    pub fn rpd(&self) -> RPD_R {
        RPD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - RPD"]
    #[inline(always)]
    pub fn rpd(&mut self) -> RPD_W {
        RPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EHERNET DMA receive poll demand register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmarpdr](index.html) module"]
pub struct DMARPDR_SPEC;
impl crate::RegisterSpec for DMARPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmarpdr::R](R) reader structure"]
impl crate::Readable for DMARPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmarpdr::W](W) writer structure"]
impl crate::Writable for DMARPDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMARPDR to value 0"]
impl crate::Resettable for DMARPDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
