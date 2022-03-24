#[doc = "Register `LPTR` reader"]
pub struct R(crate::R<LPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPTR` writer"]
pub struct W(crate::W<LPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTR_SPEC>;
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
impl From<crate::W<LPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMEOUT` reader - Timeout period"]
pub struct TIMEOUT_R(crate::FieldReader<u16, u16>);
impl TIMEOUT_R {
    pub(crate) fn new(bits: u16) -> Self {
        TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEOUT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEOUT` writer - Timeout period"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timeout period"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timeout period"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "low-power timeout register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptr](index.html) module"]
pub struct LPTR_SPEC;
impl crate::RegisterSpec for LPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptr::R](R) reader structure"]
impl crate::Readable for LPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lptr::W](W) writer structure"]
impl crate::Writable for LPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPTR to value 0"]
impl crate::Resettable for LPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
