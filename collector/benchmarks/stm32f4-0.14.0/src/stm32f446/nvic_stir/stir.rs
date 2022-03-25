#[doc = "Register `STIR` reader"]
pub struct R(crate::R<STIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STIR` writer"]
pub struct W(crate::W<STIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STIR_SPEC>;
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
impl From<crate::W<STIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTID` reader - Software generated interrupt ID"]
pub struct INTID_R(crate::FieldReader<u16, u16>);
impl INTID_R {
    pub(crate) fn new(bits: u16) -> Self {
        INTID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTID` writer - Software generated interrupt ID"]
pub struct INTID_W<'a> {
    w: &'a mut W,
}
impl<'a> INTID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Software generated interrupt ID"]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Software generated interrupt ID"]
    #[inline(always)]
    pub fn intid(&mut self) -> INTID_W {
        INTID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software trigger interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stir](index.html) module"]
pub struct STIR_SPEC;
impl crate::RegisterSpec for STIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stir::R](R) reader structure"]
impl crate::Readable for STIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stir::W](W) writer structure"]
impl crate::Writable for STIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STIR to value 0"]
impl crate::Resettable for STIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
