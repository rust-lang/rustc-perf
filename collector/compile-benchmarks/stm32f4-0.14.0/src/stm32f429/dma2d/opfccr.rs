#[doc = "Register `OPFCCR` reader"]
pub struct R(crate::R<OPFCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPFCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPFCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPFCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPFCCR` writer"]
pub struct W(crate::W<OPFCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPFCCR_SPEC>;
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
impl From<crate::W<OPFCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPFCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CM` reader - Color mode"]
pub struct CM_R(crate::FieldReader<u8, u8>);
impl CM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CM` writer - Color mode"]
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Color mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Color mode"]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "output PFC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opfccr](index.html) module"]
pub struct OPFCCR_SPEC;
impl crate::RegisterSpec for OPFCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opfccr::R](R) reader structure"]
impl crate::Readable for OPFCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opfccr::W](W) writer structure"]
impl crate::Writable for OPFCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPFCCR to value 0"]
impl crate::Resettable for OPFCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
