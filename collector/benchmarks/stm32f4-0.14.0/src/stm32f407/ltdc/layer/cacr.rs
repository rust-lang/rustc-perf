#[doc = "Register `CACR` reader"]
pub struct R(crate::R<CACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACR` writer"]
pub struct W(crate::W<CACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACR_SPEC>;
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
impl From<crate::W<CACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONSTA` reader - Constant Alpha"]
pub struct CONSTA_R(crate::FieldReader<u8, u8>);
impl CONSTA_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONSTA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONSTA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONSTA` writer - Constant Alpha"]
pub struct CONSTA_W<'a> {
    w: &'a mut W,
}
impl<'a> CONSTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Constant Alpha"]
    #[inline(always)]
    pub fn consta(&self) -> CONSTA_R {
        CONSTA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Constant Alpha"]
    #[inline(always)]
    pub fn consta(&mut self) -> CONSTA_W {
        CONSTA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layerx Constant Alpha Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cacr](index.html) module"]
pub struct CACR_SPEC;
impl crate::RegisterSpec for CACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cacr::R](R) reader structure"]
impl crate::Readable for CACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cacr::W](W) writer structure"]
impl crate::Writable for CACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACR to value 0"]
impl crate::Resettable for CACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
