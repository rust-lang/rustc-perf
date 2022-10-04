#[doc = "Register `PTPTSHUR` reader"]
pub struct R(crate::R<PTPTSHUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPTSHUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPTSHUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPTSHUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTPTSHUR` writer"]
pub struct W(crate::W<PTPTSHUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPTSHUR_SPEC>;
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
impl From<crate::W<PTPTSHUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPTSHUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSUS` reader - TSUS"]
pub struct TSUS_R(crate::FieldReader<u32, u32>);
impl TSUS_R {
    pub(crate) fn new(bits: u32) -> Self {
        TSUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSUS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSUS` writer - TSUS"]
pub struct TSUS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - TSUS"]
    #[inline(always)]
    pub fn tsus(&self) -> TSUS_R {
        TSUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSUS"]
    #[inline(always)]
    pub fn tsus(&mut self) -> TSUS_W {
        TSUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PTP time stamp high update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptptshur](index.html) module"]
pub struct PTPTSHUR_SPEC;
impl crate::RegisterSpec for PTPTSHUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptptshur::R](R) reader structure"]
impl crate::Readable for PTPTSHUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptptshur::W](W) writer structure"]
impl crate::Writable for PTPTSHUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTPTSHUR to value 0"]
impl crate::Resettable for PTPTSHUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
