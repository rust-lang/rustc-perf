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
    ENABLED = 0,
    #[doc = "1: The clock gating is disabled, the clock is always enabled"]
    DISABLED = 1,
}
impl From<AHB2APB1_CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: AHB2APB1_CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB2APB1_CKEN` reader - AHB to APB1 Bridge clock enable"]
pub struct AHB2APB1_CKEN_R(crate::FieldReader<bool, AHB2APB1_CKEN_A>);
impl AHB2APB1_CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHB2APB1_CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB2APB1_CKEN_A {
        match self.bits {
            false => AHB2APB1_CKEN_A::ENABLED,
            true => AHB2APB1_CKEN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AHB2APB1_CKEN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AHB2APB1_CKEN_A::DISABLED
    }
}
impl core::ops::Deref for AHB2APB1_CKEN_R {
    type Target = crate::FieldReader<bool, AHB2APB1_CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB2APB1_CKEN` writer - AHB to APB1 Bridge clock enable"]
pub struct AHB2APB1_CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB2APB1_CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB2APB1_CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AHB2APB1_CKEN_A::ENABLED)
    }
    #[doc = "The clock gating is disabled, the clock is always enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AHB2APB1_CKEN_A::DISABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "AHB to APB2 Bridge clock enable"]
pub type AHB2APB2_CKEN_A = AHB2APB1_CKEN_A;
#[doc = "Field `AHB2APB2_CKEN` reader - AHB to APB2 Bridge clock enable"]
pub type AHB2APB2_CKEN_R = AHB2APB1_CKEN_R;
#[doc = "Field `AHB2APB2_CKEN` writer - AHB to APB2 Bridge clock enable"]
pub struct AHB2APB2_CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB2APB2_CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB2APB2_CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AHB2APB2_CKEN_A::ENABLED)
    }
    #[doc = "The clock gating is disabled, the clock is always enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AHB2APB2_CKEN_A::DISABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Cortex M4 ETM clock enable"]
pub type CM4DBG_CKEN_A = AHB2APB1_CKEN_A;
#[doc = "Field `CM4DBG_CKEN` reader - Cortex M4 ETM clock enable"]
pub type CM4DBG_CKEN_R = AHB2APB1_CKEN_R;
#[doc = "Field `CM4DBG_CKEN` writer - Cortex M4 ETM clock enable"]
pub struct CM4DBG_CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CM4DBG_CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM4DBG_CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CM4DBG_CKEN_A::ENABLED)
    }
    #[doc = "The clock gating is disabled, the clock is always enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CM4DBG_CKEN_A::DISABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Spare clock enable"]
pub type SPARE_CKEN_A = AHB2APB1_CKEN_A;
#[doc = "Field `SPARE_CKEN` reader - Spare clock enable"]
pub type SPARE_CKEN_R = AHB2APB1_CKEN_R;
#[doc = "Field `SPARE_CKEN` writer - Spare clock enable"]
pub struct SPARE_CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPARE_CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPARE_CKEN_A::ENABLED)
    }
    #[doc = "The clock gating is disabled, the clock is always enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPARE_CKEN_A::DISABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "SRAM controller clock enable"]
pub type SRAM_CKEN_A = AHB2APB1_CKEN_A;
#[doc = "Field `SRAM_CKEN` reader - SRAM controller clock enable"]
pub type SRAM_CKEN_R = AHB2APB1_CKEN_R;
#[doc = "Field `SRAM_CKEN` writer - SRAM controller clock enable"]
pub struct SRAM_CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SRAM_CKEN_A::ENABLED)
    }
    #[doc = "The clock gating is disabled, the clock is always enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SRAM_CKEN_A::DISABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Flash interface clock enable"]
pub type FLITF_CKEN_A = AHB2APB1_CKEN_A;
#[doc = "Field `FLITF_CKEN` reader - Flash interface clock enable"]
pub type FLITF_CKEN_R = AHB2APB1_CKEN_R;
#[doc = "Field `FLITF_CKEN` writer - Flash interface clock enable"]
pub struct FLITF_CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLITF_CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLITF_CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLITF_CKEN_A::ENABLED)
    }
    #[doc = "The clock gating is disabled, the clock is always enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLITF_CKEN_A::DISABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "RCC clock enable"]
