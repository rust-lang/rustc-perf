#[doc = "Reader of register MACPMTCSR"]
pub type R = crate::R<u32, super::MACPMTCSR>;
#[doc = "Writer for register MACPMTCSR"]
pub type W = crate::W<u32, super::MACPMTCSR>;
#[doc = "Register MACPMTCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACPMTCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Power down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PD_A {
    #[doc = "1: All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received"]
    ENABLED = 1,
}
impl From<PD_A> for bool {
    #[inline(always)]
    fn from(variant: PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PD`"]
pub type PD_R = crate::R<bool, PD_A>;
impl PD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PD_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PD_A::ENABLED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PD_A::ENABLED
    }
}
#[doc = "Write proxy for field `PD`"]
pub struct PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Magic packet enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPE_A {
    #[doc = "0: No power management event generated due to Magic Packet reception"]
    DISABLED = 0,
    #[doc = "1: Enable generation of a power management event due to Magic Packet reception"]
    ENABLED = 1,
}
impl From<MPE_A> for bool {
    #[inline(always)]
    fn from(variant: MPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPE`"]
pub type MPE_R = crate::R<bool, MPE_A>;
impl MPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPE_A {
        match self.bits {
            false => MPE_A::DISABLED,
            true => MPE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MPE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MPE_A::ENABLED
    }
}
#[doc = "Write proxy for field `MPE`"]
pub struct MPE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No power management event generated due to Magic Packet reception"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MPE_A::DISABLED)
    }
    #[doc = "Enable generation of a power management event due to Magic Packet reception"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MPE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Wakeup frame enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WFE_A {
    #[doc = "0: No power management event generated due to wakeup frame reception"]
    DISABLED = 0,
    #[doc = "1: Enable generation of a power management event due to wakeup frame reception"]
    ENABLED = 1,
}
impl From<WFE_A> for bool {
    #[inline(always)]
    fn from(variant: WFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WFE`"]
pub type WFE_R = crate::R<bool, WFE_A>;
impl WFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WFE_A {
        match self.bits {
            false => WFE_A::DISABLED,
            true => WFE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WFE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WFE_A::ENABLED
    }
}
#[doc = "Write proxy for field `WFE`"]
pub struct WFE_W<'a> {
    w: &'a mut W,
}
impl<'a> WFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No power management event generated due to wakeup frame reception"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WFE_A::DISABLED)
    }
    #[doc = "Enable generation of a power management event due to wakeup frame reception"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WFE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `MPR`"]
pub type MPR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPR`"]
pub struct MPR_W<'a> {
    w: &'a mut W,
}
impl<'a> MPR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `WFR`"]
pub type WFR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WFR`"]
pub struct WFR_W<'a> {
    w: &'a mut W,
}
impl<'a> WFR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Global unicast\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GU_A {
    #[doc = "0: Normal operation"]
    DISABLED = 0,
    #[doc = "1: Any unicast packet filtered by the MAC address recognition may be a wakeup frame"]
    ENABLED = 1,
}
impl From<GU_A> for bool {
    #[inline(always)]
    fn from(variant: GU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GU`"]
pub type GU_R = crate::R<bool, GU_A>;
impl GU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GU_A {
        match self.bits {
            false => GU_A::DISABLED,
            true => GU_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GU_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GU_A::ENABLED
    }
}
#[doc = "Write proxy for field `GU`"]
pub struct GU_W<'a> {
    w: &'a mut W,
}
impl<'a> GU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GU_A::DISABLED)
    }
    #[doc = "Any unicast packet filtered by the MAC address recognition may be a wakeup frame"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GU_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Wakeup frame filter register pointer reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WFFRPR_A {
    #[doc = "1: Reset wakeup frame filter register point to 0b000. Automatically cleared"]
    RESET = 1,
}
impl From<WFFRPR_A> for bool {
    #[inline(always)]
    fn from(variant: WFFRPR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WFFRPR`"]
pub type WFFRPR_R = crate::R<bool, WFFRPR_A>;
impl WFFRPR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, WFFRPR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(WFFRPR_A::RESET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WFFRPR_A::RESET
    }
}
#[doc = "Write proxy for field `WFFRPR`"]
pub struct WFFRPR_W<'a> {
    w: &'a mut W,
}
impl<'a> WFFRPR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WFFRPR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset wakeup frame filter register point to 0b000. Automatically cleared"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(WFFRPR_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Magic packet enable"]
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup frame enable"]
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Magic packet received"]
    #[inline(always)]
    pub fn mpr(&self) -> MPR_R {
        MPR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wakeup frame received"]
    #[inline(always)]
    pub fn wfr(&self) -> WFR_R {
        WFR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    pub fn gu(&self) -> GU_R {
        GU_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Wakeup frame filter register pointer reset"]
    #[inline(always)]
    pub fn wffrpr(&self) -> WFFRPR_R {
        WFFRPR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W { w: self }
    }
    #[doc = "Bit 1 - Magic packet enable"]
    #[inline(always)]
    pub fn mpe(&mut self) -> MPE_W {
        MPE_W { w: self }
    }
    #[doc = "Bit 2 - Wakeup frame enable"]
    #[inline(always)]
    pub fn wfe(&mut self) -> WFE_W {
        WFE_W { w: self }
    }
    #[doc = "Bit 5 - Magic packet received"]
    #[inline(always)]
    pub fn mpr(&mut self) -> MPR_W {
        MPR_W { w: self }
    }
    #[doc = "Bit 6 - Wakeup frame received"]
    #[inline(always)]
    pub fn wfr(&mut self) -> WFR_W {
        WFR_W { w: self }
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    pub fn gu(&mut self) -> GU_W {
        GU_W { w: self }
    }
    #[doc = "Bit 31 - Wakeup frame filter register pointer reset"]
    #[inline(always)]
    pub fn wffrpr(&mut self) -> WFFRPR_W {
        WFFRPR_W { w: self }
    }
}
