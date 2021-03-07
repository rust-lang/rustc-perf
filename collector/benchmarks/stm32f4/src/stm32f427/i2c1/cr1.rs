#[doc = "Reader of register CR1"]
pub type R = crate::R<u32, super::CR1>;
#[doc = "Writer for register CR1"]
pub type W = crate::W<u32, super::CR1>;
#[doc = "Register CR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRST_A {
    #[doc = "0: I2C peripheral not under reset"]
    NOTRESET = 0,
    #[doc = "1: I2C peripheral under reset"]
    RESET = 1,
}
impl From<SWRST_A> for bool {
    #[inline(always)]
    fn from(variant: SWRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, SWRST_A>;
impl SWRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRST_A {
        match self.bits {
            false => SWRST_A::NOTRESET,
            true => SWRST_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRESET`"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == SWRST_A::NOTRESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SWRST_A::RESET
    }
}
#[doc = "Write proxy for field `SWRST`"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "I2C peripheral not under reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(SWRST_A::NOTRESET)
    }
    #[doc = "I2C peripheral under reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SWRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "SMBus alert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALERT_A {
    #[doc = "0: SMBA pin released high"]
    RELEASE = 0,
    #[doc = "1: SMBA pin driven low"]
    DRIVE = 1,
}
impl From<ALERT_A> for bool {
    #[inline(always)]
    fn from(variant: ALERT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALERT`"]
pub type ALERT_R = crate::R<bool, ALERT_A>;
impl ALERT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALERT_A {
        match self.bits {
            false => ALERT_A::RELEASE,
            true => ALERT_A::DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASE`"]
    #[inline(always)]
    pub fn is_release(&self) -> bool {
        *self == ALERT_A::RELEASE
    }
    #[doc = "Checks if the value of the field is `DRIVE`"]
    #[inline(always)]
    pub fn is_drive(&self) -> bool {
        *self == ALERT_A::DRIVE
    }
}
#[doc = "Write proxy for field `ALERT`"]
pub struct ALERT_W<'a> {
    w: &'a mut W,
}
impl<'a> ALERT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALERT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SMBA pin released high"]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(ALERT_A::RELEASE)
    }
    #[doc = "SMBA pin driven low"]
    #[inline(always)]
    pub fn drive(self) -> &'a mut W {
        self.variant(ALERT_A::DRIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Packet error checking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEC_A {
    #[doc = "0: No PEC transfer"]
    DISABLED = 0,
    #[doc = "1: PEC transfer"]
    ENABLED = 1,
}
impl From<PEC_A> for bool {
    #[inline(always)]
    fn from(variant: PEC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEC`"]
pub type PEC_R = crate::R<bool, PEC_A>;
impl PEC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEC_A {
        match self.bits {
            false => PEC_A::DISABLED,
            true => PEC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PEC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PEC_A::ENABLED
    }
}
#[doc = "Write proxy for field `PEC`"]
pub struct PEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No PEC transfer"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PEC_A::DISABLED)
    }
    #[doc = "PEC transfer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PEC_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Acknowledge/PEC Position (for data reception)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POS_A {
    #[doc = "0: ACK bit controls the (N)ACK of the current byte being received"]
    CURRENT = 0,
    #[doc = "1: ACK bit controls the (N)ACK of the next byte to be received"]
    NEXT = 1,
}
impl From<POS_A> for bool {
    #[inline(always)]
    fn from(variant: POS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POS`"]
pub type POS_R = crate::R<bool, POS_A>;
impl POS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POS_A {
        match self.bits {
            false => POS_A::CURRENT,
            true => POS_A::NEXT,
        }
    }
    #[doc = "Checks if the value of the field is `CURRENT`"]
    #[inline(always)]
    pub fn is_current(&self) -> bool {
        *self == POS_A::CURRENT
    }
    #[doc = "Checks if the value of the field is `NEXT`"]
    #[inline(always)]
    pub fn is_next(&self) -> bool {
        *self == POS_A::NEXT
    }
}
#[doc = "Write proxy for field `POS`"]
pub struct POS_W<'a> {
    w: &'a mut W,
}
impl<'a> POS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ACK bit controls the (N)ACK of the current byte being received"]
    #[inline(always)]
    pub fn current(self) -> &'a mut W {
        self.variant(POS_A::CURRENT)
    }
    #[doc = "ACK bit controls the (N)ACK of the next byte to be received"]
    #[inline(always)]
    pub fn next(self) -> &'a mut W {
        self.variant(POS_A::NEXT)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Acknowledge enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACK_A {
    #[doc = "0: No acknowledge returned"]
    NAK = 0,
    #[doc = "1: Acknowledge returned after a byte is received"]
    ACK = 1,
}
impl From<ACK_A> for bool {
    #[inline(always)]
    fn from(variant: ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACK`"]
pub type ACK_R = crate::R<bool, ACK_A>;
impl ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACK_A {
        match self.bits {
            false => ACK_A::NAK,
            true => ACK_A::ACK,
        }
    }
    #[doc = "Checks if the value of the field is `NAK`"]
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == ACK_A::NAK
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == ACK_A::ACK
    }
}
#[doc = "Write proxy for field `ACK`"]
pub struct ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No acknowledge returned"]
    #[inline(always)]
    pub fn nak(self) -> &'a mut W {
        self.variant(ACK_A::NAK)
    }
    #[doc = "Acknowledge returned after a byte is received"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(ACK_A::ACK)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Stop generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_A {
    #[doc = "0: No Stop generation"]
    NOSTOP = 0,
    #[doc = "1: In master mode: stop generation after current byte/start, in slave mode: release SCL and SDA after current byte"]
    STOP = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STOP`"]
pub type STOP_R = crate::R<bool, STOP_A>;
impl STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::NOSTOP,
            true => STOP_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTOP`"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOP_A::NOSTOP
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOP_A::STOP
    }
}
#[doc = "Write proxy for field `STOP`"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Stop generation"]
    #[inline(always)]
    pub fn no_stop(self) -> &'a mut W {
        self.variant(STOP_A::NOSTOP)
    }
    #[doc = "In master mode: stop generation after current byte/start, in slave mode: release SCL and SDA after current byte"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOP_A::STOP)
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
#[doc = "Start generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_A {
    #[doc = "0: No Start generation"]
    NOSTART = 0,
    #[doc = "1: In master mode: repeated start generation, in slave mode: start generation when bus is free"]
    START = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, START_A>;
