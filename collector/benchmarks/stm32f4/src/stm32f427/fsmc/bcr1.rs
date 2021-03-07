#[doc = "Reader of register BCR1"]
pub type R = crate::R<u32, super::BCR1>;
#[doc = "Writer for register BCR1"]
pub type W = crate::W<u32, super::BCR1>;
#[doc = "Register BCR1 `reset()`'s with value 0x30d0"]
impl crate::ResetValue for super::BCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x30d0
    }
}
#[doc = "Reader of field `CCLKEN`"]
pub type CCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCLKEN`"]
pub struct CCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLKEN_W<'a> {
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
#[doc = "CBURSTRW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CBURSTRW_A {
    #[doc = "1: Write operations are performed in synchronous mode"]
    ENABLED = 1,
    #[doc = "0: Write operations are always performed in asynchronous mode"]
    DISABLED = 0,
}
impl From<CBURSTRW_A> for bool {
    #[inline(always)]
    fn from(variant: CBURSTRW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CBURSTRW`"]
pub type CBURSTRW_R = crate::R<bool, CBURSTRW_A>;
impl CBURSTRW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBURSTRW_A {
        match self.bits {
            true => CBURSTRW_A::ENABLED,
            false => CBURSTRW_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CBURSTRW_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CBURSTRW_A::DISABLED
    }
}
#[doc = "Write proxy for field `CBURSTRW`"]
pub struct CBURSTRW_W<'a> {
    w: &'a mut W,
}
impl<'a> CBURSTRW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CBURSTRW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write operations are performed in synchronous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CBURSTRW_A::ENABLED)
    }
    #[doc = "Write operations are always performed in asynchronous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CBURSTRW_A::DISABLED)
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
#[doc = "CPSIZE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPSIZE_A {
    #[doc = "0: No burst split when crossing page boundary"]
    NOBURSTSPLIT = 0,
    #[doc = "1: 128 bytes CRAM page size"]
    BYTES128 = 1,
    #[doc = "2: 256 bytes CRAM page size"]
    BYTES256 = 2,
    #[doc = "3: 512 bytes CRAM page size"]
    BYTES512 = 3,
    #[doc = "4: 1024 bytes CRAM page size"]
    BYTES1024 = 4,
}
impl From<CPSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CPSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CPSIZE`"]
pub type CPSIZE_R = crate::R<u8, CPSIZE_A>;
impl CPSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CPSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CPSIZE_A::NOBURSTSPLIT),
            1 => Val(CPSIZE_A::BYTES128),
            2 => Val(CPSIZE_A::BYTES256),
            3 => Val(CPSIZE_A::BYTES512),
            4 => Val(CPSIZE_A::BYTES1024),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOBURSTSPLIT`"]
    #[inline(always)]
    pub fn is_no_burst_split(&self) -> bool {
        *self == CPSIZE_A::NOBURSTSPLIT
    }
    #[doc = "Checks if the value of the field is `BYTES128`"]
    #[inline(always)]
    pub fn is_bytes128(&self) -> bool {
        *self == CPSIZE_A::BYTES128
    }
    #[doc = "Checks if the value of the field is `BYTES256`"]
    #[inline(always)]
    pub fn is_bytes256(&self) -> bool {
        *self == CPSIZE_A::BYTES256
    }
    #[doc = "Checks if the value of the field is `BYTES512`"]
    #[inline(always)]
    pub fn is_bytes512(&self) -> bool {
        *self == CPSIZE_A::BYTES512
    }
    #[doc = "Checks if the value of the field is `BYTES1024`"]
    #[inline(always)]
    pub fn is_bytes1024(&self) -> bool {
        *self == CPSIZE_A::BYTES1024
    }
}
#[doc = "Write proxy for field `CPSIZE`"]
pub struct CPSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No burst split when crossing page boundary"]
    #[inline(always)]
    pub fn no_burst_split(self) -> &'a mut W {
        self.variant(CPSIZE_A::NOBURSTSPLIT)
    }
    #[doc = "128 bytes CRAM page size"]
    #[inline(always)]
    pub fn bytes128(self) -> &'a mut W {
        self.variant(CPSIZE_A::BYTES128)
    }
    #[doc = "256 bytes CRAM page size"]
    #[inline(always)]
    pub fn bytes256(self) -> &'a mut W {
        self.variant(CPSIZE_A::BYTES256)
    }
    #[doc = "512 bytes CRAM page size"]
    #[inline(always)]
    pub fn bytes512(self) -> &'a mut W {
        self.variant(CPSIZE_A::BYTES512)
    }
    #[doc = "1024 bytes CRAM page size"]
    #[inline(always)]
    pub fn bytes1024(self) -> &'a mut W {
        self.variant(CPSIZE_A::BYTES1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "ASYNCWAIT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASYNCWAIT_A {
    #[doc = "0: Wait signal not used in asynchronous mode"]
    DISABLED = 0,
    #[doc = "1: Wait signal used even in asynchronous mode"]
    ENABLED = 1,
}
impl From<ASYNCWAIT_A> for bool {
    #[inline(always)]
    fn from(variant: ASYNCWAIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASYNCWAIT`"]
pub type ASYNCWAIT_R = crate::R<bool, ASYNCWAIT_A>;
impl ASYNCWAIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASYNCWAIT_A {
        match self.bits {
            false => ASYNCWAIT_A::DISABLED,
            true => ASYNCWAIT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ASYNCWAIT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ASYNCWAIT_A::ENABLED
    }
}
#[doc = "Write proxy for field `ASYNCWAIT`"]
pub struct ASYNCWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNCWAIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASYNCWAIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wait signal not used in asynchronous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ASYNCWAIT_A::DISABLED)
    }
    #[doc = "Wait signal used even in asynchronous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ASYNCWAIT_A::ENABLED)
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
#[doc = "EXTMOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMOD_A {
    #[doc = "0: Values inside the FMC_BWTR are not taken into account"]
    DISABLED = 0,
    #[doc = "1: Values inside the FMC_BWTR are taken into account"]
    ENABLED = 1,
}
impl From<EXTMOD_A> for bool {
    #[inline(always)]
    fn from(variant: EXTMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTMOD`"]
pub type EXTMOD_R = crate::R<bool, EXTMOD_A>;
impl EXTMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTMOD_A {
        match self.bits {
            false => EXTMOD_A::DISABLED,
            true => EXTMOD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTMOD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXTMOD_A::ENABLED
    }
}
#[doc = "Write proxy for field `EXTMOD`"]
pub struct EXTMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTMOD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Values inside the FMC_BWTR are not taken into account"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTMOD_A::DISABLED)
    }
    #[doc = "Values inside the FMC_BWTR are taken into account"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EXTMOD_A::ENABLED)
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
#[doc = "WAITEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITEN_A {
    #[doc = "0: Values inside the FMC_BWTR are taken into account"]
    DISABLED = 0,
    #[doc = "1: NWAIT signal enabled"]
    ENABLED = 1,
}
impl From<WAITEN_A> for bool {
    #[inline(always)]
    fn from(variant: WAITEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAITEN`"]
pub type WAITEN_R = crate::R<bool, WAITEN_A>;
impl WAITEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITEN_A {
        match self.bits {
            false => WAITEN_A::DISABLED,
            true => WAITEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAITEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAITEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `WAITEN`"]
pub struct WAITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAITEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Values inside the FMC_BWTR are taken into account"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAITEN_A::DISABLED)
    }
    #[doc = "NWAIT signal enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAITEN_A::ENABLED)
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
#[doc = "WREN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WREN_A {
    #[doc = "0: Write operations disabled for the bank by the FMC"]
    DISABLED = 0,
    #[doc = "1: Write operations enabled for the bank by the FMC"]
    ENABLED = 1,
}
impl From<WREN_A> for bool {
    #[inline(always)]
    fn from(variant: WREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WREN`"]
pub type WREN_R = crate::R<bool, WREN_A>;
impl WREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WREN_A {
        match self.bits {
            false => WREN_A::DISABLED,
            true => WREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WREN_A::ENABLED
    }
}
#[doc = "Write proxy for field `WREN`"]
pub struct WREN_W<'a> {
    w: &'a mut W,
}
impl<'a> WREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write operations disabled for the bank by the FMC"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WREN_A::DISABLED)
    }
    #[doc = "Write operations enabled for the bank by the FMC"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WREN_A::ENABLED)
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
#[doc = "WAITCFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITCFG_A {
    #[doc = "0: NWAIT signal is active one data cycle before wait state"]
    BEFOREWAITSTATE = 0,
    #[doc = "1: NWAIT signal is active during wait state"]
    DURINGWAITSTATE = 1,
}
impl From<WAITCFG_A> for bool {
    #[inline(always)]
    fn from(variant: WAITCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAITCFG`"]
pub type WAITCFG_R = crate::R<bool, WAITCFG_A>;
impl WAITCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITCFG_A {
        match self.bits {
            false => WAITCFG_A::BEFOREWAITSTATE,
            true => WAITCFG_A::DURINGWAITSTATE,
        }
    }
    #[doc = "Checks if the value of the field is `BEFOREWAITSTATE`"]
    #[inline(always)]
    pub fn is_before_wait_state(&self) -> bool {
        *self == WAITCFG_A::BEFOREWAITSTATE
    }
    #[doc = "Checks if the value of the field is `DURINGWAITSTATE`"]
    #[inline(always)]
    pub fn is_during_wait_state(&self) -> bool {
        *self == WAITCFG_A::DURINGWAITSTATE
    }
}
#[doc = "Write proxy for field `WAITCFG`"]
pub struct WAITCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAITCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "NWAIT signal is active one data cycle before wait state"]
    #[inline(always)]
    pub fn before_wait_state(self) -> &'a mut W {
        self.variant(WAITCFG_A::BEFOREWAITSTATE)
    }
    #[doc = "NWAIT signal is active during wait state"]
    #[inline(always)]
    pub fn during_wait_state(self) -> &'a mut W {
        self.variant(WAITCFG_A::DURINGWAITSTATE)
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
#[doc = "Reader of field `WRAPMOD`"]
pub type WRAPMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRAPMOD`"]
pub struct WRAPMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> WRAPMOD_W<'a> {
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
#[doc = "WAITPOL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITPOL_A {
    #[doc = "0: NWAIT active low"]
    ACTIVELOW = 0,
    #[doc = "1: NWAIT active high"]
    ACTIVEHIGH = 1,
}
impl From<WAITPOL_A> for bool {
    #[inline(always)]
    fn from(variant: WAITPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAITPOL`"]
pub type WAITPOL_R = crate::R<bool, WAITPOL_A>;
impl WAITPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITPOL_A {
        match self.bits {
            false => WAITPOL_A::ACTIVELOW,
            true => WAITPOL_A::ACTIVEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVELOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == WAITPOL_A::ACTIVELOW
    }
    #[doc = "Checks if the value of the field is `ACTIVEHIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == WAITPOL_A::ACTIVEHIGH
    }
}
#[doc = "Write proxy for field `WAITPOL`"]
pub struct WAITPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAITPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "NWAIT active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(WAITPOL_A::ACTIVELOW)
    }
    #[doc = "NWAIT active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(WAITPOL_A::ACTIVEHIGH)
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
#[doc = "BURSTEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTEN_A {
    #[doc = "0: Burst mode disabled"]
    DISABLED = 0,
    #[doc = "1: Burst mode enabled"]
    ENABLED = 1,
}
impl From<BURSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: BURSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BURSTEN`"]
pub type BURSTEN_R = crate::R<bool, BURSTEN_A>;
impl BURSTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURSTEN_A {
        match self.bits {
            false => BURSTEN_A::DISABLED,
            true => BURSTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BURSTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BURSTEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `BURSTEN`"]
pub struct BURSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURSTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Burst mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BURSTEN_A::DISABLED)
    }
    #[doc = "Burst mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BURSTEN_A::ENABLED)
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
#[doc = "FACCEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FACCEN_A {
    #[doc = "0: Corresponding NOR Flash memory access is disabled"]
    DISABLED = 0,
    #[doc = "1: Corresponding NOR Flash memory access is enabled"]
    ENABLED = 1,
}
impl From<FACCEN_A> for bool {
    #[inline(always)]
    fn from(variant: FACCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FACCEN`"]
