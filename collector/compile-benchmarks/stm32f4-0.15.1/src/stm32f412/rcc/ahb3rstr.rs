#[doc = "Register `AHB3RSTR` reader"]
pub struct R(crate::R<AHB3RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB3RSTR` writer"]
pub struct W(crate::W<AHB3RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3RSTR_SPEC>;
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
impl From<crate::W<AHB3RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Flexible static memory controller module reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSMCRST_A {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<FSMCRST_A> for bool {
    #[inline(always)]
    fn from(variant: FSMCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSMCRST` reader - Flexible static memory controller module reset"]
pub type FSMCRST_R = crate::BitReader<FSMCRST_A>;
impl FSMCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FSMCRST_A> {
        match self.bits {
            true => Some(FSMCRST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FSMCRST_A::Reset
    }
}
#[doc = "Field `FSMCRST` writer - Flexible static memory controller module reset"]
pub type FSMCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3RSTR_SPEC, FSMCRST_A, O>;
impl<'a, const O: u8> FSMCRST_W<'a, O> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FSMCRST_A::Reset)
    }
}
#[doc = "QUADSPI module reset"]
pub use FSMCRST_A as QSPIRST_A;
#[doc = "Field `QSPIRST` reader - QUADSPI module reset"]
pub use FSMCRST_R as QSPIRST_R;
#[doc = "Field `QSPIRST` writer - QUADSPI module reset"]
pub use FSMCRST_W as QSPIRST_W;
impl R {
    #[doc = "Bit 0 - Flexible static memory controller module reset"]
    #[inline(always)]
    pub fn fsmcrst(&self) -> FSMCRST_R {
        FSMCRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QUADSPI module reset"]
    #[inline(always)]
    pub fn qspirst(&self) -> QSPIRST_R {
        QSPIRST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible static memory controller module reset"]
    #[inline(always)]
    pub fn fsmcrst(&mut self) -> FSMCRST_W<0> {
        FSMCRST_W::new(self)
    }
    #[doc = "Bit 1 - QUADSPI module reset"]
    #[inline(always)]
    pub fn qspirst(&mut self) -> QSPIRST_W<1> {
        QSPIRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB3 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3rstr](index.html) module"]
pub struct AHB3RSTR_SPEC;
impl crate::RegisterSpec for AHB3RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb3rstr::R](R) reader structure"]
impl crate::Readable for AHB3RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb3rstr::W](W) writer structure"]
impl crate::Writable for AHB3RSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB3RSTR to value 0"]
impl crate::Resettable for AHB3RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
