#[doc = "Register `AHB2ENR` reader"]
pub struct R(crate::R<AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2ENR` writer"]
pub struct W(crate::W<AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2ENR_SPEC>;
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
impl From<crate::W<AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB OTG FS clock enable"]
pub use CRYPEN_A as OTGFSEN_A;
#[doc = "RNGEN"]
pub use CRYPEN_A as RNGEN_A;
#[doc = "Field `OTGFSEN` reader - USB OTG FS clock enable"]
pub use CRYPEN_R as OTGFSEN_R;
#[doc = "Field `RNGEN` reader - RNGEN"]
pub use CRYPEN_R as RNGEN_R;
#[doc = "Field `OTGFSEN` writer - USB OTG FS clock enable"]
pub use CRYPEN_W as OTGFSEN_W;
#[doc = "Field `RNGEN` writer - RNGEN"]
pub use CRYPEN_W as RNGEN_W;
#[doc = "CRYP clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYPEN_A {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<CRYPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRYPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRYPEN` reader - CRYP clock enable"]
pub type CRYPEN_R = crate::BitReader<CRYPEN_A>;
impl CRYPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRYPEN_A {
        match self.bits {
            false => CRYPEN_A::Disabled,
            true => CRYPEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRYPEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRYPEN_A::Enabled
    }
}
#[doc = "Field `CRYPEN` writer - CRYP clock enable"]
pub type CRYPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, CRYPEN_A, O>;
impl<'a, const O: u8> CRYPEN_W<'a, O> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRYPEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRYPEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 7 - USB OTG FS clock enable"]
    #[inline(always)]
    pub fn otgfsen(&self) -> OTGFSEN_R {
        OTGFSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - RNGEN"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 4 - CRYP clock enable"]
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - USB OTG FS clock enable"]
    #[inline(always)]
    pub fn otgfsen(&mut self) -> OTGFSEN_W<7> {
        OTGFSEN_W::new(self)
    }
    #[doc = "Bit 6 - RNGEN"]
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<6> {
        RNGEN_W::new(self)
    }
    #[doc = "Bit 4 - CRYP clock enable"]
    #[inline(always)]
    pub fn crypen(&mut self) -> CRYPEN_W<4> {
        CRYPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2enr](index.html) module"]
pub struct AHB2ENR_SPEC;
impl crate::RegisterSpec for AHB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2enr::R](R) reader structure"]
impl crate::Readable for AHB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2enr::W](W) writer structure"]
impl crate::Writable for AHB2ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for AHB2ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
