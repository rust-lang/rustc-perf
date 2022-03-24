#[doc = "Register `MACFCR` reader"]
pub struct R(crate::R<MACFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACFCR` writer"]
pub struct W(crate::W<MACFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACFCR_SPEC>;
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
impl From<crate::W<MACFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Flow control busy/back pressure activate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCB_A {
    #[doc = "1: In full duplex, initiate a Pause control frame. In half duplex, assert back pressure"]
    PAUSEORBACKPRESSURE = 1,
    #[doc = "0: In half duplex only, deasserts back pressure"]
    DISABLEBACKPRESSURE = 0,
}
impl From<FCB_A> for bool {
    #[inline(always)]
    fn from(variant: FCB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCB` reader - Flow control busy/back pressure activate"]
pub struct FCB_R(crate::FieldReader<bool, FCB_A>);
impl FCB_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCB_A {
        match self.bits {
            true => FCB_A::PAUSEORBACKPRESSURE,
            false => FCB_A::DISABLEBACKPRESSURE,
        }
    }
    #[doc = "Checks if the value of the field is `PAUSEORBACKPRESSURE`"]
    #[inline(always)]
    pub fn is_pause_or_back_pressure(&self) -> bool {
        **self == FCB_A::PAUSEORBACKPRESSURE
    }
    #[doc = "Checks if the value of the field is `DISABLEBACKPRESSURE`"]
    #[inline(always)]
    pub fn is_disable_back_pressure(&self) -> bool {
        **self == FCB_A::DISABLEBACKPRESSURE
    }
}
impl core::ops::Deref for FCB_R {
    type Target = crate::FieldReader<bool, FCB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCB` writer - Flow control busy/back pressure activate"]
pub struct FCB_W<'a> {
    w: &'a mut W,
}
impl<'a> FCB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "In full duplex, initiate a Pause control frame. In half duplex, assert back pressure"]
    #[inline(always)]
    pub fn pause_or_back_pressure(self) -> &'a mut W {
        self.variant(FCB_A::PAUSEORBACKPRESSURE)
    }
    #[doc = "In half duplex only, deasserts back pressure"]
    #[inline(always)]
    pub fn disable_back_pressure(self) -> &'a mut W {
        self.variant(FCB_A::DISABLEBACKPRESSURE)
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
#[doc = "Transmit flow control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFCE_A {
    #[doc = "0: In full duplex, flow control is disabled. In half duplex, back pressure is disabled"]
    DISABLED = 0,
    #[doc = "1: In full duplex, flow control is enabled. In half duplex, back pressure is enabled"]
    ENABLED = 1,
}
impl From<TFCE_A> for bool {
    #[inline(always)]
    fn from(variant: TFCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFCE` reader - Transmit flow control enable"]
pub struct TFCE_R(crate::FieldReader<bool, TFCE_A>);
impl TFCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFCE_A {
        match self.bits {
            false => TFCE_A::DISABLED,
            true => TFCE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TFCE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TFCE_A::ENABLED
    }
}
impl core::ops::Deref for TFCE_R {
    type Target = crate::FieldReader<bool, TFCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFCE` writer - Transmit flow control enable"]
pub struct TFCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "In full duplex, flow control is disabled. In half duplex, back pressure is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TFCE_A::DISABLED)
    }
    #[doc = "In full duplex, flow control is enabled. In half duplex, back pressure is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TFCE_A::ENABLED)
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
#[doc = "Receive flow control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFCE_A {
    #[doc = "0: Pause frames are not decoded"]
    DISABLED = 0,
    #[doc = "1: MAC decodes received Pause frames and disables its transmitted for a specified time"]
    ENABLED = 1,
}
impl From<RFCE_A> for bool {
    #[inline(always)]
    fn from(variant: RFCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFCE` reader - Receive flow control enable"]
pub struct RFCE_R(crate::FieldReader<bool, RFCE_A>);
impl RFCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFCE_A {
        match self.bits {
            false => RFCE_A::DISABLED,
            true => RFCE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RFCE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RFCE_A::ENABLED
    }
}
impl core::ops::Deref for RFCE_R {
    type Target = crate::FieldReader<bool, RFCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFCE` writer - Receive flow control enable"]
pub struct RFCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pause frames are not decoded"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RFCE_A::DISABLED)
    }
    #[doc = "MAC decodes received Pause frames and disables its transmitted for a specified time"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RFCE_A::ENABLED)
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
#[doc = "Unicast pause frame detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPFD_A {
    #[doc = "0: MAC detects only a Pause frame with the multicast address specified in the 802.3x standard"]
    DISABLED = 0,
    #[doc = "1: MAC additionally detects Pause frames with the station's unicast address"]
    ENABLED = 1,
}
impl From<UPFD_A> for bool {
    #[inline(always)]
    fn from(variant: UPFD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPFD` reader - Unicast pause frame detect"]
pub struct UPFD_R(crate::FieldReader<bool, UPFD_A>);
impl UPFD_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPFD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPFD_A {
        match self.bits {
            false => UPFD_A::DISABLED,
            true => UPFD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == UPFD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == UPFD_A::ENABLED
    }
}
impl core::ops::Deref for UPFD_R {
    type Target = crate::FieldReader<bool, UPFD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPFD` writer - Unicast pause frame detect"]
pub struct UPFD_W<'a> {
    w: &'a mut W,
}
impl<'a> UPFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPFD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MAC detects only a Pause frame with the multicast address specified in the 802.3x standard"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPFD_A::DISABLED)
    }
    #[doc = "MAC additionally detects Pause frames with the station's unicast address"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UPFD_A::ENABLED)
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
#[doc = "Pause low threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLT_A {
    #[doc = "0: Pause time minus 4 slot times"]
    PLT4 = 0,
    #[doc = "1: Pause time minus 28 slot times"]
    PLT28 = 1,
    #[doc = "2: Pause time minus 144 slot times"]
    PLT144 = 2,
    #[doc = "3: Pause time minus 256 slot times"]
    PLT256 = 3,
}
impl From<PLT_A> for u8 {
    #[inline(always)]
    fn from(variant: PLT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLT` reader - Pause low threshold"]
pub struct PLT_R(crate::FieldReader<u8, PLT_A>);
impl PLT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLT_A {
        match self.bits {
            0 => PLT_A::PLT4,
            1 => PLT_A::PLT28,
            2 => PLT_A::PLT144,
            3 => PLT_A::PLT256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PLT4`"]
    #[inline(always)]
    pub fn is_plt4(&self) -> bool {
        **self == PLT_A::PLT4
    }
    #[doc = "Checks if the value of the field is `PLT28`"]
    #[inline(always)]
    pub fn is_plt28(&self) -> bool {
        **self == PLT_A::PLT28
    }
    #[doc = "Checks if the value of the field is `PLT144`"]
    #[inline(always)]
    pub fn is_plt144(&self) -> bool {
        **self == PLT_A::PLT144
    }
    #[doc = "Checks if the value of the field is `PLT256`"]
    #[inline(always)]
    pub fn is_plt256(&self) -> bool {
        **self == PLT_A::PLT256
    }
}
impl core::ops::Deref for PLT_R {
    type Target = crate::FieldReader<u8, PLT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLT` writer - Pause low threshold"]
pub struct PLT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Pause time minus 4 slot times"]
    #[inline(always)]
    pub fn plt4(self) -> &'a mut W {
        self.variant(PLT_A::PLT4)
    }
    #[doc = "Pause time minus 28 slot times"]
    #[inline(always)]
    pub fn plt28(self) -> &'a mut W {
        self.variant(PLT_A::PLT28)
    }
    #[doc = "Pause time minus 144 slot times"]
    #[inline(always)]
    pub fn plt144(self) -> &'a mut W {
        self.variant(PLT_A::PLT144)
    }
    #[doc = "Pause time minus 256 slot times"]
    #[inline(always)]
    pub fn plt256(self) -> &'a mut W {
        self.variant(PLT_A::PLT256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Zero-quanta pause disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZQPD_A {
    #[doc = "0: Normal operation with automatic zero-quanta pause control frame generation"]
    ENABLED = 0,
    #[doc = "1: Automatic generation of zero-quanta pause control frames is disabled"]
    DISABLED = 1,
}
impl From<ZQPD_A> for bool {
    #[inline(always)]
    fn from(variant: ZQPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ZQPD` reader - Zero-quanta pause disable"]
pub struct ZQPD_R(crate::FieldReader<bool, ZQPD_A>);
impl ZQPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZQPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZQPD_A {
        match self.bits {
            false => ZQPD_A::ENABLED,
            true => ZQPD_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ZQPD_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ZQPD_A::DISABLED
    }
}
impl core::ops::Deref for ZQPD_R {
    type Target = crate::FieldReader<bool, ZQPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZQPD` writer - Zero-quanta pause disable"]
pub struct ZQPD_W<'a> {
    w: &'a mut W,
}
impl<'a> ZQPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZQPD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal operation with automatic zero-quanta pause control frame generation"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ZQPD_A::ENABLED)
    }
    #[doc = "Automatic generation of zero-quanta pause control frames is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ZQPD_A::DISABLED)
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
#[doc = "Field `PT` reader - Pause time"]
pub struct PT_R(crate::FieldReader<u16, u16>);
impl PT_R {
    pub(crate) fn new(bits: u16) -> Self {
        PT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PT` writer - Pause time"]
pub struct PT_W<'a> {
    w: &'a mut W,
}
impl<'a> PT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flow control busy/back pressure activate"]
    #[inline(always)]
    pub fn fcb(&self) -> FCB_R {
        FCB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit flow control enable"]
    #[inline(always)]
    pub fn tfce(&self) -> TFCE_R {
        TFCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive flow control enable"]
    #[inline(always)]
    pub fn rfce(&self) -> RFCE_R {
        RFCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Unicast pause frame detect"]
    #[inline(always)]
    pub fn upfd(&self) -> UPFD_R {
        UPFD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Pause low threshold"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Zero-quanta pause disable"]
    #[inline(always)]
    pub fn zqpd(&self) -> ZQPD_R {
        ZQPD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Pause time"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow control busy/back pressure activate"]
    #[inline(always)]
    pub fn fcb(&mut self) -> FCB_W {
        FCB_W { w: self }
    }
    #[doc = "Bit 1 - Transmit flow control enable"]
    #[inline(always)]
    pub fn tfce(&mut self) -> TFCE_W {
        TFCE_W { w: self }
    }
    #[doc = "Bit 2 - Receive flow control enable"]
    #[inline(always)]
    pub fn rfce(&mut self) -> RFCE_W {
        RFCE_W { w: self }
    }
    #[doc = "Bit 3 - Unicast pause frame detect"]
    #[inline(always)]
    pub fn upfd(&mut self) -> UPFD_W {
        UPFD_W { w: self }
    }
    #[doc = "Bits 4:5 - Pause low threshold"]
    #[inline(always)]
    pub fn plt(&mut self) -> PLT_W {
        PLT_W { w: self }
    }
    #[doc = "Bit 7 - Zero-quanta pause disable"]
    #[inline(always)]
    pub fn zqpd(&mut self) -> ZQPD_W {
        ZQPD_W { w: self }
    }
    #[doc = "Bits 16:31 - Pause time"]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W {
        PT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC flow control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macfcr](index.html) module"]
pub struct MACFCR_SPEC;
impl crate::RegisterSpec for MACFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macfcr::R](R) reader structure"]
impl crate::Readable for MACFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macfcr::W](W) writer structure"]
impl crate::Writable for MACFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACFCR to value 0"]
impl crate::Resettable for MACFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
