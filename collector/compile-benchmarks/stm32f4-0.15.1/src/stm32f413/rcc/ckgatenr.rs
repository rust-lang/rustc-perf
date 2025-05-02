#[doc = "Register `CKGATENR` reader"]
pub struct R(crate::R<CKGATENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKGATENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKGATENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKGATENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKGATENR` writer"]
pub struct W(crate::W<CKGATENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKGATENR_SPEC>;
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
impl From<crate::W<CKGATENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKGATENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "AHB to APB1 Bridge clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB2APB1_CKEN_A {
    #[doc = "0: The clock gating is enabled"]
    Enabled = 0,
    #[doc = "1: The clock gating is disabled, the clock is always enabled"]
    Disabled = 1,
}
impl From<AHB2APB1_CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: AHB2APB1_CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB2APB1_CKEN` reader - AHB to APB1 Bridge clock enable"]
pub type AHB2APB1_CKEN_R = crate::BitReader<AHB2APB1_CKEN_A>;
impl AHB2APB1_CKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB2APB1_CKEN_A {
        match self.bits {
            false => AHB2APB1_CKEN_A::Enabled,
            true => AHB2APB1_CKEN_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AHB2APB1_CKEN_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AHB2APB1_CKEN_A::Disabled
    }
}
#[doc = "Field `AHB2APB1_CKEN` writer - AHB to APB1 Bridge clock enable"]
pub type AHB2APB1_CKEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CKGATENR_SPEC, AHB2APB1_CKEN_A, O>;
impl<'a, const O: u8> AHB2APB1_CKEN_W<'a, O> {
    #[doc = "The clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AHB2APB1_CKEN_A::Enabled)
    }
    #[doc = "The clock gating is disabled, the clock is always enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AHB2APB1_CKEN_A::Disabled)
    }
}
#[doc = "AHB to APB2 Bridge clock enable"]
pub use AHB2APB1_CKEN_A as AHB2APB2_CKEN_A;
#[doc = "Cortex M4 ETM clock enable"]
pub use AHB2APB1_CKEN_A as CM4DBG_CKEN_A;
#[doc = "Spare clock enable"]
pub use AHB2APB1_CKEN_A as SPARE_CKEN_A;
#[doc = "SRQAM controller clock enable"]
pub use AHB2APB1_CKEN_A as SRAM_CKEN_A;
#[doc = "Flash Interface clock enable"]
pub use AHB2APB1_CKEN_A as FLITF_CKEN_A;
#[doc = "RCC clock enable"]
pub use AHB2APB1_CKEN_A as RCC_CKEN_A;
#[doc = "EVTCL_CKEN"]
pub use AHB2APB1_CKEN_A as EVTCL_CKEN_A;
#[doc = "Field `AHB2APB2_CKEN` reader - AHB to APB2 Bridge clock enable"]
pub use AHB2APB1_CKEN_R as AHB2APB2_CKEN_R;
#[doc = "Field `CM4DBG_CKEN` reader - Cortex M4 ETM clock enable"]
pub use AHB2APB1_CKEN_R as CM4DBG_CKEN_R;
#[doc = "Field `SPARE_CKEN` reader - Spare clock enable"]
pub use AHB2APB1_CKEN_R as SPARE_CKEN_R;
#[doc = "Field `SRAM_CKEN` reader - SRQAM controller clock enable"]
pub use AHB2APB1_CKEN_R as SRAM_CKEN_R;
#[doc = "Field `FLITF_CKEN` reader - Flash Interface clock enable"]
pub use AHB2APB1_CKEN_R as FLITF_CKEN_R;
#[doc = "Field `RCC_CKEN` reader - RCC clock enable"]
pub use AHB2APB1_CKEN_R as RCC_CKEN_R;
#[doc = "Field `EVTCL_CKEN` reader - EVTCL_CKEN"]
pub use AHB2APB1_CKEN_R as EVTCL_CKEN_R;
#[doc = "Field `AHB2APB2_CKEN` writer - AHB to APB2 Bridge clock enable"]
pub use AHB2APB1_CKEN_W as AHB2APB2_CKEN_W;
#[doc = "Field `CM4DBG_CKEN` writer - Cortex M4 ETM clock enable"]
pub use AHB2APB1_CKEN_W as CM4DBG_CKEN_W;
#[doc = "Field `SPARE_CKEN` writer - Spare clock enable"]
pub use AHB2APB1_CKEN_W as SPARE_CKEN_W;
#[doc = "Field `SRAM_CKEN` writer - SRQAM controller clock enable"]
pub use AHB2APB1_CKEN_W as SRAM_CKEN_W;
#[doc = "Field `FLITF_CKEN` writer - Flash Interface clock enable"]
pub use AHB2APB1_CKEN_W as FLITF_CKEN_W;
#[doc = "Field `RCC_CKEN` writer - RCC clock enable"]
pub use AHB2APB1_CKEN_W as RCC_CKEN_W;
#[doc = "Field `EVTCL_CKEN` writer - EVTCL_CKEN"]
pub use AHB2APB1_CKEN_W as EVTCL_CKEN_W;
impl R {
    #[doc = "Bit 0 - AHB to APB1 Bridge clock enable"]
    #[inline(always)]
    pub fn ahb2apb1_cken(&self) -> AHB2APB1_CKEN_R {
        AHB2APB1_CKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AHB to APB2 Bridge clock enable"]
    #[inline(always)]
    pub fn ahb2apb2_cken(&self) -> AHB2APB2_CKEN_R {
        AHB2APB2_CKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cortex M4 ETM clock enable"]
    #[inline(always)]
    pub fn cm4dbg_cken(&self) -> CM4DBG_CKEN_R {
        CM4DBG_CKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Spare clock enable"]
    #[inline(always)]
    pub fn spare_cken(&self) -> SPARE_CKEN_R {
        SPARE_CKEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRQAM controller clock enable"]
    #[inline(always)]
    pub fn sram_cken(&self) -> SRAM_CKEN_R {
        SRAM_CKEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash Interface clock enable"]
    #[inline(always)]
    pub fn flitf_cken(&self) -> FLITF_CKEN_R {
        FLITF_CKEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RCC clock enable"]
    #[inline(always)]
    pub fn rcc_cken(&self) -> RCC_CKEN_R {
        RCC_CKEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EVTCL_CKEN"]
    #[inline(always)]
    pub fn evtcl_cken(&self) -> EVTCL_CKEN_R {
        EVTCL_CKEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AHB to APB1 Bridge clock enable"]
    #[inline(always)]
    pub fn ahb2apb1_cken(&mut self) -> AHB2APB1_CKEN_W<0> {
        AHB2APB1_CKEN_W::new(self)
    }
    #[doc = "Bit 1 - AHB to APB2 Bridge clock enable"]
    #[inline(always)]
    pub fn ahb2apb2_cken(&mut self) -> AHB2APB2_CKEN_W<1> {
        AHB2APB2_CKEN_W::new(self)
    }
    #[doc = "Bit 2 - Cortex M4 ETM clock enable"]
    #[inline(always)]
    pub fn cm4dbg_cken(&mut self) -> CM4DBG_CKEN_W<2> {
        CM4DBG_CKEN_W::new(self)
    }
    #[doc = "Bit 3 - Spare clock enable"]
    #[inline(always)]
    pub fn spare_cken(&mut self) -> SPARE_CKEN_W<3> {
        SPARE_CKEN_W::new(self)
    }
    #[doc = "Bit 4 - SRQAM controller clock enable"]
    #[inline(always)]
    pub fn sram_cken(&mut self) -> SRAM_CKEN_W<4> {
        SRAM_CKEN_W::new(self)
    }
    #[doc = "Bit 5 - Flash Interface clock enable"]
    #[inline(always)]
    pub fn flitf_cken(&mut self) -> FLITF_CKEN_W<5> {
        FLITF_CKEN_W::new(self)
    }
    #[doc = "Bit 6 - RCC clock enable"]
    #[inline(always)]
    pub fn rcc_cken(&mut self) -> RCC_CKEN_W<6> {
        RCC_CKEN_W::new(self)
    }
    #[doc = "Bit 7 - EVTCL_CKEN"]
    #[inline(always)]
    pub fn evtcl_cken(&mut self) -> EVTCL_CKEN_W<7> {
        EVTCL_CKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC clocks gated enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgatenr](index.html) module"]
pub struct CKGATENR_SPEC;
impl crate::RegisterSpec for CKGATENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckgatenr::R](R) reader structure"]
impl crate::Readable for CKGATENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckgatenr::W](W) writer structure"]
impl crate::Writable for CKGATENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CKGATENR to value 0"]
impl crate::Resettable for CKGATENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
