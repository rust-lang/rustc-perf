#[doc = "Register `ALRMBSSR` reader"]
pub struct R(crate::R<ALRMBSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALRMBSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALRMBSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALRMBSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALRMBSSR` writer"]
pub struct W(crate::W<ALRMBSSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRMBSSR_SPEC>;
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
impl From<crate::W<ALRMBSSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRMBSSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASKSS` reader - Mask the most-significant bits starting at this bit"]
pub struct MASKSS_R(crate::FieldReader<u8, u8>);
impl MASKSS_R {
    pub(crate) fn new(bits: u8) -> Self {
        MASKSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASKSS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASKSS` writer - Mask the most-significant bits starting at this bit"]
pub struct MASKSS_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `SS` reader - Sub seconds value"]
pub struct SS_R(crate::FieldReader<u16, u16>);
impl SS_R {
    pub(crate) fn new(bits: u16) -> Self {
        SS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SS` writer - Sub seconds value"]
pub struct SS_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:27 - Mask the most-significant bits starting at this bit"]
    #[inline(always)]
    pub fn maskss(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 0:14 - Sub seconds value"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:27 - Mask the most-significant bits starting at this bit"]
    #[inline(always)]
    pub fn maskss(&mut self) -> MASKSS_W {
        MASKSS_W { w: self }
    }
    #[doc = "Bits 0:14 - Sub seconds value"]
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W {
        SS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "alarm B sub second register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmbssr](index.html) module"]
pub struct ALRMBSSR_SPEC;
impl crate::RegisterSpec for ALRMBSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alrmbssr::R](R) reader structure"]
impl crate::Readable for ALRMBSSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alrmbssr::W](W) writer structure"]
impl crate::Writable for ALRMBSSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALRMBSSR to value 0"]
impl crate::Resettable for ALRMBSSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
