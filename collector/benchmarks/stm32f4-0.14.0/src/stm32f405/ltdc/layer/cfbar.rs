#[doc = "Register `CFBAR` reader"]
pub struct R(crate::R<CFBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFBAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFBAR` writer"]
pub struct W(crate::W<CFBAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFBAR_SPEC>;
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
impl From<crate::W<CFBAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFBAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFBADD` reader - Color Frame Buffer Start Address"]
pub struct CFBADD_R(crate::FieldReader<u32, u32>);
impl CFBADD_R {
    pub(crate) fn new(bits: u32) -> Self {
        CFBADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFBADD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFBADD` writer - Color Frame Buffer Start Address"]
pub struct CFBADD_W<'a> {
    w: &'a mut W,
}
impl<'a> CFBADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Color Frame Buffer Start Address"]
    #[inline(always)]
    pub fn cfbadd(&self) -> CFBADD_R {
        CFBADD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Color Frame Buffer Start Address"]
    #[inline(always)]
    pub fn cfbadd(&mut self) -> CFBADD_W {
        CFBADD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layerx Color Frame Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfbar](index.html) module"]
pub struct CFBAR_SPEC;
impl crate::RegisterSpec for CFBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfbar::R](R) reader structure"]
impl crate::Readable for CFBAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfbar::W](W) writer structure"]
impl crate::Writable for CFBAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFBAR to value 0"]
impl crate::Resettable for CFBAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