impl START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::NOSTART,
            true => START_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTART`"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == START_A::NOSTART
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::START
    }
}
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Start generation"]
    #[inline(always)]
    pub fn no_start(self) -> &'a mut W {
        self.variant(START_A::NOSTART)
    }
    #[doc = "In master mode: repeated start generation, in slave mode: start generation when bus is free"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Clock stretching disable (Slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOSTRETCH_A {
    #[doc = "0: Clock stretching enabled"]
    ENABLED = 0,
    #[doc = "1: Clock stretching disabled"]
    DISABLED = 1,
}
impl From<NOSTRETCH_A> for bool {
    #[inline(always)]
    fn from(variant: NOSTRETCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NOSTRETCH`"]
pub type NOSTRETCH_R = crate::R<bool, NOSTRETCH_A>;
impl NOSTRETCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOSTRETCH_A {
        match self.bits {
            false => NOSTRETCH_A::ENABLED,
            true => NOSTRETCH_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NOSTRETCH_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NOSTRETCH_A::DISABLED
    }
}
#[doc = "Write proxy for field `NOSTRETCH`"]
pub struct NOSTRETCH_W<'a> {
    w: &'a mut W,
}
impl<'a> NOSTRETCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOSTRETCH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock stretching enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NOSTRETCH_A::ENABLED)
    }
    #[doc = "Clock stretching disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NOSTRETCH_A::DISABLED)
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
#[doc = "General call enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENGC_A {
    #[doc = "0: General call disabled"]
    DISABLED = 0,
    #[doc = "1: General call enabled"]
    ENABLED = 1,
}
impl From<ENGC_A> for bool {
    #[inline(always)]
    fn from(variant: ENGC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENGC`"]
pub type ENGC_R = crate::R<bool, ENGC_A>;
impl ENGC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENGC_A {
        match self.bits {
            false => ENGC_A::DISABLED,
            true => ENGC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENGC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENGC_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENGC`"]
pub struct ENGC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENGC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENGC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "General call disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENGC_A::DISABLED)
    }
    #[doc = "General call enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENGC_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "PEC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENPEC_A {
    #[doc = "0: PEC calculation disabled"]
    DISABLED = 0,
    #[doc = "1: PEC calculation enabled"]
    ENABLED = 1,
}
impl From<ENPEC_A> for bool {
    #[inline(always)]
    fn from(variant: ENPEC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENPEC`"]
pub type ENPEC_R = crate::R<bool, ENPEC_A>;
impl ENPEC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENPEC_A {
        match self.bits {
            false => ENPEC_A::DISABLED,
            true => ENPEC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENPEC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENPEC_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENPEC`"]
pub struct ENPEC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENPEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENPEC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PEC calculation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENPEC_A::DISABLED)
    }
    #[doc = "PEC calculation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENPEC_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "ARP enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENARP_A {
    #[doc = "0: ARP disabled"]
    DISABLED = 0,
    #[doc = "1: ARP enabled"]
    ENABLED = 1,
}
impl From<ENARP_A> for bool {
    #[inline(always)]
    fn from(variant: ENARP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENARP`"]
pub type ENARP_R = crate::R<bool, ENARP_A>;
impl ENARP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENARP_A {
        match self.bits {
            false => ENARP_A::DISABLED,
            true => ENARP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENARP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENARP_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENARP`"]
pub struct ENARP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENARP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENARP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ARP disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENARP_A::DISABLED)
    }
    #[doc = "ARP enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENARP_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "SMBus type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBTYPE_A {
    #[doc = "0: SMBus Device"]
    DEVICE = 0,
    #[doc = "1: SMBus Host"]
    HOST = 1,
}
impl From<SMBTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: SMBTYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMBTYPE`"]
pub type SMBTYPE_R = crate::R<bool, SMBTYPE_A>;
impl SMBTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMBTYPE_A {
        match self.bits {
            false => SMBTYPE_A::DEVICE,
            true => SMBTYPE_A::HOST,
        }
    }
    #[doc = "Checks if the value of the field is `DEVICE`"]
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        *self == SMBTYPE_A::DEVICE
    }
    #[doc = "Checks if the value of the field is `HOST`"]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == SMBTYPE_A::HOST
    }
}
#[doc = "Write proxy for field `SMBTYPE`"]
pub struct SMBTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMBTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMBTYPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SMBus Device"]
    #[inline(always)]
    pub fn device(self) -> &'a mut W {
        self.variant(SMBTYPE_A::DEVICE)
    }
    #[doc = "SMBus Host"]
    #[inline(always)]
    pub fn host(self) -> &'a mut W {
        self.variant(SMBTYPE_A::HOST)
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
#[doc = "SMBus mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBUS_A {
    #[doc = "0: I2C Mode"]
    I2C = 0,
    #[doc = "1: SMBus"]
    SMBUS = 1,
}
impl From<SMBUS_A> for bool {
    #[inline(always)]
    fn from(variant: SMBUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMBUS`"]
