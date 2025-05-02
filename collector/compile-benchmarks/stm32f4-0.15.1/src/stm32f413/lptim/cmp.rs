#[doc = "Register `CMP` reader"]
pub struct R(crate::R<CMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMP` writer"]
pub struct W(crate::W<CMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMP_SPEC>;
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
impl From<crate::W<CMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP` reader - Compare value"]
pub type CMP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CMP` writer - Compare value"]
pub type CMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMP_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Compare value"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare value"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CMP_W<0> {
        CMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp](index.html) module"]
pub struct CMP_SPEC;
impl crate::RegisterSpec for CMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmp::R](R) reader structure"]
impl crate::Readable for CMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmp::W](W) writer structure"]
impl crate::Writable for CMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMP to value 0"]
impl crate::Resettable for CMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
