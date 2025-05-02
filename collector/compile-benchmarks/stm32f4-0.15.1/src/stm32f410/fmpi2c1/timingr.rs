#[doc = "Register `TIMINGR` reader"]
pub struct R(crate::R<TIMINGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMINGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMINGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMINGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMINGR` writer"]
pub struct W(crate::W<TIMINGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMINGR_SPEC>;
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
impl From<crate::W<TIMINGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMINGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLL` reader - SCL low period (master mode)"]
pub type SCLL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLL` writer - SCL low period (master mode)"]
pub type SCLL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TIMINGR_SPEC, u8, u8, 8, O>;
#[doc = "Field `SCLH` reader - SCL high period (master mode)"]
pub type SCLH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLH` writer - SCL high period (master mode)"]
pub type SCLH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TIMINGR_SPEC, u8, u8, 8, O>;
#[doc = "Field `SDADEL` reader - Data hold time"]
pub type SDADEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDADEL` writer - Data hold time"]
pub type SDADEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TIMINGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `SCLDEL` reader - Data setup time"]
pub type SCLDEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLDEL` writer - Data setup time"]
pub type SCLDEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TIMINGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `PRESC` reader - Timing prescaler"]
pub type PRESC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESC` writer - Timing prescaler"]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TIMINGR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - SCL low period (master mode)"]
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCL high period (master mode)"]
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data hold time"]
    #[inline(always)]
    pub fn sdadel(&self) -> SDADEL_R {
        SDADEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Data setup time"]
    #[inline(always)]
    pub fn scldel(&self) -> SCLDEL_R {
        SCLDEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Timing prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCL low period (master mode)"]
    #[inline(always)]
    pub fn scll(&mut self) -> SCLL_W<0> {
        SCLL_W::new(self)
    }
    #[doc = "Bits 8:15 - SCL high period (master mode)"]
    #[inline(always)]
    pub fn sclh(&mut self) -> SCLH_W<8> {
        SCLH_W::new(self)
    }
    #[doc = "Bits 16:19 - Data hold time"]
    #[inline(always)]
    pub fn sdadel(&mut self) -> SDADEL_W<16> {
        SDADEL_W::new(self)
    }
    #[doc = "Bits 20:23 - Data setup time"]
    #[inline(always)]
    pub fn scldel(&mut self) -> SCLDEL_W<20> {
        SCLDEL_W::new(self)
    }
    #[doc = "Bits 28:31 - Timing prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<28> {
        PRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timingr](index.html) module"]
pub struct TIMINGR_SPEC;
impl crate::RegisterSpec for TIMINGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timingr::R](R) reader structure"]
impl crate::Readable for TIMINGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timingr::W](W) writer structure"]
impl crate::Writable for TIMINGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMINGR to value 0"]
impl crate::Resettable for TIMINGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
