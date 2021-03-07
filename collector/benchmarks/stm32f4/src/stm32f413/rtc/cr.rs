#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Calibration output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COE_A {
    #[doc = "0: Calibration output disabled"]
    DISABLED = 0,
    #[doc = "1: Calibration output enabled"]
    ENABLED = 1,
}
impl From<COE_A> for bool {
    #[inline(always)]
    fn from(variant: COE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COE`"]
pub type COE_R = crate::R<bool, COE_A>;
impl COE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COE_A {
        match self.bits {
            false => COE_A::DISABLED,
            true => COE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COE_A::ENABLED
    }
}
#[doc = "Write proxy for field `COE`"]
pub struct COE_W<'a> {
    w: &'a mut W,
}
impl<'a> COE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Calibration output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COE_A::DISABLED)
    }
    #[doc = "Calibration output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSEL_A {
    #[doc = "0: Output disabled"]
    DISABLED = 0,
    #[doc = "1: Alarm A output enabled"]
    ALARMA = 1,
    #[doc = "2: Alarm B output enabled"]
    ALARMB = 2,
    #[doc = "3: Wakeup output enabled"]
    WAKEUP = 3,
}
impl From<OSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OSEL`"]
pub type OSEL_R = crate::R<u8, OSEL_A>;
impl OSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSEL_A {
        match self.bits {
            0 => OSEL_A::DISABLED,
            1 => OSEL_A::ALARMA,
            2 => OSEL_A::ALARMB,
            3 => OSEL_A::WAKEUP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSEL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ALARMA`"]
    #[inline(always)]
    pub fn is_alarm_a(&self) -> bool {
        *self == OSEL_A::ALARMA
    }
    #[doc = "Checks if the value of the field is `ALARMB`"]
    #[inline(always)]
    pub fn is_alarm_b(&self) -> bool {
        *self == OSEL_A::ALARMB
    }
    #[doc = "Checks if the value of the field is `WAKEUP`"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == OSEL_A::WAKEUP
    }
}
#[doc = "Write proxy for field `OSEL`"]
pub struct OSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSEL_A::DISABLED)
    }
    #[doc = "Alarm A output enabled"]
    #[inline(always)]
    pub fn alarm_a(self) -> &'a mut W {
        self.variant(OSEL_A::ALARMA)
    }
    #[doc = "Alarm B output enabled"]
    #[inline(always)]
    pub fn alarm_b(self) -> &'a mut W {
        self.variant(OSEL_A::ALARMB)
    }
    #[doc = "Wakeup output enabled"]
    #[inline(always)]
    pub fn wakeup(self) -> &'a mut W {
        self.variant(OSEL_A::WAKEUP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_A {
    #[doc = "0: The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
    HIGH = 0,
    #[doc = "1: The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
    LOW = 1,
}
impl From<POL_A> for bool {
    #[inline(always)]
    fn from(variant: POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POL`"]
pub type POL_R = crate::R<bool, POL_A>;
impl POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_A {
        match self.bits {
            false => POL_A::HIGH,
            true => POL_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_A::LOW
    }
}
#[doc = "Write proxy for field `POL`"]
pub struct POL_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_A::HIGH)
    }
    #[doc = "The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_A::LOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Calibration Output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COSEL_A {
    #[doc = "0: Calibration output is 512 Hz (with default prescaler setting)"]
    CALFREQ_512HZ = 0,
    #[doc = "1: Calibration output is 1 Hz (with default prescaler setting)"]
    CALFREQ_1HZ = 1,
}
impl From<COSEL_A> for bool {
    #[inline(always)]
    fn from(variant: COSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COSEL`"]
pub type COSEL_R = crate::R<bool, COSEL_A>;
impl COSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COSEL_A {
        match self.bits {
            false => COSEL_A::CALFREQ_512HZ,
            true => COSEL_A::CALFREQ_1HZ,
        }
    }
    #[doc = "Checks if the value of the field is `CALFREQ_512HZ`"]
    #[inline(always)]
    pub fn is_cal_freq_512hz(&self) -> bool {
        *self == COSEL_A::CALFREQ_512HZ
    }
    #[doc = "Checks if the value of the field is `CALFREQ_1HZ`"]
    #[inline(always)]
    pub fn is_cal_freq_1hz(&self) -> bool {
        *self == COSEL_A::CALFREQ_1HZ
    }
}
#[doc = "Write proxy for field `COSEL`"]
pub struct COSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Calibration output is 512 Hz (with default prescaler setting)"]
    #[inline(always)]
    pub fn cal_freq_512hz(self) -> &'a mut W {
        self.variant(COSEL_A::CALFREQ_512HZ)
    }
    #[doc = "Calibration output is 1 Hz (with default prescaler setting)"]
    #[inline(always)]
    pub fn cal_freq_1hz(self) -> &'a mut W {
        self.variant(COSEL_A::CALFREQ_1HZ)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Backup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKP_A {
    #[doc = "0: Daylight Saving Time change has not been performed"]
    DST_NOT_CHANGED = 0,
    #[doc = "1: Daylight Saving Time change has been performed"]
    DST_CHANGED = 1,
}
impl From<BKP_A> for bool {
    #[inline(always)]
    fn from(variant: BKP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BKP`"]
pub type BKP_R = crate::R<bool, BKP_A>;
impl BKP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKP_A {
        match self.bits {
            false => BKP_A::DST_NOT_CHANGED,
            true => BKP_A::DST_CHANGED,
        }
    }
    #[doc = "Checks if the value of the field is `DST_NOT_CHANGED`"]
    #[inline(always)]
    pub fn is_dst_not_changed(&self) -> bool {
        *self == BKP_A::DST_NOT_CHANGED
    }
    #[doc = "Checks if the value of the field is `DST_CHANGED`"]
    #[inline(always)]
    pub fn is_dst_changed(&self) -> bool {
        *self == BKP_A::DST_CHANGED
    }
}
#[doc = "Write proxy for field `BKP`"]
pub struct BKP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Daylight Saving Time change has not been performed"]
    #[inline(always)]
    pub fn dst_not_changed(self) -> &'a mut W {
        self.variant(BKP_A::DST_NOT_CHANGED)
    }
    #[doc = "Daylight Saving Time change has been performed"]
    #[inline(always)]
    pub fn dst_changed(self) -> &'a mut W {
        self.variant(BKP_A::DST_CHANGED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Subtract 1 hour (winter time change)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUB1H_A {
    #[doc = "1: Subtracts 1 hour to the current time. This can be used for winter time change outside initialization mode"]
    SUB1 = 1,
}
impl From<SUB1H_A> for bool {
    #[inline(always)]
    fn from(variant: SUB1H_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SUB1H`"]
pub type SUB1H_R = crate::R<bool, SUB1H_A>;
impl SUB1H_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SUB1H_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SUB1H_A::SUB1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUB1`"]
    #[inline(always)]
    pub fn is_sub1(&self) -> bool {
        *self == SUB1H_A::SUB1
    }
}
#[doc = "Write proxy for field `SUB1H`"]
pub struct SUB1H_W<'a> {
    w: &'a mut W,
}
impl<'a> SUB1H_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUB1H_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Subtracts 1 hour to the current time. This can be used for winter time change outside initialization mode"]
    #[inline(always)]
    pub fn sub1(self) -> &'a mut W {
        self.variant(SUB1H_A::SUB1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Add 1 hour (summer time change)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADD1H_A {
    #[doc = "1: Adds 1 hour to the current time. This can be used for summer time change outside initialization mode"]
    ADD1 = 1,
}
impl From<ADD1H_A> for bool {
    #[inline(always)]
    fn from(variant: ADD1H_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADD1H`"]
pub type ADD1H_R = crate::R<bool, ADD1H_A>;
impl ADD1H_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ADD1H_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(ADD1H_A::ADD1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADD1`"]
    #[inline(always)]
    pub fn is_add1(&self) -> bool {
        *self == ADD1H_A::ADD1
    }
}
#[doc = "Write proxy for field `ADD1H`"]
pub struct ADD1H_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD1H_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADD1H_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Adds 1 hour to the current time. This can be used for summer time change outside initialization mode"]
    #[inline(always)]
    pub fn add1(self) -> &'a mut W {
        self.variant(ADD1H_A::ADD1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Time-stamp interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIE_A {
    #[doc = "0: Time-stamp Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Time-stamp Interrupt enabled"]
    ENABLED = 1,
}
impl From<TSIE_A> for bool {
    #[inline(always)]
    fn from(variant: TSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSIE`"]
pub type TSIE_R = crate::R<bool, TSIE_A>;
impl TSIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSIE_A {
        match self.bits {
            false => TSIE_A::DISABLED,
            true => TSIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `TSIE`"]
pub struct TSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Time-stamp Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSIE_A::DISABLED)
    }
    #[doc = "Time-stamp Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSIE_A::ENABLED)
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
#[doc = "Wakeup timer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTIE_A {
    #[doc = "0: Wakeup timer interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Wakeup timer interrupt enabled"]
    ENABLED = 1,
}
impl From<WUTIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WUTIE`"]
pub type WUTIE_R = crate::R<bool, WUTIE_A>;
impl WUTIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUTIE_A {
        match self.bits {
            false => WUTIE_A::DISABLED,
            true => WUTIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUTIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUTIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `WUTIE`"]
pub struct WUTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WUTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wakeup timer interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUTIE_A::DISABLED)
    }
    #[doc = "Wakeup timer interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUTIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Alarm B interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRBIE_A {
    #[doc = "0: Alarm B Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Alarm B Interrupt enabled"]
    ENABLED = 1,
}
impl From<ALRBIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALRBIE`"]
pub type ALRBIE_R = crate::R<bool, ALRBIE_A>;
impl ALRBIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALRBIE_A {
        match self.bits {
            false => ALRBIE_A::DISABLED,
            true => ALRBIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRBIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRBIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `ALRBIE`"]
pub struct ALRBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRBIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALRBIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Alarm B Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRBIE_A::DISABLED)
    }
    #[doc = "Alarm B Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRBIE_A::ENABLED)
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
#[doc = "Alarm A interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRAIE_A {
    #[doc = "0: Alarm A interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Alarm A interrupt enabled"]
    ENABLED = 1,
}
impl From<ALRAIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALRAIE`"]
pub type ALRAIE_R = crate::R<bool, ALRAIE_A>;
impl ALRAIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALRAIE_A {
        match self.bits {
            false => ALRAIE_A::DISABLED,
            true => ALRAIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRAIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRAIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `ALRAIE`"]
pub struct ALRAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRAIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALRAIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Alarm A interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRAIE_A::DISABLED)
    }
    #[doc = "Alarm A interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRAIE_A::ENABLED)
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
#[doc = "Time stamp enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSE_A {
    #[doc = "0: Timestamp disabled"]
    DISABLED = 0,
    #[doc = "1: Timestamp enabled"]
    ENABLED = 1,
}
impl From<TSE_A> for bool {
    #[inline(always)]
    fn from(variant: TSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSE`"]
pub type TSE_R = crate::R<bool, TSE_A>;
impl TSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSE_A {
        match self.bits {
            false => TSE_A::DISABLED,
            true => TSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSE_A::ENABLED
    }
}
#[doc = "Write proxy for field `TSE`"]
pub struct TSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timestamp disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSE_A::DISABLED)
    }
    #[doc = "Timestamp enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSE_A::ENABLED)
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
#[doc = "Wakeup timer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTE_A {
    #[doc = "0: Wakeup timer disabled"]
    DISABLED = 0,
    #[doc = "1: Wakeup timer enabled"]
    ENABLED = 1,
}
impl From<WUTE_A> for bool {
    #[inline(always)]
    fn from(variant: WUTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WUTE`"]
pub type WUTE_R = crate::R<bool, WUTE_A>;
impl WUTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUTE_A {
        match self.bits {
            false => WUTE_A::DISABLED,
            true => WUTE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUTE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUTE_A::ENABLED
    }
}
#[doc = "Write proxy for field `WUTE`"]
pub struct WUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> WUTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wakeup timer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUTE_A::DISABLED)
    }
    #[doc = "Wakeup timer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUTE_A::ENABLED)
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
#[doc = "Alarm B enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRBE_A {
    #[doc = "0: Alarm B disabled"]
    DISABLED = 0,
    #[doc = "1: Alarm B enabled"]
    ENABLED = 1,
}
impl From<ALRBE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALRBE`"]
pub type ALRBE_R = crate::R<bool, ALRBE_A>;
impl ALRBE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALRBE_A {
        match self.bits {
            false => ALRBE_A::DISABLED,
            true => ALRBE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRBE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRBE_A::ENABLED
    }
}
#[doc = "Write proxy for field `ALRBE`"]
pub struct ALRBE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRBE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALRBE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Alarm B disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRBE_A::DISABLED)
    }
    #[doc = "Alarm B enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRBE_A::ENABLED)
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
#[doc = "Alarm A enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRAE_A {
    #[doc = "0: Alarm A disabled"]
    DISABLED = 0,
    #[doc = "1: Alarm A enabled"]
    ENABLED = 1,
}
impl From<ALRAE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALRAE`"]
pub type ALRAE_R = crate::R<bool, ALRAE_A>;
impl ALRAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALRAE_A {
        match self.bits {
            false => ALRAE_A::DISABLED,
            true => ALRAE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRAE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRAE_A::ENABLED
    }
}
#[doc = "Write proxy for field `ALRAE`"]
pub struct ALRAE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALRAE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Alarm A disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRAE_A::DISABLED)
    }
    #[doc = "Alarm A enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRAE_A::ENABLED)
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
#[doc = "Reader of field `DCE`"]
pub type DCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCE`"]
pub struct DCE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCE_W<'a> {
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
#[doc = "Hour format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMT_A {
    #[doc = "0: 24 hour/day format"]
    TWENTY_FOUR_HOUR = 0,
    #[doc = "1: AM/PM hour format"]
    AM_PM = 1,
}
impl From<FMT_A> for bool {
    #[inline(always)]
    fn from(variant: FMT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FMT`"]
pub type FMT_R = crate::R<bool, FMT_A>;
impl FMT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMT_A {
        match self.bits {
            false => FMT_A::TWENTY_FOUR_HOUR,
            true => FMT_A::AM_PM,
        }
    }
    #[doc = "Checks if the value of the field is `TWENTY_FOUR_HOUR`"]
    #[inline(always)]
    pub fn is_twenty_four_hour(&self) -> bool {
        *self == FMT_A::TWENTY_FOUR_HOUR
    }
    #[doc = "Checks if the value of the field is `AM_PM`"]
    #[inline(always)]
    pub fn is_am_pm(&self) -> bool {
        *self == FMT_A::AM_PM
    }
}
#[doc = "Write proxy for field `FMT`"]
pub struct FMT_W<'a> {
    w: &'a mut W,
}
impl<'a> FMT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "24 hour/day format"]
    #[inline(always)]
    pub fn twenty_four_hour(self) -> &'a mut W {
        self.variant(FMT_A::TWENTY_FOUR_HOUR)
    }
    #[doc = "AM/PM hour format"]
    #[inline(always)]
    pub fn am_pm(self) -> &'a mut W {
        self.variant(FMT_A::AM_PM)
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
#[doc = "Bypass the shadow registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPSHAD_A {
    #[doc = "0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles"]
    SHADOWREG = 0,
    #[doc = "1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters"]
    BYPASSSHADOWREG = 1,
}
impl From<BYPSHAD_A> for bool {
    #[inline(always)]
    fn from(variant: BYPSHAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BYPSHAD`"]
pub type BYPSHAD_R = crate::R<bool, BYPSHAD_A>;
impl BYPSHAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPSHAD_A {
        match self.bits {
            false => BYPSHAD_A::SHADOWREG,
            true => BYPSHAD_A::BYPASSSHADOWREG,
        }
    }
    #[doc = "Checks if the value of the field is `SHADOWREG`"]
    #[inline(always)]
    pub fn is_shadow_reg(&self) -> bool {
        *self == BYPSHAD_A::SHADOWREG
    }
    #[doc = "Checks if the value of the field is `BYPASSSHADOWREG`"]
    #[inline(always)]
    pub fn is_bypass_shadow_reg(&self) -> bool {
        *self == BYPSHAD_A::BYPASSSHADOWREG
    }
}
#[doc = "Write proxy for field `BYPSHAD`"]
pub struct BYPSHAD_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPSHAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPSHAD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles"]
    #[inline(always)]
    pub fn shadow_reg(self) -> &'a mut W {
        self.variant(BYPSHAD_A::SHADOWREG)
    }
    #[doc = "Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters"]
    #[inline(always)]
    pub fn bypass_shadow_reg(self) -> &'a mut W {
        self.variant(BYPSHAD_A::BYPASSSHADOWREG)
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
#[doc = "Reference clock detection enable (50 or 60 Hz)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFCKON_A {
    #[doc = "0: RTC_REFIN detection disabled"]
    DISABLED = 0,
    #[doc = "1: RTC_REFIN detection enabled"]
    ENABLED = 1,
}
impl From<REFCKON_A> for bool {
    #[inline(always)]
    fn from(variant: REFCKON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFCKON`"]
pub type REFCKON_R = crate::R<bool, REFCKON_A>;
impl REFCKON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFCKON_A {
        match self.bits {
            false => REFCKON_A::DISABLED,
            true => REFCKON_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REFCKON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REFCKON_A::ENABLED
    }
}
#[doc = "Write proxy for field `REFCKON`"]
pub struct REFCKON_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCKON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFCKON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RTC_REFIN detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REFCKON_A::DISABLED)
    }
    #[doc = "RTC_REFIN detection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REFCKON_A::ENABLED)
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
#[doc = "Time-stamp event active edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEDGE_A {
    #[doc = "0: RTC_TS input rising edge generates a time-stamp event"]
    RISINGEDGE = 0,
    #[doc = "1: RTC_TS input falling edge generates a time-stamp event"]
    FALLINGEDGE = 1,
}
impl From<TSEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: TSEDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSEDGE`"]
pub type TSEDGE_R = crate::R<bool, TSEDGE_A>;
impl TSEDGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSEDGE_A {
        match self.bits {
            false => TSEDGE_A::RISINGEDGE,
            true => TSEDGE_A::FALLINGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TSEDGE_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TSEDGE_A::FALLINGEDGE
    }
}
#[doc = "Write proxy for field `TSEDGE`"]
pub struct TSEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEDGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RTC_TS input rising edge generates a time-stamp event"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(TSEDGE_A::RISINGEDGE)
    }
    #[doc = "RTC_TS input falling edge generates a time-stamp event"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(TSEDGE_A::FALLINGEDGE)
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
#[doc = "Wakeup clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUCKSEL_A {
    #[doc = "0: RTC/16 clock is selected"]
    DIV16 = 0,
    #[doc = "1: RTC/8 clock is selected"]
    DIV8 = 1,
    #[doc = "2: RTC/4 clock is selected"]
    DIV4 = 2,
    #[doc = "3: RTC/2 clock is selected"]
    DIV2 = 3,
    #[doc = "4: ck_spre (usually 1 Hz) clock is selected"]
    CLOCKSPARE = 4,
    #[doc = "6: ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value"]
    CLOCKSPAREWITHOFFSET = 6,
}
impl From<WUCKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WUCKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WUCKSEL`"]
pub type WUCKSEL_R = crate::R<u8, WUCKSEL_A>;
impl WUCKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WUCKSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WUCKSEL_A::DIV16),
            1 => Val(WUCKSEL_A::DIV8),
            2 => Val(WUCKSEL_A::DIV4),
            3 => Val(WUCKSEL_A::DIV2),
            4 => Val(WUCKSEL_A::CLOCKSPARE),
            6 => Val(WUCKSEL_A::CLOCKSPAREWITHOFFSET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == WUCKSEL_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == WUCKSEL_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == WUCKSEL_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == WUCKSEL_A::DIV2
    }
    #[doc = "Checks if the value of the field is `CLOCKSPARE`"]
    #[inline(always)]
    pub fn is_clock_spare(&self) -> bool {
        *self == WUCKSEL_A::CLOCKSPARE
    }
    #[doc = "Checks if the value of the field is `CLOCKSPAREWITHOFFSET`"]
    #[inline(always)]
    pub fn is_clock_spare_with_offset(&self) -> bool {
        *self == WUCKSEL_A::CLOCKSPAREWITHOFFSET
    }
}
#[doc = "Write proxy for field `WUCKSEL`"]
pub struct WUCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WUCKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUCKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "RTC/16 clock is selected"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(WUCKSEL_A::DIV16)
    }
    #[doc = "RTC/8 clock is selected"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(WUCKSEL_A::DIV8)
    }
    #[doc = "RTC/4 clock is selected"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(WUCKSEL_A::DIV4)
    }
    #[doc = "RTC/2 clock is selected"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(WUCKSEL_A::DIV2)
    }
    #[doc = "ck_spre (usually 1 Hz) clock is selected"]
    #[inline(always)]
    pub fn clock_spare(self) -> &'a mut W {
        self.variant(WUCKSEL_A::CLOCKSPARE)
    }
    #[doc = "ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value"]
    #[inline(always)]
    pub fn clock_spare_with_offset(self) -> &'a mut W {
        self.variant(WUCKSEL_A::CLOCKSPAREWITHOFFSET)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - Output selection"]
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Calibration Output selection"]
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Backup"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change)"]
    #[inline(always)]
    pub fn sub1h(&self) -> SUB1H_R {
        SUB1H_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change)"]
    #[inline(always)]
    pub fn add1h(&self) -> ADD1H_R {
        ADD1H_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Time-stamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    pub fn alrbie(&self) -> ALRBIE_R {
        ALRBIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Time stamp enable"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    pub fn alrbe(&self) -> ALRBE_R {
        ALRBE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Coarse digital calibration enable"]
    #[inline(always)]
    pub fn dce(&self) -> DCE_R {
        DCE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Time-stamp event active edge"]
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Wakeup clock selection"]
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    pub fn coe(&mut self) -> COE_W {
        COE_W { w: self }
    }
    #[doc = "Bits 21:22 - Output selection"]
    #[inline(always)]
    pub fn osel(&mut self) -> OSEL_W {
        OSEL_W { w: self }
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W {
        POL_W { w: self }
    }
    #[doc = "Bit 19 - Calibration Output selection"]
    #[inline(always)]
    pub fn cosel(&mut self) -> COSEL_W {
        COSEL_W { w: self }
    }
    #[doc = "Bit 18 - Backup"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W {
        BKP_W { w: self }
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change)"]
    #[inline(always)]
    pub fn sub1h(&mut self) -> SUB1H_W {
        SUB1H_W { w: self }
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change)"]
    #[inline(always)]
    pub fn add1h(&mut self) -> ADD1H_W {
        ADD1H_W { w: self }
    }
    #[doc = "Bit 15 - Time-stamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W {
        TSIE_W { w: self }
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn wutie(&mut self) -> WUTIE_W {
        WUTIE_W { w: self }
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    pub fn alrbie(&mut self) -> ALRBIE_W {
        ALRBIE_W { w: self }
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alraie(&mut self) -> ALRAIE_W {
        ALRAIE_W { w: self }
    }
    #[doc = "Bit 11 - Time stamp enable"]
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W {
        TSE_W { w: self }
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    pub fn wute(&mut self) -> WUTE_W {
        WUTE_W { w: self }
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    pub fn alrbe(&mut self) -> ALRBE_W {
        ALRBE_W { w: self }
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrae(&mut self) -> ALRAE_W {
        ALRAE_W { w: self }
    }
    #[doc = "Bit 7 - Coarse digital calibration enable"]
    #[inline(always)]
    pub fn dce(&mut self) -> DCE_W {
        DCE_W { w: self }
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn fmt(&mut self) -> FMT_W {
        FMT_W { w: self }
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    pub fn bypshad(&mut self) -> BYPSHAD_W {
        BYPSHAD_W { w: self }
    }
    #[doc = "Bit 4 - Reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    pub fn refckon(&mut self) -> REFCKON_W {
        REFCKON_W { w: self }
    }
    #[doc = "Bit 3 - Time-stamp event active edge"]
    #[inline(always)]
    pub fn tsedge(&mut self) -> TSEDGE_W {
        TSEDGE_W { w: self }
    }
    #[doc = "Bits 0:2 - Wakeup clock selection"]
    #[inline(always)]
    pub fn wucksel(&mut self) -> WUCKSEL_W {
        WUCKSEL_W { w: self }
    }
}
