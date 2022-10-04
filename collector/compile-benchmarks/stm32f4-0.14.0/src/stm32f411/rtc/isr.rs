#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISR` writer"]
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Alarm A write flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRAWF_A {
    #[doc = "0: Alarm update not allowed"]
    UPDATENOTALLOWED = 0,
    #[doc = "1: Alarm update allowed"]
    UPDATEALLOWED = 1,
}
impl From<ALRAWF_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAWF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAWF` reader - Alarm A write flag"]
pub struct ALRAWF_R(crate::FieldReader<bool, ALRAWF_A>);
impl ALRAWF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRAWF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALRAWF_A {
        match self.bits {
            false => ALRAWF_A::UPDATENOTALLOWED,
            true => ALRAWF_A::UPDATEALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `UPDATENOTALLOWED`"]
    #[inline(always)]
    pub fn is_update_not_allowed(&self) -> bool {
        **self == ALRAWF_A::UPDATENOTALLOWED
    }
    #[doc = "Checks if the value of the field is `UPDATEALLOWED`"]
    #[inline(always)]
    pub fn is_update_allowed(&self) -> bool {
        **self == ALRAWF_A::UPDATEALLOWED
    }
}
impl core::ops::Deref for ALRAWF_R {
    type Target = crate::FieldReader<bool, ALRAWF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Alarm B write flag"]
pub type ALRBWF_A = ALRAWF_A;
#[doc = "Field `ALRBWF` reader - Alarm B write flag"]
pub type ALRBWF_R = ALRAWF_R;
#[doc = "Wakeup timer write flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTWF_A {
    #[doc = "0: Wakeup timer configuration update not allowed"]
    UPDATENOTALLOWED = 0,
    #[doc = "1: Wakeup timer configuration update allowed"]
    UPDATEALLOWED = 1,
}
impl From<WUTWF_A> for bool {
    #[inline(always)]
    fn from(variant: WUTWF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTWF` reader - Wakeup timer write flag"]