pub type FACCEN_R = crate::R<bool, FACCEN_A>;
impl FACCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FACCEN_A {
        match self.bits {
            false => FACCEN_A::DISABLED,
            true => FACCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FACCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FACCEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `FACCEN`"]
pub struct FACCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FACCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FACCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding NOR Flash memory access is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FACCEN_A::DISABLED)
    }
    #[doc = "Corresponding NOR Flash memory access is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FACCEN_A::ENABLED)
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
#[doc = "MWID\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MWID_A {
    #[doc = "0: Memory data bus width 8 bits"]
    BITS8 = 0,
    #[doc = "1: Memory data bus width 16 bits"]
    BITS16 = 1,
    #[doc = "2: Memory data bus width 32 bits"]
    BITS32 = 2,
}
impl From<MWID_A> for u8 {
    #[inline(always)]
    fn from(variant: MWID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MWID`"]
pub type MWID_R = crate::R<u8, MWID_A>;
impl MWID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MWID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MWID_A::BITS8),
            1 => Val(MWID_A::BITS16),
            2 => Val(MWID_A::BITS32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BITS8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == MWID_A::BITS8
    }
    #[doc = "Checks if the value of the field is `BITS16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == MWID_A::BITS16
    }
    #[doc = "Checks if the value of the field is `BITS32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == MWID_A::BITS32
    }
}
#[doc = "Write proxy for field `MWID`"]
pub struct MWID_W<'a> {
    w: &'a mut W,
}
impl<'a> MWID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MWID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Memory data bus width 8 bits"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(MWID_A::BITS8)
    }
    #[doc = "Memory data bus width 16 bits"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(MWID_A::BITS16)
    }
    #[doc = "Memory data bus width 32 bits"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(MWID_A::BITS32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "MTYP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MTYP_A {
    #[doc = "0: SRAM memory type"]
    SRAM = 0,
    #[doc = "1: PSRAM (CRAM) memory type"]
    PSRAM = 1,
    #[doc = "2: NOR Flash/OneNAND Flash"]
    FLASH = 2,
}
impl From<MTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: MTYP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MTYP`"]
pub type MTYP_R = crate::R<u8, MTYP_A>;
impl MTYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MTYP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MTYP_A::SRAM),
            1 => Val(MTYP_A::PSRAM),
            2 => Val(MTYP_A::FLASH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SRAM`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == MTYP_A::SRAM
    }
    #[doc = "Checks if the value of the field is `PSRAM`"]
    #[inline(always)]
    pub fn is_psram(&self) -> bool {
        *self == MTYP_A::PSRAM
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == MTYP_A::FLASH
    }
}
#[doc = "Write proxy for field `MTYP`"]
pub struct MTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> MTYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTYP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SRAM memory type"]
    #[inline(always)]
    pub fn sram(self) -> &'a mut W {
        self.variant(MTYP_A::SRAM)
    }
    #[doc = "PSRAM (CRAM) memory type"]
    #[inline(always)]
    pub fn psram(self) -> &'a mut W {
        self.variant(MTYP_A::PSRAM)
    }
    #[doc = "NOR Flash/OneNAND Flash"]
    #[inline(always)]
    pub fn flash(self) -> &'a mut W {
        self.variant(MTYP_A::FLASH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "MUXEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUXEN_A {
    #[doc = "0: Address/Data non-multiplexed"]
    DISABLED = 0,
    #[doc = "1: Address/Data multiplexed on databus"]
    ENABLED = 1,
}
impl From<MUXEN_A> for bool {
    #[inline(always)]
    fn from(variant: MUXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MUXEN`"]
pub type MUXEN_R = crate::R<bool, MUXEN_A>;
impl MUXEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUXEN_A {
        match self.bits {
            false => MUXEN_A::DISABLED,
            true => MUXEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MUXEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MUXEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `MUXEN`"]
pub struct MUXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUXEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Address/Data non-multiplexed"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MUXEN_A::DISABLED)
    }
    #[doc = "Address/Data multiplexed on databus"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MUXEN_A::ENABLED)
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
#[doc = "MBKEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBKEN_A {
    #[doc = "0: Corresponding memory bank is disabled"]
    DISABLED = 0,
    #[doc = "1: Corresponding memory bank is enabled"]
    ENABLED = 1,
}
impl From<MBKEN_A> for bool {
    #[inline(always)]
    fn from(variant: MBKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MBKEN`"]
pub type MBKEN_R = crate::R<bool, MBKEN_A>;
impl MBKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBKEN_A {
        match self.bits {
            false => MBKEN_A::DISABLED,
            true => MBKEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MBKEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MBKEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `MBKEN`"]
pub struct MBKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MBKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MBKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Corresponding memory bank is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MBKEN_A::DISABLED)
    }
    #[doc = "Corresponding memory bank is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MBKEN_A::ENABLED)
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
    #[doc = "Bit 20 - CCLKEN"]
    #[inline(always)]
    pub fn cclken(&self) -> CCLKEN_R {
        CCLKEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CBURSTRW"]
    #[inline(always)]
    pub fn cburstrw(&self) -> CBURSTRW_R {
        CBURSTRW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - CPSIZE"]
    #[inline(always)]
    pub fn cpsize(&self) -> CPSIZE_R {
        CPSIZE_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 15 - ASYNCWAIT"]
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EXTMOD"]
    #[inline(always)]
    pub fn extmod(&self) -> EXTMOD_R {
        EXTMOD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - WAITEN"]
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - WREN"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - WAITCFG"]
    #[inline(always)]
    pub fn waitcfg(&self) -> WAITCFG_R {
        WAITCFG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - WRAPMOD"]
    #[inline(always)]
    pub fn wrapmod(&self) -> WRAPMOD_R {
        WRAPMOD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - WAITPOL"]
    #[inline(always)]
    pub fn waitpol(&self) -> WAITPOL_R {
        WAITPOL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BURSTEN"]
    #[inline(always)]
    pub fn bursten(&self) -> BURSTEN_R {
        BURSTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FACCEN"]
    #[inline(always)]
    pub fn faccen(&self) -> FACCEN_R {
        FACCEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - MWID"]
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - MTYP"]
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - MUXEN"]
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - MBKEN"]
    #[inline(always)]
    pub fn mbken(&self) -> MBKEN_R {
        MBKEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - CCLKEN"]
    #[inline(always)]
    pub fn cclken(&mut self) -> CCLKEN_W {
        CCLKEN_W { w: self }
    }
    #[doc = "Bit 19 - CBURSTRW"]
    #[inline(always)]
    pub fn cburstrw(&mut self) -> CBURSTRW_W {
        CBURSTRW_W { w: self }
    }
    #[doc = "Bits 16:18 - CPSIZE"]
    #[inline(always)]
    pub fn cpsize(&mut self) -> CPSIZE_W {
        CPSIZE_W { w: self }
    }
    #[doc = "Bit 15 - ASYNCWAIT"]
    #[inline(always)]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W {
        ASYNCWAIT_W { w: self }
    }
    #[doc = "Bit 14 - EXTMOD"]
    #[inline(always)]
    pub fn extmod(&mut self) -> EXTMOD_W {
        EXTMOD_W { w: self }
    }
    #[doc = "Bit 13 - WAITEN"]
    #[inline(always)]
    pub fn waiten(&mut self) -> WAITEN_W {
        WAITEN_W { w: self }
    }
    #[doc = "Bit 12 - WREN"]
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W {
        WREN_W { w: self }
    }
    #[doc = "Bit 11 - WAITCFG"]
    #[inline(always)]
    pub fn waitcfg(&mut self) -> WAITCFG_W {
        WAITCFG_W { w: self }
    }
    #[doc = "Bit 10 - WRAPMOD"]
    #[inline(always)]
    pub fn wrapmod(&mut self) -> WRAPMOD_W {
        WRAPMOD_W { w: self }
    }
    #[doc = "Bit 9 - WAITPOL"]
    #[inline(always)]
    pub fn waitpol(&mut self) -> WAITPOL_W {
        WAITPOL_W { w: self }
    }
    #[doc = "Bit 8 - BURSTEN"]
    #[inline(always)]
    pub fn bursten(&mut self) -> BURSTEN_W {
        BURSTEN_W { w: self }
    }
    #[doc = "Bit 6 - FACCEN"]
    #[inline(always)]
    pub fn faccen(&mut self) -> FACCEN_W {
        FACCEN_W { w: self }
    }
    #[doc = "Bits 4:5 - MWID"]
    #[inline(always)]
    pub fn mwid(&mut self) -> MWID_W {
        MWID_W { w: self }
    }
    #[doc = "Bits 2:3 - MTYP"]
    #[inline(always)]
    pub fn mtyp(&mut self) -> MTYP_W {
        MTYP_W { w: self }
    }
    #[doc = "Bit 1 - MUXEN"]
    #[inline(always)]
    pub fn muxen(&mut self) -> MUXEN_W {
        MUXEN_W { w: self }
    }
    #[doc = "Bit 0 - MBKEN"]
    #[inline(always)]
    pub fn mbken(&mut self) -> MBKEN_W {
        MBKEN_W { w: self }
    }
}
