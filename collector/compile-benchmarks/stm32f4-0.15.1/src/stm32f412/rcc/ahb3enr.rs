#[doc = "Register `AHB3ENR` reader"]
pub struct R(crate::R<AHB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB3ENR` writer"]
pub struct W(crate::W<AHB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3ENR_SPEC>;
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
impl From<crate::W<AHB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Flexible static memory controller module clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSMCEN_A {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<FSMCEN_A> for bool {
    #[inline(always)]
    fn from(variant: FSMCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSMCEN` reader - Flexible static memory controller module clock enable"]
pub type FSMCEN_R = crate::BitReader<FSMCEN_A>;
impl FSMCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSMCEN_A {
        match self.bits {
            false => FSMCEN_A::Disabled,
            true => FSMCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FSMCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FSMCEN_A::Enabled
    }
}
#[doc = "Field `FSMCEN` writer - Flexible static memory controller module clock enable"]
pub type FSMCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3ENR_SPEC, FSMCEN_A, O>;
impl<'a, const O: u8> FSMCEN_W<'a, O> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FSMCEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FSMCEN_A::Enabled)
    }
}
#[doc = "QUADSPI memory controller module clock enable"]
pub use FSMCEN_A as QSPIEN_A;
#[doc = "Field `QSPIEN` reader - QUADSPI memory controller module clock enable"]
pub use FSMCEN_R as QSPIEN_R;
#[doc = "Field `QSPIEN` writer - QUADSPI memory controller module clock enable"]
pub use FSMCEN_W as QSPIEN_W;
impl R {
    #[doc = "Bit 0 - Flexible static memory controller module clock enable"]
    #[inline(always)]
    pub fn fsmcen(&self) -> FSMCEN_R {
        FSMCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QUADSPI memory controller module clock enable"]
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible static memory controller module clock enable"]
    #[inline(always)]
    pub fn fsmcen(&mut self) -> FSMCEN_W<0> {
        FSMCEN_W::new(self)
    }
    #[doc = "Bit 1 - QUADSPI memory controller module clock enable"]
    #[inline(always)]
    pub fn qspien(&mut self) -> QSPIEN_W<1> {
        QSPIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB3 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3enr](index.html) module"]
pub struct AHB3ENR_SPEC;
impl crate::RegisterSpec for AHB3ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb3enr::R](R) reader structure"]
impl crate::Readable for AHB3ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb3enr::W](W) writer structure"]
impl crate::Writable for AHB3ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB3ENR to value 0"]
impl crate::Resettable for AHB3ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
