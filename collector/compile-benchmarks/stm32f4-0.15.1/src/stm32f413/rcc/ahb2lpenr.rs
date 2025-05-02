#[doc = "Register `AHB2LPENR` reader"]
pub struct R(crate::R<AHB2LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2LPENR` writer"]
pub struct W(crate::W<AHB2LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2LPENR_SPEC>;
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
impl From<crate::W<AHB2LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USB OTG FS clock enable during Sleep mode"]
pub use FSMCLPEN_A as OTGFSLPEN_A;
#[doc = "RNGLPEN"]
pub use FSMCLPEN_A as RNGLPEN_A;
#[doc = "Field `OTGFSLPEN` reader - USB OTG FS clock enable during Sleep mode"]
pub use FSMCLPEN_R as OTGFSLPEN_R;
#[doc = "Field `RNGLPEN` reader - RNGLPEN"]
pub use FSMCLPEN_R as RNGLPEN_R;
#[doc = "Field `OTGFSLPEN` writer - USB OTG FS clock enable during Sleep mode"]
pub use FSMCLPEN_W as OTGFSLPEN_W;
#[doc = "Field `RNGLPEN` writer - RNGLPEN"]
pub use FSMCLPEN_W as RNGLPEN_W;
#[doc = "Flexible memory controller module clock enable during Sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSMCLPEN_A {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DisabledInSleep = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    EnabledInSleep = 1,
}
impl From<FSMCLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: FSMCLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSMCLPEN` reader - Flexible memory controller module clock enable during Sleep mode"]
pub type FSMCLPEN_R = crate::BitReader<FSMCLPEN_A>;
impl FSMCLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSMCLPEN_A {
        match self.bits {
            false => FSMCLPEN_A::DisabledInSleep,
            true => FSMCLPEN_A::EnabledInSleep,
        }
    }
    #[doc = "Checks if the value of the field is `DisabledInSleep`"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == FSMCLPEN_A::DisabledInSleep
    }
    #[doc = "Checks if the value of the field is `EnabledInSleep`"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == FSMCLPEN_A::EnabledInSleep
    }
}
#[doc = "Field `FSMCLPEN` writer - Flexible memory controller module clock enable during Sleep mode"]
pub type FSMCLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2LPENR_SPEC, FSMCLPEN_A, O>;
impl<'a, const O: u8> FSMCLPEN_W<'a, O> {
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(FSMCLPEN_A::DisabledInSleep)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(FSMCLPEN_A::EnabledInSleep)
    }
}
#[doc = "QUADSPI memory controller module clock enable during Sleep mode"]
pub use FSMCLPEN_A as QSPILPEN_A;
#[doc = "Field `QSPILPEN` reader - QUADSPI memory controller module clock enable during Sleep mode"]
pub use FSMCLPEN_R as QSPILPEN_R;
#[doc = "Field `QSPILPEN` writer - QUADSPI memory controller module clock enable during Sleep mode"]
pub use FSMCLPEN_W as QSPILPEN_W;
impl R {
    #[doc = "Bit 7 - USB OTG FS clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otgfslpen(&self) -> OTGFSLPEN_R {
        OTGFSLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - RNGLPEN"]
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 0 - Flexible memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn fsmclpen(&self) -> FSMCLPEN_R {
        FSMCLPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QUADSPI memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn qspilpen(&self) -> QSPILPEN_R {
        QSPILPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - USB OTG FS clock enable during Sleep mode"]
    #[inline(always)]
    pub fn otgfslpen(&mut self) -> OTGFSLPEN_W<7> {
        OTGFSLPEN_W::new(self)
    }
    #[doc = "Bit 6 - RNGLPEN"]
    #[inline(always)]
    pub fn rnglpen(&mut self) -> RNGLPEN_W<6> {
        RNGLPEN_W::new(self)
    }
    #[doc = "Bit 0 - Flexible memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn fsmclpen(&mut self) -> FSMCLPEN_W<0> {
        FSMCLPEN_W::new(self)
    }
    #[doc = "Bit 1 - QUADSPI memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn qspilpen(&mut self) -> QSPILPEN_W<1> {
        QSPILPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2lpenr](index.html) module"]
pub struct AHB2LPENR_SPEC;
impl crate::RegisterSpec for AHB2LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2lpenr::R](R) reader structure"]
impl crate::Readable for AHB2LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2lpenr::W](W) writer structure"]
impl crate::Writable for AHB2LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB2LPENR to value 0xf1"]
impl crate::Resettable for AHB2LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf1
    }
}
