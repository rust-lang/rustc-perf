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
    UpdateNotAllowed = 0,
    #[doc = "1: Alarm update allowed"]
    UpdateAllowed = 1,
}
impl From<ALRAWF_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAWF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAWF` reader - Alarm A write flag"]
pub type ALRAWF_R = crate::BitReader<ALRAWF_A>;
impl ALRAWF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALRAWF_A {
        match self.bits {
            false => ALRAWF_A::UpdateNotAllowed,
            true => ALRAWF_A::UpdateAllowed,
        }
    }
    #[doc = "Checks if the value of the field is `UpdateNotAllowed`"]
    #[inline(always)]
    pub fn is_update_not_allowed(&self) -> bool {
        *self == ALRAWF_A::UpdateNotAllowed
    }
    #[doc = "Checks if the value of the field is `UpdateAllowed`"]
    #[inline(always)]
    pub fn is_update_allowed(&self) -> bool {
        *self == ALRAWF_A::UpdateAllowed
    }
}
#[doc = "Alarm B write flag"]
pub use ALRAWF_A as ALRBWF_A;
#[doc = "Field `ALRBWF` reader - Alarm B write flag"]
pub use ALRAWF_R as ALRBWF_R;
#[doc = "Wakeup timer write flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTWF_A {
    #[doc = "0: Wakeup timer configuration update not allowed"]
    UpdateNotAllowed = 0,
    #[doc = "1: Wakeup timer configuration update allowed"]
    UpdateAllowed = 1,
}
impl From<WUTWF_A> for bool {
    #[inline(always)]
    fn from(variant: WUTWF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTWF` reader - Wakeup timer write flag"]
