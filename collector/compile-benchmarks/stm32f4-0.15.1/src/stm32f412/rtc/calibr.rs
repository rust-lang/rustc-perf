#[doc = "Register `CALIBR` reader"]
pub struct R(crate::R<CALIBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALIBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALIBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALIBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALIBR` writer"]
pub struct W(crate::W<CALIBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALIBR_SPEC>;
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
impl From<crate::W<CALIBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALIBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCS` reader - Digital calibration sign"]
pub type DCS_R = crate::BitReader<bool>;
#[doc = "Field `DCS` writer - Digital calibration sign"]
pub type DCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALIBR_SPEC, bool, O>;
#[doc = "Field `DC` reader - Digital calibration"]
pub type DC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DC` writer - Digital calibration"]
pub type DC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALIBR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 7 - Digital calibration sign"]
    #[inline(always)]
    pub fn dcs(&self) -> DCS_R {
        DCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 0:4 - Digital calibration"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Digital calibration sign"]
    #[inline(always)]
    pub fn dcs(&mut self) -> DCS_W<7> {
        DCS_W::new(self)
    }
    #[doc = "Bits 0:4 - Digital calibration"]
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W<0> {
        DC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calibr](index.html) module"]
pub struct CALIBR_SPEC;
impl crate::RegisterSpec for CALIBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calibr::R](R) reader structure"]
impl crate::Readable for CALIBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calibr::W](W) writer structure"]
impl crate::Writable for CALIBR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALIBR to value 0"]
impl crate::Resettable for CALIBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