pub struct WUTWF_R(crate::FieldReader<bool, WUTWF_A>);
impl WUTWF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUTWF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUTWF_A {
        match self.bits {
            false => WUTWF_A::UPDATENOTALLOWED,
            true => WUTWF_A::UPDATEALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `UPDATENOTALLOWED`"]
    #[inline(always)]
    pub fn is_update_not_allowed(&self) -> bool {
        **self == WUTWF_A::UPDATENOTALLOWED
    }
    #[doc = "Checks if the value of the field is `UPDATEALLOWED`"]
    #[inline(always)]
    pub fn is_update_allowed(&self) -> bool {
        **self == WUTWF_A::UPDATEALLOWED
    }
}
impl core::ops::Deref for WUTWF_R {
    type Target = crate::FieldReader<bool, WUTWF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Shift operation pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHPF_A {
    #[doc = "0: No shift operation is pending"]
    NOSHIFTPENDING = 0,
    #[doc = "1: A shift operation is pending"]
    SHIFTPENDING = 1,
}
impl From<SHPF_A> for bool {
    #[inline(always)]
    fn from(variant: SHPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHPF` reader - Shift operation pending"]
pub struct SHPF_R(crate::FieldReader<bool, SHPF_A>);
impl SHPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHPF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHPF_A {
        match self.bits {
            false => SHPF_A::NOSHIFTPENDING,
            true => SHPF_A::SHIFTPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOSHIFTPENDING`"]
    #[inline(always)]
    pub fn is_no_shift_pending(&self) -> bool {
        **self == SHPF_A::NOSHIFTPENDING
    }
    #[doc = "Checks if the value of the field is `SHIFTPENDING`"]
    #[inline(always)]
    pub fn is_shift_pending(&self) -> bool {
        **self == SHPF_A::SHIFTPENDING
    }
}
impl core::ops::Deref for SHPF_R {
    type Target = crate::FieldReader<bool, SHPF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHPF` writer - Shift operation pending"]
pub struct SHPF_W<'a> {
    w: &'a mut W,
}
impl<'a> SHPF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHPF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No shift operation is pending"]
    #[inline(always)]
    pub fn no_shift_pending(self) -> &'a mut W {
        self.variant(SHPF_A::NOSHIFTPENDING)
    }
    #[doc = "A shift operation is pending"]
    #[inline(always)]
    pub fn shift_pending(self) -> &'a mut W {
        self.variant(SHPF_A::SHIFTPENDING)
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
#[doc = "Initialization status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITS_A {
    #[doc = "0: Calendar has not been initialized"]
    NOTINITALIZED = 0,
    #[doc = "1: Calendar has been initialized"]
    INITALIZED = 1,
}
impl From<INITS_A> for bool {
    #[inline(always)]
    fn from(variant: INITS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITS` reader - Initialization status flag"]
pub struct INITS_R(crate::FieldReader<bool, INITS_A>);
impl INITS_R {
    pub(crate) fn new(bits: bool) -> Self {
        INITS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITS_A {
        match self.bits {
            false => INITS_A::NOTINITALIZED,
            true => INITS_A::INITALIZED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINITALIZED`"]
    #[inline(always)]
    pub fn is_not_initalized(&self) -> bool {
        **self == INITS_A::NOTINITALIZED
    }
    #[doc = "Checks if the value of the field is `INITALIZED`"]
    #[inline(always)]
    pub fn is_initalized(&self) -> bool {
        **self == INITS_A::INITALIZED
    }
}
impl core::ops::Deref for INITS_R {
    type Target = crate::FieldReader<bool, INITS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Registers synchronization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSF_A {
    #[doc = "0: Calendar shadow registers not yet synchronized"]
    NOTSYNCED = 0,
    #[doc = "1: Calendar shadow registers synchronized"]
    SYNCED = 1,
}
impl From<RSF_A> for bool {
    #[inline(always)]
    fn from(variant: RSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` reader - Registers synchronization flag"]
pub struct RSF_R(crate::FieldReader<bool, RSF_A>);
impl RSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSF_A {
        match self.bits {
            false => RSF_A::NOTSYNCED,
            true => RSF_A::SYNCED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSYNCED`"]
    #[inline(always)]
    pub fn is_not_synced(&self) -> bool {
        **self == RSF_A::NOTSYNCED
    }
    #[doc = "Checks if the value of the field is `SYNCED`"]
    #[inline(always)]
    pub fn is_synced(&self) -> bool {
        **self == RSF_A::SYNCED
    }
}
impl core::ops::Deref for RSF_R {
    type Target = crate::FieldReader<bool, RSF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Registers synchronization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSF_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    CLEAR = 0,
}
impl From<RSF_AW> for bool {
    #[inline(always)]
    fn from(variant: RSF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` writer - Registers synchronization flag"]
pub struct RSF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RSF_AW::CLEAR)
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
#[doc = "Initialization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITF_A {
    #[doc = "0: Calendar registers update is not allowed"]
    NOTALLOWED = 0,
    #[doc = "1: Calendar registers update is allowed"]
    ALLOWED = 1,
}
impl From<INITF_A> for bool {
    #[inline(always)]
    fn from(variant: INITF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITF` reader - Initialization flag"]
pub struct INITF_R(crate::FieldReader<bool, INITF_A>);
impl INITF_R {
    pub(crate) fn new(bits: bool) -> Self {
        INITF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITF_A {
        match self.bits {
            false => INITF_A::NOTALLOWED,
            true => INITF_A::ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTALLOWED`"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        **self == INITF_A::NOTALLOWED
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        **self == INITF_A::ALLOWED
    }
}
impl core::ops::Deref for INITF_R {
    type Target = crate::FieldReader<bool, INITF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Initialization mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_A {
    #[doc = "0: Free running mode"]
    FREERUNNINGMODE = 0,
    #[doc = "1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
    INITMODE = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT` reader - Initialization mode"]
pub struct INIT_R(crate::FieldReader<bool, INIT_A>);
impl INIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::FREERUNNINGMODE,
            true => INIT_A::INITMODE,
        }
    }
    #[doc = "Checks if the value of the field is `FREERUNNINGMODE`"]
    #[inline(always)]
    pub fn is_free_running_mode(&self) -> bool {
        **self == INIT_A::FREERUNNINGMODE
    }
    #[doc = "Checks if the value of the field is `INITMODE`"]
    #[inline(always)]
    pub fn is_init_mode(&self) -> bool {
        **self == INIT_A::INITMODE
    }
}
impl core::ops::Deref for INIT_R {
    type Target = crate::FieldReader<bool, INIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIT` writer - Initialization mode"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Free running mode"]
    #[inline(always)]
    pub fn free_running_mode(self) -> &'a mut W {
        self.variant(INIT_A::FREERUNNINGMODE)
    }
    #[doc = "Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
    #[inline(always)]
    pub fn init_mode(self) -> &'a mut W {
        self.variant(INIT_A::INITMODE)
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
#[doc = "Alarm A flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRAF_A {
    #[doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)"]
    MATCH = 1,
}
impl From<ALRAF_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAF` reader - Alarm A flag"]
pub struct ALRAF_R(crate::FieldReader<bool, ALRAF_A>);
impl ALRAF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRAF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALRAF_A> {
        match self.bits {
            true => Some(ALRAF_A::MATCH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        **self == ALRAF_A::MATCH
    }
}
impl core::ops::Deref for ALRAF_R {
    type Target = crate::FieldReader<bool, ALRAF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Alarm A flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRAF_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    CLEAR = 0,
}
impl From<ALRAF_AW> for bool {
    #[inline(always)]
    fn from(variant: ALRAF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAF` writer - Alarm A flag"]
pub struct ALRAF_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRAF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALRAF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ALRAF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Alarm B flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRBF_A {
    #[doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm B register (RTC_ALRMBR)"]
    MATCH = 1,
}
impl From<ALRBF_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRBF` reader - Alarm B flag"]
pub struct ALRBF_R(crate::FieldReader<bool, ALRBF_A>);
impl ALRBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRBF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALRBF_A> {
        match self.bits {
            true => Some(ALRBF_A::MATCH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        **self == ALRBF_A::MATCH
    }
}
impl core::ops::Deref for ALRBF_R {
    type Target = crate::FieldReader<bool, ALRBF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Alarm B flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRBF_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    CLEAR = 0,
}
impl From<ALRBF_AW> for bool {
    #[inline(always)]
    fn from(variant: ALRBF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRBF` writer - Alarm B flag"]
pub struct ALRBF_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRBF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALRBF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ALRBF_AW::CLEAR)
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
#[doc = "Wakeup timer flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTF_A {
    #[doc = "1: This flag is set by hardware when the wakeup auto-reload counter reaches 0"]
    ZERO = 1,
}
impl From<WUTF_A> for bool {
    #[inline(always)]
    fn from(variant: WUTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTF` reader - Wakeup timer flag"]
pub struct WUTF_R(crate::FieldReader<bool, WUTF_A>);
impl WUTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WUTF_A> {
        match self.bits {
            true => Some(WUTF_A::ZERO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == WUTF_A::ZERO
    }
}
impl core::ops::Deref for WUTF_R {
    type Target = crate::FieldReader<bool, WUTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Wakeup timer flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTF_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    CLEAR = 0,
}
impl From<WUTF_AW> for bool {
    #[inline(always)]
    fn from(variant: WUTF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTF` writer - Wakeup timer flag"]
pub struct WUTF_W<'a> {
    w: &'a mut W,
}
impl<'a> WUTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUTF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WUTF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Time-stamp flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSF_A {
    #[doc = "1: This flag is set by hardware when a time-stamp event occurs"]
    TIMESTAMPEVENT = 1,
}
impl From<TSF_A> for bool {
    #[inline(always)]
    fn from(variant: TSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSF` reader - Time-stamp flag"]
pub struct TSF_R(crate::FieldReader<bool, TSF_A>);
impl TSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSF_A> {
        match self.bits {
            true => Some(TSF_A::TIMESTAMPEVENT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIMESTAMPEVENT`"]
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        **self == TSF_A::TIMESTAMPEVENT
    }
}
impl core::ops::Deref for TSF_R {
    type Target = crate::FieldReader<bool, TSF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Time-stamp flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSF_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    CLEAR = 0,
}
impl From<TSF_AW> for bool {
    #[inline(always)]
    fn from(variant: TSF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSF` writer - Time-stamp flag"]
pub struct TSF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TSF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Time-stamp overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSOVF_A {
    #[doc = "1: This flag is set by hardware when a time-stamp event occurs while TSF is already set"]
    OVERFLOW = 1,
}
impl From<TSOVF_A> for bool {
    #[inline(always)]
    fn from(variant: TSOVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSOVF` reader - Time-stamp overflow flag"]
pub struct TSOVF_R(crate::FieldReader<bool, TSOVF_A>);
impl TSOVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSOVF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSOVF_A> {
        match self.bits {
            true => Some(TSOVF_A::OVERFLOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OVERFLOW`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        **self == TSOVF_A::OVERFLOW
    }
}
impl core::ops::Deref for TSOVF_R {
    type Target = crate::FieldReader<bool, TSOVF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Time-stamp overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSOVF_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    CLEAR = 0,
}
impl From<TSOVF_AW> for bool {
    #[inline(always)]
    fn from(variant: TSOVF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSOVF` writer - Time-stamp overflow flag"]
pub struct TSOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSOVF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSOVF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TSOVF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Tamper detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1F_A {
    #[doc = "1: This flag is set by hardware when a tamper detection event is detected on the RTC_TAMPx input"]
    TAMPERED = 1,
}
impl From<TAMP1F_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1F` reader - Tamper detection flag"]
pub struct TAMP1F_R(crate::FieldReader<bool, TAMP1F_A>);
impl TAMP1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TAMP1F_A> {
        match self.bits {
            true => Some(TAMP1F_A::TAMPERED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TAMPERED`"]
    #[inline(always)]
    pub fn is_tampered(&self) -> bool {
        **self == TAMP1F_A::TAMPERED
    }
}
impl core::ops::Deref for TAMP1F_R {
    type Target = crate::FieldReader<bool, TAMP1F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Tamper detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1F_AW {
    #[doc = "0: Flag cleared by software writing 0"]
    CLEAR = 0,
}
impl From<TAMP1F_AW> for bool {
    #[inline(always)]
    fn from(variant: TAMP1F_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1F` writer - Tamper detection flag"]
pub struct TAMP1F_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP1F_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flag cleared by software writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TAMP1F_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "TAMPER2 detection flag"]
pub type TAMP2F_A = TAMP1F_A;
#[doc = "Field `TAMP2F` reader - TAMPER2 detection flag"]
pub type TAMP2F_R = TAMP1F_R;
#[doc = "TAMPER2 detection flag"]
pub type TAMP2F_AW = TAMP1F_AW;
#[doc = "Field `TAMP2F` writer - TAMPER2 detection flag"]
pub struct TAMP2F_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMP2F_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flag cleared by software writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TAMP2F_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Recalibration pending Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECALPF_A {
    #[doc = "1: The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0"]
    PENDING = 1,
}
impl From<RECALPF_A> for bool {
    #[inline(always)]
    fn from(variant: RECALPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECALPF` reader - Recalibration pending Flag"]
pub struct RECALPF_R(crate::FieldReader<bool, RECALPF_A>);
impl RECALPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECALPF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RECALPF_A> {
        match self.bits {
            true => Some(RECALPF_A::PENDING),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RECALPF_A::PENDING
    }
}
impl core::ops::Deref for RECALPF_R {
    type Target = crate::FieldReader<bool, RECALPF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Alarm A write flag"]
    #[inline(always)]
    pub fn alrawf(&self) -> ALRAWF_R {
        ALRAWF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alarm B write flag"]
    #[inline(always)]
    pub fn alrbwf(&self) -> ALRBWF_R {
        ALRBWF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer write flag"]
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Initialization status flag"]
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Initialization flag"]
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    pub fn alrbf(&self) -> ALRBF_R {
        ALRBF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Tamper detection flag"]
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TAMPER2 detection flag"]
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Recalibration pending Flag"]
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    pub fn shpf(&mut self) -> SHPF_W {
        SHPF_W { w: self }
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W {
        RSF_W { w: self }
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    pub fn alraf(&mut self) -> ALRAF_W {
        ALRAF_W { w: self }
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    pub fn alrbf(&mut self) -> ALRBF_W {
        ALRBF_W { w: self }
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    pub fn wutf(&mut self) -> WUTF_W {
        WUTF_W { w: self }
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W {
        TSF_W { w: self }
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    pub fn tsovf(&mut self) -> TSOVF_W {
        TSOVF_W { w: self }
    }
    #[doc = "Bit 13 - Tamper detection flag"]
    #[inline(always)]
    pub fn tamp1f(&mut self) -> TAMP1F_W {
        TAMP1F_W { w: self }
    }
    #[doc = "Bit 14 - TAMPER2 detection flag"]
    #[inline(always)]
    pub fn tamp2f(&mut self) -> TAMP2F_W {
        TAMP2F_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "initialization and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr::W](W) writer structure"]
impl crate::Writable for ISR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISR to value 0x07"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