pub type WUTWF_R = crate::BitReader<WUTWF_A>;
impl WUTWF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUTWF_A {
        match self.bits {
            false => WUTWF_A::UpdateNotAllowed,
            true => WUTWF_A::UpdateAllowed,
        }
    }
    #[doc = "Checks if the value of the field is `UpdateNotAllowed`"]
    #[inline(always)]
    pub fn is_update_not_allowed(&self) -> bool {
        *self == WUTWF_A::UpdateNotAllowed
    }
    #[doc = "Checks if the value of the field is `UpdateAllowed`"]
    #[inline(always)]
    pub fn is_update_allowed(&self) -> bool {
        *self == WUTWF_A::UpdateAllowed
    }
}
#[doc = "Shift operation pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHPF_A {
    #[doc = "0: No shift operation is pending"]
    NoShiftPending = 0,
    #[doc = "1: A shift operation is pending"]
    ShiftPending = 1,
}
impl From<SHPF_A> for bool {
    #[inline(always)]
    fn from(variant: SHPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHPF` reader - Shift operation pending"]
pub type SHPF_R = crate::BitReader<SHPF_A>;
impl SHPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHPF_A {
        match self.bits {
            false => SHPF_A::NoShiftPending,
            true => SHPF_A::ShiftPending,
        }
    }
    #[doc = "Checks if the value of the field is `NoShiftPending`"]
    #[inline(always)]
    pub fn is_no_shift_pending(&self) -> bool {
        *self == SHPF_A::NoShiftPending
    }
    #[doc = "Checks if the value of the field is `ShiftPending`"]
    #[inline(always)]
    pub fn is_shift_pending(&self) -> bool {
        *self == SHPF_A::ShiftPending
    }
}
#[doc = "Field `SHPF` writer - Shift operation pending"]
pub type SHPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, SHPF_A, O>;
impl<'a, const O: u8> SHPF_W<'a, O> {
    #[doc = "No shift operation is pending"]
    #[inline(always)]
    pub fn no_shift_pending(self) -> &'a mut W {
        self.variant(SHPF_A::NoShiftPending)
    }
    #[doc = "A shift operation is pending"]
    #[inline(always)]
    pub fn shift_pending(self) -> &'a mut W {
        self.variant(SHPF_A::ShiftPending)
    }
}
#[doc = "Initialization status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITS_A {
    #[doc = "0: Calendar has not been initialized"]
    NotInitalized = 0,
    #[doc = "1: Calendar has been initialized"]
    Initalized = 1,
}
impl From<INITS_A> for bool {
    #[inline(always)]
    fn from(variant: INITS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITS` reader - Initialization status flag"]
pub type INITS_R = crate::BitReader<INITS_A>;
impl INITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITS_A {
        match self.bits {
            false => INITS_A::NotInitalized,
            true => INITS_A::Initalized,
        }
    }
    #[doc = "Checks if the value of the field is `NotInitalized`"]
    #[inline(always)]
    pub fn is_not_initalized(&self) -> bool {
        *self == INITS_A::NotInitalized
    }
    #[doc = "Checks if the value of the field is `Initalized`"]
    #[inline(always)]
    pub fn is_initalized(&self) -> bool {
        *self == INITS_A::Initalized
    }
}
#[doc = "Registers synchronization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSF_A {
    #[doc = "0: Calendar shadow registers not yet synchronized"]
    NotSynced = 0,
    #[doc = "1: Calendar shadow registers synchronized"]
    Synced = 1,
}
impl From<RSF_A> for bool {
    #[inline(always)]
    fn from(variant: RSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` reader - Registers synchronization flag"]
pub type RSF_R = crate::BitReader<RSF_A>;
impl RSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSF_A {
        match self.bits {
            false => RSF_A::NotSynced,
            true => RSF_A::Synced,
        }
    }
    #[doc = "Checks if the value of the field is `NotSynced`"]
    #[inline(always)]
    pub fn is_not_synced(&self) -> bool {
        *self == RSF_A::NotSynced
    }
    #[doc = "Checks if the value of the field is `Synced`"]
    #[inline(always)]
    pub fn is_synced(&self) -> bool {
        *self == RSF_A::Synced
    }
}
#[doc = "Registers synchronization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSF_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<RSF_AW> for bool {
    #[inline(always)]
    fn from(variant: RSF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` writer - Registers synchronization flag"]
pub type RSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, RSF_AW, O>;
impl<'a, const O: u8> RSF_W<'a, O> {
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RSF_AW::Clear)
    }
}
#[doc = "Initialization flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITF_A {
    #[doc = "0: Calendar registers update is not allowed"]
    NotAllowed = 0,
    #[doc = "1: Calendar registers update is allowed"]
    Allowed = 1,
}
impl From<INITF_A> for bool {
    #[inline(always)]
    fn from(variant: INITF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITF` reader - Initialization flag"]
pub type INITF_R = crate::BitReader<INITF_A>;
impl INITF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITF_A {
        match self.bits {
            false => INITF_A::NotAllowed,
            true => INITF_A::Allowed,
        }
    }
    #[doc = "Checks if the value of the field is `NotAllowed`"]
    #[inline(always)]
    pub fn is_not_allowed(&self) -> bool {
        *self == INITF_A::NotAllowed
    }
    #[doc = "Checks if the value of the field is `Allowed`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == INITF_A::Allowed
    }
}
#[doc = "Initialization mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_A {
    #[doc = "0: Free running mode"]
    FreeRunningMode = 0,
    #[doc = "1: Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
    InitMode = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT` reader - Initialization mode"]
pub type INIT_R = crate::BitReader<INIT_A>;
impl INIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::FreeRunningMode,
            true => INIT_A::InitMode,
        }
    }
    #[doc = "Checks if the value of the field is `FreeRunningMode`"]
    #[inline(always)]
    pub fn is_free_running_mode(&self) -> bool {
        *self == INIT_A::FreeRunningMode
    }
    #[doc = "Checks if the value of the field is `InitMode`"]
    #[inline(always)]
    pub fn is_init_mode(&self) -> bool {
        *self == INIT_A::InitMode
    }
}
#[doc = "Field `INIT` writer - Initialization mode"]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, INIT_A, O>;
impl<'a, const O: u8> INIT_W<'a, O> {
    #[doc = "Free running mode"]
    #[inline(always)]
    pub fn free_running_mode(self) -> &'a mut W {
        self.variant(INIT_A::FreeRunningMode)
    }
    #[doc = "Initialization mode used to program time and date register (RTC_TR and RTC_DR), and prescaler register (RTC_PRER). Counters are stopped and start counting from the new value when INIT is reset."]
    #[inline(always)]
    pub fn init_mode(self) -> &'a mut W {
        self.variant(INIT_A::InitMode)
    }
}
#[doc = "Alarm A flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRAF_A {
    #[doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)"]
    Match = 1,
}
impl From<ALRAF_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAF` reader - Alarm A flag"]
pub type ALRAF_R = crate::BitReader<ALRAF_A>;
impl ALRAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALRAF_A> {
        match self.bits {
            true => Some(ALRAF_A::Match),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Match`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ALRAF_A::Match
    }
}
#[doc = "Alarm A flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRAF_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<ALRAF_AW> for bool {
    #[inline(always)]
    fn from(variant: ALRAF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAF` writer - Alarm A flag"]
pub type ALRAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, ALRAF_AW, O>;
impl<'a, const O: u8> ALRAF_W<'a, O> {
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ALRAF_AW::Clear)
    }
}
#[doc = "Alarm B flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRBF_A {
    #[doc = "1: This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm B register (RTC_ALRMBR)"]
    Match = 1,
}
impl From<ALRBF_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRBF` reader - Alarm B flag"]
pub type ALRBF_R = crate::BitReader<ALRBF_A>;
impl ALRBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALRBF_A> {
        match self.bits {
            true => Some(ALRBF_A::Match),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Match`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ALRBF_A::Match
    }
}
#[doc = "Alarm B flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRBF_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<ALRBF_AW> for bool {
    #[inline(always)]
    fn from(variant: ALRBF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRBF` writer - Alarm B flag"]
pub type ALRBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, ALRBF_AW, O>;
impl<'a, const O: u8> ALRBF_W<'a, O> {
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ALRBF_AW::Clear)
    }
}
#[doc = "Wakeup timer flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTF_A {
    #[doc = "1: This flag is set by hardware when the wakeup auto-reload counter reaches 0"]
    Zero = 1,
}
impl From<WUTF_A> for bool {
    #[inline(always)]
    fn from(variant: WUTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTF` reader - Wakeup timer flag"]
pub type WUTF_R = crate::BitReader<WUTF_A>;
impl WUTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WUTF_A> {
        match self.bits {
            true => Some(WUTF_A::Zero),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Zero`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == WUTF_A::Zero
    }
}
#[doc = "Wakeup timer flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTF_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<WUTF_AW> for bool {
    #[inline(always)]
    fn from(variant: WUTF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTF` writer - Wakeup timer flag"]
pub type WUTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, WUTF_AW, O>;
impl<'a, const O: u8> WUTF_W<'a, O> {
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WUTF_AW::Clear)
    }
}
#[doc = "Time-stamp flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSF_A {
    #[doc = "1: This flag is set by hardware when a time-stamp event occurs"]
    TimestampEvent = 1,
}
impl From<TSF_A> for bool {
    #[inline(always)]
    fn from(variant: TSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSF` reader - Time-stamp flag"]
pub type TSF_R = crate::BitReader<TSF_A>;
impl TSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSF_A> {
        match self.bits {
            true => Some(TSF_A::TimestampEvent),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TimestampEvent`"]
    #[inline(always)]
    pub fn is_timestamp_event(&self) -> bool {
        *self == TSF_A::TimestampEvent
    }
}
#[doc = "Time-stamp flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSF_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<TSF_AW> for bool {
    #[inline(always)]
    fn from(variant: TSF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSF` writer - Time-stamp flag"]
pub type TSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, TSF_AW, O>;
impl<'a, const O: u8> TSF_W<'a, O> {
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TSF_AW::Clear)
    }
}
#[doc = "Time-stamp overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSOVF_A {
    #[doc = "1: This flag is set by hardware when a time-stamp event occurs while TSF is already set"]
    Overflow = 1,
}
impl From<TSOVF_A> for bool {
    #[inline(always)]
    fn from(variant: TSOVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSOVF` reader - Time-stamp overflow flag"]
pub type TSOVF_R = crate::BitReader<TSOVF_A>;
impl TSOVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSOVF_A> {
        match self.bits {
            true => Some(TSOVF_A::Overflow),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Overflow`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == TSOVF_A::Overflow
    }
}
#[doc = "Time-stamp overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSOVF_AW {
    #[doc = "0: This flag is cleared by software by writing 0"]
    Clear = 0,
}
impl From<TSOVF_AW> for bool {
    #[inline(always)]
    fn from(variant: TSOVF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSOVF` writer - Time-stamp overflow flag"]
pub type TSOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, TSOVF_AW, O>;
impl<'a, const O: u8> TSOVF_W<'a, O> {
    #[doc = "This flag is cleared by software by writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TSOVF_AW::Clear)
    }
}
#[doc = "Tamper detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1F_A {
    #[doc = "1: This flag is set by hardware when a tamper detection event is detected on the RTC_TAMPx input"]
    Tampered = 1,
}
impl From<TAMP1F_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1F` reader - Tamper detection flag"]
pub type TAMP1F_R = crate::BitReader<TAMP1F_A>;
impl TAMP1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TAMP1F_A> {
        match self.bits {
            true => Some(TAMP1F_A::Tampered),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Tampered`"]
    #[inline(always)]
    pub fn is_tampered(&self) -> bool {
        *self == TAMP1F_A::Tampered
    }
}
#[doc = "Tamper detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1F_AW {
    #[doc = "0: Flag cleared by software writing 0"]
    Clear = 0,
}
impl From<TAMP1F_AW> for bool {
    #[inline(always)]
    fn from(variant: TAMP1F_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1F` writer - Tamper detection flag"]
pub type TAMP1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, TAMP1F_AW, O>;
impl<'a, const O: u8> TAMP1F_W<'a, O> {
    #[doc = "Flag cleared by software writing 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TAMP1F_AW::Clear)
    }
}
#[doc = "TAMPER2 detection flag"]
pub use TAMP1F_A as TAMP2F_A;
#[doc = "TAMPER2 detection flag"]
pub use TAMP1F_AW as TAMP2F_AW;
#[doc = "Field `TAMP2F` reader - TAMPER2 detection flag"]
pub use TAMP1F_R as TAMP2F_R;
#[doc = "Field `TAMP2F` writer - TAMPER2 detection flag"]
pub use TAMP1F_W as TAMP2F_W;
#[doc = "Recalibration pending Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECALPF_A {
    #[doc = "1: The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0"]
    Pending = 1,
}
impl From<RECALPF_A> for bool {
    #[inline(always)]
    fn from(variant: RECALPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECALPF` reader - Recalibration pending Flag"]
pub type RECALPF_R = crate::BitReader<RECALPF_A>;
impl RECALPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RECALPF_A> {
        match self.bits {
            true => Some(RECALPF_A::Pending),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pending`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECALPF_A::Pending
    }
}
impl R {
    #[doc = "Bit 0 - Alarm A write flag"]
    #[inline(always)]
    pub fn alrawf(&self) -> ALRAWF_R {
        ALRAWF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B write flag"]
    #[inline(always)]
    pub fn alrbwf(&self) -> ALRBWF_R {
        ALRBWF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer write flag"]
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Initialization status flag"]
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization flag"]
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    pub fn alrbf(&self) -> ALRBF_R {
        ALRBF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tamper detection flag"]
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TAMPER2 detection flag"]
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Recalibration pending Flag"]
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    pub fn shpf(&mut self) -> SHPF_W<3> {
        SHPF_W::new(self)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<5> {
        RSF_W::new(self)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W<7> {
        INIT_W::new(self)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    pub fn alraf(&mut self) -> ALRAF_W<8> {
        ALRAF_W::new(self)
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    pub fn alrbf(&mut self) -> ALRBF_W<9> {
        ALRBF_W::new(self)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    pub fn wutf(&mut self) -> WUTF_W<10> {
        WUTF_W::new(self)
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W<11> {
        TSF_W::new(self)
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    pub fn tsovf(&mut self) -> TSOVF_W<12> {
        TSOVF_W::new(self)
    }
    #[doc = "Bit 13 - Tamper detection flag"]
    #[inline(always)]
    pub fn tamp1f(&mut self) -> TAMP1F_W<13> {
        TAMP1F_W::new(self)
    }
    #[doc = "Bit 14 - TAMPER2 detection flag"]
    #[inline(always)]
    pub fn tamp2f(&mut self) -> TAMP2F_W<14> {
        TAMP2F_W::new(self)
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