pub type RCC_CKEN_A = AHB2APB1_CKEN_A;
#[doc = "Field `RCC_CKEN` reader - RCC clock enable"]
pub type RCC_CKEN_R = AHB2APB1_CKEN_R;
#[doc = "Field `RCC_CKEN` writer - RCC clock enable"]
pub struct RCC_CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCC_CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCC_CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RCC_CKEN_A::ENABLED)
    }
    #[doc = "The clock gating is disabled, the clock is always enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RCC_CKEN_A::DISABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "EVTCL clock enable"]
pub type EVTCL_CKEN_A = AHB2APB1_CKEN_A;
#[doc = "Field `EVTCL_CKEN` reader - EVTCL clock enable"]
pub type EVTCL_CKEN_R = AHB2APB1_CKEN_R;
#[doc = "Field `EVTCL_CKEN` writer - EVTCL clock enable"]
pub struct EVTCL_CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EVTCL_CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVTCL_CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gating is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EVTCL_CKEN_A::ENABLED)
    }
    #[doc = "The clock gating is disabled, the clock is always enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EVTCL_CKEN_A::DISABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - AHB to APB1 Bridge clock enable"]
    #[inline(always)]
    pub fn ahb2apb1_cken(&self) -> AHB2APB1_CKEN_R {
        AHB2APB1_CKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AHB to APB2 Bridge clock enable"]
    #[inline(always)]
    pub fn ahb2apb2_cken(&self) -> AHB2APB2_CKEN_R {
        AHB2APB2_CKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Cortex M4 ETM clock enable"]
    #[inline(always)]
    pub fn cm4dbg_cken(&self) -> CM4DBG_CKEN_R {
        CM4DBG_CKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Spare clock enable"]
    #[inline(always)]
    pub fn spare_cken(&self) -> SPARE_CKEN_R {
        SPARE_CKEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SRAM controller clock enable"]
    #[inline(always)]
    pub fn sram_cken(&self) -> SRAM_CKEN_R {
        SRAM_CKEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Flash interface clock enable"]
    #[inline(always)]
    pub fn flitf_cken(&self) -> FLITF_CKEN_R {
        FLITF_CKEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RCC clock enable"]
    #[inline(always)]
    pub fn rcc_cken(&self) -> RCC_CKEN_R {
        RCC_CKEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EVTCL clock enable"]
    #[inline(always)]
    pub fn evtcl_cken(&self) -> EVTCL_CKEN_R {
        EVTCL_CKEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AHB to APB1 Bridge clock enable"]
    #[inline(always)]
    pub fn ahb2apb1_cken(&mut self) -> AHB2APB1_CKEN_W {
        AHB2APB1_CKEN_W { w: self }
    }
    #[doc = "Bit 1 - AHB to APB2 Bridge clock enable"]
    #[inline(always)]
    pub fn ahb2apb2_cken(&mut self) -> AHB2APB2_CKEN_W {
        AHB2APB2_CKEN_W { w: self }
    }
    #[doc = "Bit 2 - Cortex M4 ETM clock enable"]
    #[inline(always)]
    pub fn cm4dbg_cken(&mut self) -> CM4DBG_CKEN_W {
        CM4DBG_CKEN_W { w: self }
    }
    #[doc = "Bit 3 - Spare clock enable"]
    #[inline(always)]
    pub fn spare_cken(&mut self) -> SPARE_CKEN_W {
        SPARE_CKEN_W { w: self }
    }
    #[doc = "Bit 4 - SRAM controller clock enable"]
    #[inline(always)]
    pub fn sram_cken(&mut self) -> SRAM_CKEN_W {
        SRAM_CKEN_W { w: self }
    }
    #[doc = "Bit 5 - Flash interface clock enable"]
    #[inline(always)]
    pub fn flitf_cken(&mut self) -> FLITF_CKEN_W {
        FLITF_CKEN_W { w: self }
    }
    #[doc = "Bit 6 - RCC clock enable"]
    #[inline(always)]
    pub fn rcc_cken(&mut self) -> RCC_CKEN_W {
        RCC_CKEN_W { w: self }
    }
    #[doc = "Bit 7 - EVTCL clock enable"]
    #[inline(always)]
    pub fn evtcl_cken(&mut self) -> EVTCL_CKEN_W {
        EVTCL_CKEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clocks gated enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgatenr](index.html) module"]
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