pub type SMBUS_R = crate::R<bool, SMBUS_A>;
impl SMBUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMBUS_A {
        match self.bits {
            false => SMBUS_A::I2C,
            true => SMBUS_A::SMBUS,
        }
    }
    #[doc = "Checks if the value of the field is `I2C`"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == SMBUS_A::I2C
    }
    #[doc = "Checks if the value of the field is `SMBUS`"]
    #[inline(always)]
    pub fn is_smbus(&self) -> bool {
        *self == SMBUS_A::SMBUS
    }
}
#[doc = "Write proxy for field `SMBUS`"]
pub struct SMBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SMBUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMBUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "I2C Mode"]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut W {
        self.variant(SMBUS_A::I2C)
    }
    #[doc = "SMBus"]
    #[inline(always)]
    pub fn smbus(self) -> &'a mut W {
        self.variant(SMBUS_A::SMBUS)
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
#[doc = "Peripheral enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PE_A {
    #[doc = "0: Peripheral disabled"]
    DISABLED = 0,
    #[doc = "1: Peripheral enabled"]
    ENABLED = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, PE_A>;
impl PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::DISABLED,
            true => PE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PE_A::ENABLED
    }
}
#[doc = "Write proxy for field `PE`"]
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PE_A::DISABLED)
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PE_A::ENABLED)
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
impl R {
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline(always)]
    pub fn pos(&self) -> POS_R {
        POS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Clock stretching disable (Slave mode)"]
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline(always)]
    pub fn engc(&self) -> ENGC_R {
        ENGC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline(always)]
    pub fn enpec(&self) -> ENPEC_R {
        ENPEC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ARP enable"]
    #[inline(always)]
    pub fn enarp(&self) -> ENARP_R {
        ENARP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SMBus type"]
    #[inline(always)]
    pub fn smbtype(&self) -> SMBTYPE_R {
        SMBTYPE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SMBus mode"]
    #[inline(always)]
    pub fn smbus(&self) -> SMBUS_R {
        SMBUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    pub fn alert(&mut self) -> ALERT_W {
        ALERT_W { w: self }
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline(always)]
    pub fn pec(&mut self) -> PEC_W {
        PEC_W { w: self }
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline(always)]
    pub fn pos(&mut self) -> POS_W {
        POS_W { w: self }
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W { w: self }
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 7 - Clock stretching disable (Slave mode)"]
    #[inline(always)]
    pub fn nostretch(&mut self) -> NOSTRETCH_W {
        NOSTRETCH_W { w: self }
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline(always)]
    pub fn engc(&mut self) -> ENGC_W {
        ENGC_W { w: self }
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline(always)]
    pub fn enpec(&mut self) -> ENPEC_W {
        ENPEC_W { w: self }
    }
    #[doc = "Bit 4 - ARP enable"]
    #[inline(always)]
    pub fn enarp(&mut self) -> ENARP_W {
        ENARP_W { w: self }
    }
    #[doc = "Bit 3 - SMBus type"]
    #[inline(always)]
    pub fn smbtype(&mut self) -> SMBTYPE_W {
        SMBTYPE_W { w: self }
    }
    #[doc = "Bit 1 - SMBus mode"]
    #[inline(always)]
    pub fn smbus(&mut self) -> SMBUS_W {
        SMBUS_W { w: self }
    }
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
}
