#[doc = "Register `PRER` reader"]
pub struct R(crate::R<PRER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRER` writer"]
pub struct W(crate::W<PRER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRER_SPEC>;
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
impl From<crate::W<PRER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PREDIV_A` reader - Asynchronous prescaler factor"]
pub struct PREDIV_A_R(crate::FieldReader<u8, u8>);
impl PREDIV_A_R {
    pub(crate) fn new(bits: u8) -> Self {
        PREDIV_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREDIV_A_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREDIV_A` writer - Asynchronous prescaler factor"]
pub struct PREDIV_A_W<'a> {
    w: &'a mut W,
}
impl<'a> PREDIV_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `PREDIV_S` reader - Synchronous prescaler factor"]
pub struct PREDIV_S_R(crate::FieldReader<u16, u16>);
impl PREDIV_S_R {
    pub(crate) fn new(bits: u16) -> Self {
        PREDIV_S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREDIV_S_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREDIV_S` writer - Synchronous prescaler factor"]
pub struct PREDIV_S_W<'a> {
    w: &'a mut W,
}
impl<'a> PREDIV_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:22 - Asynchronous prescaler factor"]
    #[inline(always)]
    pub fn prediv_a(&self) -> PREDIV_A_R {
        PREDIV_A_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 0:14 - Synchronous prescaler factor"]
    #[inline(always)]
    pub fn prediv_s(&self) -> PREDIV_S_R {
        PREDIV_S_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:22 - Asynchronous prescaler factor"]
    #[inline(always)]
    pub fn prediv_a(&mut self) -> PREDIV_A_W {
        PREDIV_A_W { w: self }
    }
    #[doc = "Bits 0:14 - Synchronous prescaler factor"]
    #[inline(always)]
    pub fn prediv_s(&mut self) -> PREDIV_S_W {
        PREDIV_S_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prer](index.html) module"]
pub struct PRER_SPEC;
impl crate::RegisterSpec for PRER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prer::R](R) reader structure"]
impl crate::Readable for PRER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prer::W](W) writer structure"]
impl crate::Writable for PRER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRER to value 0x007f_00ff"]
impl crate::Resettable for PRER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x007f_00ff
    }
}
