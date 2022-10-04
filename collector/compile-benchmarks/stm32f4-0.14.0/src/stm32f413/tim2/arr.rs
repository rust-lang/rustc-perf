#[doc = "Register `ARR` reader"]
pub struct R(crate::R<ARR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARR` writer"]
pub struct W(crate::W<ARR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARR_SPEC>;
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
impl From<crate::W<ARR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARR` reader - Auto-reload value"]
pub struct ARR_R(crate::FieldReader<u32, u32>);
impl ARR_R {
    pub(crate) fn new(bits: u32) -> Self {
        ARR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARR` writer - Auto-reload value"]
pub struct ARR_W<'a> {
    w: &'a mut W,
}
impl<'a> ARR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Auto-reload value"]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Auto-reload value"]
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W {
        ARR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arr](index.html) module"]
pub struct ARR_SPEC;
impl crate::RegisterSpec for ARR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arr::R](R) reader structure"]
impl crate::Readable for ARR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arr::W](W) writer structure"]
impl crate::Writable for ARR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARR to value 0"]
impl crate::Resettable for ARR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
