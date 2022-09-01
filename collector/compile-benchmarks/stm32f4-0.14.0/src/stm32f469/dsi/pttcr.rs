#[doc = "Register `PTTCR` reader"]
pub struct R(crate::R<PTTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTTCR` writer"]
pub struct W(crate::W<PTTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTTCR_SPEC>;
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
impl From<crate::W<PTTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_TRIG` reader - Transmission Trigger"]
pub struct TX_TRIG_R(crate::FieldReader<u8, u8>);
impl TX_TRIG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_TRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_TRIG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_TRIG` writer - Transmission Trigger"]
pub struct TX_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TRIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Transmission Trigger"]
    #[inline(always)]
    pub fn tx_trig(&self) -> TX_TRIG_R {
        TX_TRIG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmission Trigger"]
    #[inline(always)]
    pub fn tx_trig(&mut self) -> TX_TRIG_W {
        TX_TRIG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host PHY TX Triggers Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pttcr](index.html) module"]
pub struct PTTCR_SPEC;
impl crate::RegisterSpec for PTTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pttcr::R](R) reader structure"]
impl crate::Readable for PTTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pttcr::W](W) writer structure"]
impl crate::Writable for PTTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTTCR to value 0"]
impl crate::Resettable for PTTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
