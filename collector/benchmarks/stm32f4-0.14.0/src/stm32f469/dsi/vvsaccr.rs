#[doc = "Register `VVSACCR` reader"]
pub struct R(crate::R<VVSACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VVSACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VVSACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VVSACCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VVSACCR` writer"]
pub struct W(crate::W<VVSACCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VVSACCR_SPEC>;
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
impl From<crate::W<VVSACCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VVSACCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VSA` reader - Vertical Synchronism Active duration"]
pub struct VSA_R(crate::FieldReader<u16, u16>);
impl VSA_R {
    pub(crate) fn new(bits: u16) -> Self {
        VSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSA` writer - Vertical Synchronism Active duration"]
pub struct VSA_W<'a> {
    w: &'a mut W,
}
impl<'a> VSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Vertical Synchronism Active duration"]
    #[inline(always)]
    pub fn vsa(&self) -> VSA_R {
        VSA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical Synchronism Active duration"]
    #[inline(always)]
    pub fn vsa(&mut self) -> VSA_W {
        VSA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Video VSA Current Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vvsaccr](index.html) module"]
pub struct VVSACCR_SPEC;
impl crate::RegisterSpec for VVSACCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vvsaccr::R](R) reader structure"]
impl crate::Readable for VVSACCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vvsaccr::W](W) writer structure"]
impl crate::Writable for VVSACCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VVSACCR to value 0"]
impl crate::Resettable for VVSACCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
