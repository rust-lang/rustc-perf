#[doc = "Register `AR` reader"]
pub struct R(crate::R<AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AR` writer"]
pub struct W(crate::W<AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AR_SPEC>;
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
impl From<crate::W<AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS` reader - Address"]
pub struct ADDRESS_R(crate::FieldReader<u32, u32>);
impl ADDRESS_R {
    pub(crate) fn new(bits: u32) -> Self {
        ADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRESS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRESS` writer - Address"]
pub struct ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Address"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address"]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W {
        ADDRESS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ar](index.html) module"]
pub struct AR_SPEC;
impl crate::RegisterSpec for AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ar::R](R) reader structure"]
impl crate::Readable for AR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ar::W](W) writer structure"]
impl crate::Writable for AR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AR to value 0"]
impl crate::Resettable for AR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
