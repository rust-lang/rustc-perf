#[doc = "Register `MACPMTCSR` reader"]
pub struct R(crate::R<MACPMTCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACPMTCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACPMTCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACPMTCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACPMTCSR` writer"]
pub struct W(crate::W<MACPMTCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACPMTCSR_SPEC>;
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
impl From<crate::W<MACPMTCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACPMTCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Power down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_A {
    #[doc = "1: All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received"]
    Enabled = 1,
}
impl From<PD_A> for bool {
    #[inline(always)]
    fn from(variant: PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD` reader - Power down"]
pub type PD_R = crate::BitReader<PD_A>;
impl PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PD_A> {
        match self.bits {
            true => Some(PD_A::Enabled),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PD_A::Enabled
    }
}
#[doc = "Field `PD` writer - Power down"]
pub type PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, PD_A, O>;
impl<'a, const O: u8> PD_W<'a, O> {
    #[doc = "All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD_A::Enabled)
    }
}
#[doc = "Magic packet enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPE_A {
    #[doc = "0: No power management event generated due to Magic Packet reception"]
    Disabled = 0,
    #[doc = "1: Enable generation of a power management event due to Magic Packet reception"]
    Enabled = 1,
}
impl From<MPE_A> for bool {
    #[inline(always)]
    fn from(variant: MPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPE` reader - Magic packet enable"]
pub type MPE_R = crate::BitReader<MPE_A>;
impl MPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPE_A {
        match self.bits {
            false => MPE_A::Disabled,
            true => MPE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MPE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MPE_A::Enabled
    }
}
#[doc = "Field `MPE` writer - Magic packet enable"]
pub type MPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, MPE_A, O>;
impl<'a, const O: u8> MPE_W<'a, O> {
    #[doc = "No power management event generated due to Magic Packet reception"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MPE_A::Disabled)
    }
    #[doc = "Enable generation of a power management event due to Magic Packet reception"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MPE_A::Enabled)
    }
}
#[doc = "Wakeup frame enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WFE_A {
    #[doc = "0: No power management event generated due to wakeup frame reception"]
    Disabled = 0,
    #[doc = "1: Enable generation of a power management event due to wakeup frame reception"]
    Enabled = 1,
}
impl From<WFE_A> for bool {
    #[inline(always)]
    fn from(variant: WFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WFE` reader - Wakeup frame enable"]
pub type WFE_R = crate::BitReader<WFE_A>;
impl WFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WFE_A {
        match self.bits {
            false => WFE_A::Disabled,
            true => WFE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WFE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WFE_A::Enabled
    }
}
#[doc = "Field `WFE` writer - Wakeup frame enable"]
pub type WFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, WFE_A, O>;
impl<'a, const O: u8> WFE_W<'a, O> {
    #[doc = "No power management event generated due to wakeup frame reception"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WFE_A::Disabled)
    }
    #[doc = "Enable generation of a power management event due to wakeup frame reception"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WFE_A::Enabled)
    }
}
#[doc = "Field `MPR` reader - Magic packet received"]
pub type MPR_R = crate::BitReader<bool>;
#[doc = "Field `MPR` writer - Magic packet received"]
pub type MPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, O>;
#[doc = "Field `WFR` reader - Wakeup frame received"]
pub type WFR_R = crate::BitReader<bool>;
#[doc = "Field `WFR` writer - Wakeup frame received"]
pub type WFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, O>;
#[doc = "Global unicast\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GU_A {
    #[doc = "0: Normal operation"]
    Disabled = 0,
    #[doc = "1: Any unicast packet filtered by the MAC address recognition may be a wakeup frame"]
    Enabled = 1,
}
impl From<GU_A> for bool {
    #[inline(always)]
    fn from(variant: GU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GU` reader - Global unicast"]
pub type GU_R = crate::BitReader<GU_A>;
impl GU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GU_A {
        match self.bits {
            false => GU_A::Disabled,
            true => GU_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GU_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GU_A::Enabled
    }
}
#[doc = "Field `GU` writer - Global unicast"]
pub type GU_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, GU_A, O>;
impl<'a, const O: u8> GU_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GU_A::Disabled)
    }
    #[doc = "Any unicast packet filtered by the MAC address recognition may be a wakeup frame"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GU_A::Enabled)
    }
}
#[doc = "Wakeup frame filter register pointer reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WFFRPR_A {
    #[doc = "1: Reset wakeup frame filter register point to 0b000. Automatically cleared"]
    Reset = 1,
}
impl From<WFFRPR_A> for bool {
    #[inline(always)]
    fn from(variant: WFFRPR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WFFRPR` reader - Wakeup frame filter register pointer reset"]
pub type WFFRPR_R = crate::BitReader<WFFRPR_A>;
impl WFFRPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WFFRPR_A> {
        match self.bits {
            true => Some(WFFRPR_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WFFRPR_A::Reset
    }
}
#[doc = "Field `WFFRPR` writer - Wakeup frame filter register pointer reset"]
pub type WFFRPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, WFFRPR_A, O>;
impl<'a, const O: u8> WFFRPR_W<'a, O> {
    #[doc = "Reset wakeup frame filter register point to 0b000. Automatically cleared"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(WFFRPR_A::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Magic packet enable"]
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup frame enable"]
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Magic packet received"]
    #[inline(always)]
    pub fn mpr(&self) -> MPR_R {
        MPR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup frame received"]
    #[inline(always)]
    pub fn wfr(&self) -> WFR_R {
        WFR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    pub fn gu(&self) -> GU_R {
        GU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Wakeup frame filter register pointer reset"]
    #[inline(always)]
    pub fn wffrpr(&self) -> WFFRPR_R {
        WFFRPR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W<0> {
        PD_W::new(self)
    }
    #[doc = "Bit 1 - Magic packet enable"]
    #[inline(always)]
    pub fn mpe(&mut self) -> MPE_W<1> {
        MPE_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup frame enable"]
    #[inline(always)]
    pub fn wfe(&mut self) -> WFE_W<2> {
        WFE_W::new(self)
    }
    #[doc = "Bit 5 - Magic packet received"]
    #[inline(always)]
    pub fn mpr(&mut self) -> MPR_W<5> {
        MPR_W::new(self)
    }
    #[doc = "Bit 6 - Wakeup frame received"]
    #[inline(always)]
    pub fn wfr(&mut self) -> WFR_W<6> {
        WFR_W::new(self)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    pub fn gu(&mut self) -> GU_W<9> {
        GU_W::new(self)
    }
    #[doc = "Bit 31 - Wakeup frame filter register pointer reset"]
    #[inline(always)]
    pub fn wffrpr(&mut self) -> WFFRPR_W<31> {
        WFFRPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC PMT control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macpmtcsr](index.html) module"]
pub struct MACPMTCSR_SPEC;
impl crate::RegisterSpec for MACPMTCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macpmtcsr::R](R) reader structure"]
impl crate::Readable for MACPMTCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macpmtcsr::W](W) writer structure"]
impl crate::Writable for MACPMTCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACPMTCSR to value 0"]
impl crate::Resettable for MACPMTCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
