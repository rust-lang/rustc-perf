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
    ENABLED = 1,
}
impl From<PD_A> for bool {
    #[inline(always)]
    fn from(variant: PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD` reader - Power down"]
pub struct PD_R(crate::FieldReader<bool, PD_A>);
impl PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PD_A> {
        match self.bits {
            true => Some(PD_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PD_A::ENABLED
    }
}
impl core::ops::Deref for PD_R {
    type Target = crate::FieldReader<bool, PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD` writer - Power down"]
pub struct PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
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
#[doc = "Field `MPE` reader - Magic packet enable"]
pub struct MPE_R(crate::FieldReader<bool, MPE_A>);
impl MPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPE_R(crate::FieldReader::new(bits))
    }
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
        **self == MPE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MPE_A::ENABLED
    }
}
impl core::ops::Deref for MPE_R {
    type Target = crate::FieldReader<bool, MPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPE` writer - Magic packet enable"]
pub struct MPE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPE_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
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
#[doc = "Field `WFE` reader - Wakeup frame enable"]
pub struct WFE_R(crate::FieldReader<bool, WFE_A>);
impl WFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WFE_R(crate::FieldReader::new(bits))
    }
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
        **self == WFE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WFE_A::ENABLED
    }
}
impl core::ops::Deref for WFE_R {
    type Target = crate::FieldReader<bool, WFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WFE` writer - Wakeup frame enable"]
pub struct WFE_W<'a> {
    w: &'a mut W,
}
impl<'a> WFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WFE_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `MPR` reader - Magic packet received"]
pub struct MPR_R(crate::FieldReader<bool, bool>);
impl MPR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPR` writer - Magic packet received"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `WFR` reader - Wakeup frame received"]
pub struct WFR_R(crate::FieldReader<bool, bool>);
impl WFR_R {
    pub(crate) fn new(bits: bool) -> Self {
        WFR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WFR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WFR` writer - Wakeup frame received"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
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
#[doc = "Field `GU` reader - Global unicast"]
pub struct GU_R(crate::FieldReader<bool, GU_A>);
impl GU_R {
    pub(crate) fn new(bits: bool) -> Self {
        GU_R(crate::FieldReader::new(bits))
    }
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
        **self == GU_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == GU_A::ENABLED
    }
}
impl core::ops::Deref for GU_R {
    type Target = crate::FieldReader<bool, GU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GU` writer - Global unicast"]
pub struct GU_W<'a> {
    w: &'a mut W,
}
impl<'a> GU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GU_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
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
#[doc = "Field `WFFRPR` reader - Wakeup frame filter register pointer reset"]
pub struct WFFRPR_R(crate::FieldReader<bool, WFFRPR_A>);
impl WFFRPR_R {
    pub(crate) fn new(bits: bool) -> Self {
        WFFRPR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WFFRPR_A> {
        match self.bits {
            true => Some(WFFRPR_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == WFFRPR_A::RESET
    }
}
impl core::ops::Deref for WFFRPR_R {
    type Target = crate::FieldReader<bool, WFFRPR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WFFRPR` writer - Wakeup frame filter register pointer reset"]
pub struct WFFRPR_W<'a> {
    w: &'a mut W,
}
impl<'a> WFFRPR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WFFRPR_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
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
