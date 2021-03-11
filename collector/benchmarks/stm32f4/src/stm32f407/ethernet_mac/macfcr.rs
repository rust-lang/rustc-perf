#[doc = "Reader of register MACFCR"]
pub type R = crate::R<u32, super::MACFCR>;
#[doc = "Writer for register MACFCR"]
pub type W = crate::W<u32, super::MACFCR>;
#[doc = "Register MACFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `FCB`"]
pub type FCB_R = crate::R<bool, FCB_A>;
impl FCB_R {
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
        *self == FCB_A::PAUSEORBACKPRESSURE
    }
    #[doc = "Checks if the value of the field is `DISABLEBACKPRESSURE`"]
    #[inline(always)]
    pub fn is_disable_back_pressure(&self) -> bool {
        *self == FCB_A::DISABLEBACKPRESSURE
    }
}
#[doc = "Write proxy for field `FCB`"]
pub struct FCB_W<'a> {
    w: &'a mut W,
}
impl<'a> FCB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
#[doc = "Reader of field `TFCE`"]
pub type TFCE_R = crate::R<bool, TFCE_A>;
impl TFCE_R {
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
        *self == TFCE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TFCE_A::ENABLED
    }
}
#[doc = "Write proxy for field `TFCE`"]
pub struct TFCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
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
#[doc = "Reader of field `RFCE`"]
pub type RFCE_R = crate::R<bool, RFCE_A>;
impl RFCE_R {
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
        *self == RFCE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RFCE_A::ENABLED
    }
}
#[doc = "Write proxy for field `RFCE`"]
pub struct RFCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
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
#[doc = "Reader of field `UPFD`"]
pub type UPFD_R = crate::R<bool, UPFD_A>;
impl UPFD_R {
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
        *self == UPFD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPFD_A::ENABLED
    }
}
#[doc = "Write proxy for field `UPFD`"]
pub struct UPFD_W<'a> {
    w: &'a mut W,
}
impl<'a> UPFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPFD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
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
#[doc = "Reader of field `PLT`"]
pub type PLT_R = crate::R<u8, PLT_A>;
impl PLT_R {
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
        *self == PLT_A::PLT4
    }
    #[doc = "Checks if the value of the field is `PLT28`"]
    #[inline(always)]
    pub fn is_plt28(&self) -> bool {
        *self == PLT_A::PLT28
    }
    #[doc = "Checks if the value of the field is `PLT144`"]
    #[inline(always)]
    pub fn is_plt144(&self) -> bool {
        *self == PLT_A::PLT144
    }
    #[doc = "Checks if the value of the field is `PLT256`"]
    #[inline(always)]
    pub fn is_plt256(&self) -> bool {
        *self == PLT_A::PLT256
    }
}
#[doc = "Write proxy for field `PLT`"]
pub struct PLT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
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
#[doc = "Reader of field `ZQPD`"]
pub type ZQPD_R = crate::R<bool, ZQPD_A>;
impl ZQPD_R {
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
        *self == ZQPD_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ZQPD_A::DISABLED
    }
}
#[doc = "Write proxy for field `ZQPD`"]
pub struct ZQPD_W<'a> {
    w: &'a mut W,
}
impl<'a> ZQPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZQPD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PT`"]
pub type PT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PT`"]
pub struct PT_W<'a> {
    w: &'a mut W,
}
impl<'a> PT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
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
}
