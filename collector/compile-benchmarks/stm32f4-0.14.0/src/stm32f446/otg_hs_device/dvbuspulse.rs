#[doc = "Register `DVBUSPULSE` reader"]
pub struct R(crate::R<DVBUSPULSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVBUSPULSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVBUSPULSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVBUSPULSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DVBUSPULSE` writer"]
pub struct W(crate::W<DVBUSPULSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVBUSPULSE_SPEC>;
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
impl From<crate::W<DVBUSPULSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVBUSPULSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DVBUSP` reader - Device VBUS pulsing time"]
pub struct DVBUSP_R(crate::FieldReader<u16, u16>);
impl DVBUSP_R {
    pub(crate) fn new(bits: u16) -> Self {
        DVBUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DVBUSP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DVBUSP` writer - Device VBUS pulsing time"]
pub struct DVBUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> DVBUSP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Device VBUS pulsing time"]
    #[inline(always)]
    pub fn dvbusp(&self) -> DVBUSP_R {
        DVBUSP_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Device VBUS pulsing time"]
    #[inline(always)]
    pub fn dvbusp(&mut self) -> DVBUSP_W {
        DVBUSP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS device VBUS pulsing time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvbuspulse](index.html) module"]
pub struct DVBUSPULSE_SPEC;
impl crate::RegisterSpec for DVBUSPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvbuspulse::R](R) reader structure"]
impl crate::Readable for DVBUSPULSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvbuspulse::W](W) writer structure"]
impl crate::Writable for DVBUSPULSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DVBUSPULSE to value 0x05b8"]
impl crate::Resettable for DVBUSPULSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05b8
    }
}
