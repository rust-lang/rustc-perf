#[doc = "Register `ABR` reader"]
pub struct R(crate::R<ABR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ABR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ABR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ABR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ABR` writer"]
pub struct W(crate::W<ABR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ABR_SPEC>;
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
impl From<crate::W<ABR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ABR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALTERNATE` reader - ALTERNATE"]
pub struct ALTERNATE_R(crate::FieldReader<u32, u32>);
impl ALTERNATE_R {
    pub(crate) fn new(bits: u32) -> Self {
        ALTERNATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALTERNATE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALTERNATE` writer - ALTERNATE"]
pub struct ALTERNATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTERNATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ALTERNATE"]
    #[inline(always)]
    pub fn alternate(&self) -> ALTERNATE_R {
        ALTERNATE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ALTERNATE"]
    #[inline(always)]
    pub fn alternate(&mut self) -> ALTERNATE_W {
        ALTERNATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ABR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abr](index.html) module"]
pub struct ABR_SPEC;
impl crate::RegisterSpec for ABR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [abr::R](R) reader structure"]
impl crate::Readable for ABR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [abr::W](W) writer structure"]
impl crate::Writable for ABR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ABR to value 0"]
impl crate::Resettable for ABR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
