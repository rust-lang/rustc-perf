#[doc = "Register `DMARPDR` reader"]
pub struct R(crate::R<DMARPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMARPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMARPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMARPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMARPDR` writer"]
pub struct W(crate::W<DMARPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMARPDR_SPEC>;
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
impl From<crate::W<DMARPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMARPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive poll demand\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum RPD_A {
    #[doc = "0: Poll the receive descriptor list"]
    POLL = 0,
}
impl From<RPD_A> for u32 {
    #[inline(always)]
    fn from(variant: RPD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RPD` reader - Receive poll demand"]
pub struct RPD_R(crate::FieldReader<u32, RPD_A>);
impl RPD_R {
    pub(crate) fn new(bits: u32) -> Self {
        RPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RPD_A> {
        match self.bits {
            0 => Some(RPD_A::POLL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `POLL`"]
    #[inline(always)]
    pub fn is_poll(&self) -> bool {
        **self == RPD_A::POLL
    }
}
impl core::ops::Deref for RPD_R {
    type Target = crate::FieldReader<u32, RPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPD` writer - Receive poll demand"]
pub struct RPD_W<'a> {
    w: &'a mut W,
}
impl<'a> RPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Poll the receive descriptor list"]
    #[inline(always)]
    pub fn poll(self) -> &'a mut W {
        self.variant(RPD_A::POLL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    pub fn rpd(&self) -> RPD_R {
        RPD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    pub fn rpd(&mut self) -> RPD_W {
        RPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EHERNET DMA receive poll demand register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmarpdr](index.html) module"]
pub struct DMARPDR_SPEC;
impl crate::RegisterSpec for DMARPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmarpdr::R](R) reader structure"]
impl crate::Readable for DMARPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmarpdr::W](W) writer structure"]
impl crate::Writable for DMARPDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMARPDR to value 0"]
impl crate::Resettable for DMARPDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
