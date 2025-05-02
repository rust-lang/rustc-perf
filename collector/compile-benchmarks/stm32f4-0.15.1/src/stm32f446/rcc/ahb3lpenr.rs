#[doc = "Register `AHB3LPENR` reader"]
pub struct R(crate::R<AHB3LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB3LPENR` writer"]
pub struct W(crate::W<AHB3LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3LPENR_SPEC>;
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
impl From<crate::W<AHB3LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Flexible memory controller module clock enable during Sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMCLPEN_A {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DisabledInSleep = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    EnabledInSleep = 1,
}
impl From<FMCLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: FMCLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMCLPEN` reader - Flexible memory controller module clock enable during Sleep mode"]
pub type FMCLPEN_R = crate::BitReader<FMCLPEN_A>;
impl FMCLPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMCLPEN_A {
        match self.bits {
            false => FMCLPEN_A::DisabledInSleep,
            true => FMCLPEN_A::EnabledInSleep,
        }
    }
    #[doc = "Checks if the value of the field is `DisabledInSleep`"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == FMCLPEN_A::DisabledInSleep
    }
    #[doc = "Checks if the value of the field is `EnabledInSleep`"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == FMCLPEN_A::EnabledInSleep
    }
}
#[doc = "Field `FMCLPEN` writer - Flexible memory controller module clock enable during Sleep mode"]
pub type FMCLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3LPENR_SPEC, FMCLPEN_A, O>;
impl<'a, const O: u8> FMCLPEN_W<'a, O> {
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(FMCLPEN_A::DisabledInSleep)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(FMCLPEN_A::EnabledInSleep)
    }
}
#[doc = "QUADSPI memory controller module clock enable during Sleep mode"]
pub use FMCLPEN_A as QSPILPEN_A;
#[doc = "Field `QSPILPEN` reader - QUADSPI memory controller module clock enable during Sleep mode"]
pub use FMCLPEN_R as QSPILPEN_R;
#[doc = "Field `QSPILPEN` writer - QUADSPI memory controller module clock enable during Sleep mode"]
pub use FMCLPEN_W as QSPILPEN_W;
impl R {
    #[doc = "Bit 0 - Flexible memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QUADSPI memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn qspilpen(&self) -> QSPILPEN_R {
        QSPILPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn fmclpen(&mut self) -> FMCLPEN_W<0> {
        FMCLPEN_W::new(self)
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
#[doc = "AHB3 peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3lpenr](index.html) module"]
pub struct AHB3LPENR_SPEC;
impl crate::RegisterSpec for AHB3LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb3lpenr::R](R) reader structure"]
impl crate::Readable for AHB3LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb3lpenr::W](W) writer structure"]
impl crate::Writable for AHB3LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB3LPENR to value 0x01"]
impl crate::Resettable for AHB3LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
