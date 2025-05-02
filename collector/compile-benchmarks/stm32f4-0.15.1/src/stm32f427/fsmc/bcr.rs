#[doc = "Register `BCR%s` reader"]
pub struct R(crate::R<BCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCR%s` writer"]
pub struct W(crate::W<BCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCR_SPEC>;
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
impl From<crate::W<BCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CBURSTRW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CBURSTRW_A {
    #[doc = "0: Write operations are always performed in asynchronous mode"]
    Disabled = 0,
    #[doc = "1: Write operations are performed in synchronous mode"]
    Enabled = 1,
}
impl From<CBURSTRW_A> for bool {
    #[inline(always)]
    fn from(variant: CBURSTRW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBURSTRW` reader - CBURSTRW"]
pub type CBURSTRW_R = crate::BitReader<CBURSTRW_A>;
impl CBURSTRW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBURSTRW_A {
        match self.bits {
            false => CBURSTRW_A::Disabled,
            true => CBURSTRW_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CBURSTRW_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CBURSTRW_A::Enabled
    }
}
#[doc = "Field `CBURSTRW` writer - CBURSTRW"]
pub type CBURSTRW_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR_SPEC, CBURSTRW_A, O>;
impl<'a, const O: u8> CBURSTRW_W<'a, O> {
    #[doc = "Write operations are always performed in asynchronous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CBURSTRW_A::Disabled)
    }
    #[doc = "Write operations are performed in synchronous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CBURSTRW_A::Enabled)
    }
}
#[doc = "CPSIZE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPSIZE_A {
    #[doc = "0: No burst split when crossing page boundary"]
    NoBurstSplit = 0,
    #[doc = "1: 128 bytes CRAM page size"]
    Bytes128 = 1,
    #[doc = "2: 256 bytes CRAM page size"]
    Bytes256 = 2,
    #[doc = "3: 512 bytes CRAM page size"]
    Bytes512 = 3,
    #[doc = "4: 1024 bytes CRAM page size"]
    Bytes1024 = 4,
}
impl From<CPSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CPSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CPSIZE` reader - CPSIZE"]
pub type CPSIZE_R = crate::FieldReader<u8, CPSIZE_A>;
impl CPSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPSIZE_A> {
        match self.bits {
            0 => Some(CPSIZE_A::NoBurstSplit),
            1 => Some(CPSIZE_A::Bytes128),
            2 => Some(CPSIZE_A::Bytes256),
            3 => Some(CPSIZE_A::Bytes512),
            4 => Some(CPSIZE_A::Bytes1024),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoBurstSplit`"]
    #[inline(always)]
    pub fn is_no_burst_split(&self) -> bool {
        *self == CPSIZE_A::NoBurstSplit
    }
    #[doc = "Checks if the value of the field is `Bytes128`"]
    #[inline(always)]
    pub fn is_bytes128(&self) -> bool {
        *self == CPSIZE_A::Bytes128
    }
    #[doc = "Checks if the value of the field is `Bytes256`"]
    #[inline(always)]
    pub fn is_bytes256(&self) -> bool {
        *self == CPSIZE_A::Bytes256
    }
    #[doc = "Checks if the value of the field is `Bytes512`"]
    #[inline(always)]
    pub fn is_bytes512(&self) -> bool {
        *self == CPSIZE_A::Bytes512
    }
    #[doc = "Checks if the value of the field is `Bytes1024`"]
    #[inline(always)]
    pub fn is_bytes1024(&self) -> bool {
        *self == CPSIZE_A::Bytes1024
    }
}
#[doc = "Field `CPSIZE` writer - CPSIZE"]
pub type CPSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR_SPEC, u8, CPSIZE_A, 3, O>;
impl<'a, const O: u8> CPSIZE_W<'a, O> {
    #[doc = "No burst split when crossing page boundary"]
    #[inline(always)]
    pub fn no_burst_split(self) -> &'a mut W {
        self.variant(CPSIZE_A::NoBurstSplit)
    }
    #[doc = "128 bytes CRAM page size"]
    #[inline(always)]
    pub fn bytes128(self) -> &'a mut W {
        self.variant(CPSIZE_A::Bytes128)
    }
    #[doc = "256 bytes CRAM page size"]
    #[inline(always)]
    pub fn bytes256(self) -> &'a mut W {
        self.variant(CPSIZE_A::Bytes256)
    }
    #[doc = "512 bytes CRAM page size"]
    #[inline(always)]
    pub fn bytes512(self) -> &'a mut W {
        self.variant(CPSIZE_A::Bytes512)
    }
    #[doc = "1024 bytes CRAM page size"]
    #[inline(always)]
    pub fn bytes1024(self) -> &'a mut W {
        self.variant(CPSIZE_A::Bytes1024)
    }
}
#[doc = "ASYNCWAIT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASYNCWAIT_A {
    #[doc = "0: Wait signal not used in asynchronous mode"]
    Disabled = 0,
    #[doc = "1: Wait signal used even in asynchronous mode"]
    Enabled = 1,
}
impl From<ASYNCWAIT_A> for bool {
    #[inline(always)]
    fn from(variant: ASYNCWAIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASYNCWAIT` reader - ASYNCWAIT"]
pub type ASYNCWAIT_R = crate::BitReader<ASYNCWAIT_A>;
impl ASYNCWAIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASYNCWAIT_A {
        match self.bits {
            false => ASYNCWAIT_A::Disabled,
            true => ASYNCWAIT_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ASYNCWAIT_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ASYNCWAIT_A::Enabled
    }
}
#[doc = "Field `ASYNCWAIT` writer - ASYNCWAIT"]
pub type ASYNCWAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR_SPEC, ASYNCWAIT_A, O>;
impl<'a, const O: u8> ASYNCWAIT_W<'a, O> {
    #[doc = "Wait signal not used in asynchronous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ASYNCWAIT_A::Disabled)
    }
    #[doc = "Wait signal used even in asynchronous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ASYNCWAIT_A::Enabled)
    }
}
#[doc = "EXTMOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMOD_A {
    #[doc = "0: Values inside the FMC_BWTR are not taken into account"]
    Disabled = 0,
    #[doc = "1: Values inside the FMC_BWTR are taken into account"]
    Enabled = 1,
}
impl From<EXTMOD_A> for bool {
    #[inline(always)]
    fn from(variant: EXTMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMOD` reader - EXTMOD"]
pub type EXTMOD_R = crate::BitReader<EXTMOD_A>;
impl EXTMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTMOD_A {
        match self.bits {
            false => EXTMOD_A::Disabled,
            true => EXTMOD_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTMOD_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXTMOD_A::Enabled
    }
}
#[doc = "Field `EXTMOD` writer - EXTMOD"]
pub type EXTMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR_SPEC, EXTMOD_A, O>;
impl<'a, const O: u8> EXTMOD_W<'a, O> {
    #[doc = "Values inside the FMC_BWTR are not taken into account"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTMOD_A::Disabled)
    }
    #[doc = "Values inside the FMC_BWTR are taken into account"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EXTMOD_A::Enabled)
    }
}
#[doc = "WAITEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITEN_A {
    #[doc = "0: Values inside the FMC_BWTR are taken into account"]
    Disabled = 0,
    #[doc = "1: NWAIT signal enabled"]
    Enabled = 1,
}
impl From<WAITEN_A> for bool {
    #[inline(always)]
    fn from(variant: WAITEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAITEN` reader - WAITEN"]
pub type WAITEN_R = crate::BitReader<WAITEN_A>;
impl WAITEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITEN_A {
        match self.bits {
            false => WAITEN_A::Disabled,
            true => WAITEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAITEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAITEN_A::Enabled
    }
}
#[doc = "Field `WAITEN` writer - WAITEN"]
pub type WAITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR_SPEC, WAITEN_A, O>;
impl<'a, const O: u8> WAITEN_W<'a, O> {
    #[doc = "Values inside the FMC_BWTR are taken into account"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAITEN_A::Disabled)
    }
    #[doc = "NWAIT signal enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAITEN_A::Enabled)
    }
}
#[doc = "WREN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WREN_A {
    #[doc = "0: Write operations disabled for the bank by the FMC"]
    Disabled = 0,
    #[doc = "1: Write operations enabled for the bank by the FMC"]
    Enabled = 1,
}
impl From<WREN_A> for bool {
    #[inline(always)]
    fn from(variant: WREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WREN` reader - WREN"]
pub type WREN_R = crate::BitReader<WREN_A>;
impl WREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WREN_A {
        match self.bits {
            false => WREN_A::Disabled,
            true => WREN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WREN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WREN_A::Enabled
    }
}
#[doc = "Field `WREN` writer - WREN"]
pub type WREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR_SPEC, WREN_A, O>;
impl<'a, const O: u8> WREN_W<'a, O> {
    #[doc = "Write operations disabled for the bank by the FMC"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WREN_A::Disabled)
    }
    #[doc = "Write operations enabled for the bank by the FMC"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WREN_A::Enabled)
    }
}
#[doc = "WAITCFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITCFG_A {
    #[doc = "0: NWAIT signal is active one data cycle before wait state"]
    BeforeWaitState = 0,
    #[doc = "1: NWAIT signal is active during wait state"]
    DuringWaitState = 1,
}
impl From<WAITCFG_A> for bool {
    #[inline(always)]
    fn from(variant: WAITCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAITCFG` reader - WAITCFG"]
pub type WAITCFG_R = crate::BitReader<WAITCFG_A>;
impl WAITCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITCFG_A {
        match self.bits {
            false => WAITCFG_A::BeforeWaitState,
            true => WAITCFG_A::DuringWaitState,
        }
    }
    #[doc = "Checks if the value of the field is `BeforeWaitState`"]
    #[inline(always)]
    pub fn is_before_wait_state(&self) -> bool {
        *self == WAITCFG_A::BeforeWaitState
    }
    #[doc = "Checks if the value of the field is `DuringWaitState`"]
    #[inline(always)]
    pub fn is_during_wait_state(&self) -> bool {
        *self == WAITCFG_A::DuringWaitState
    }
}
#[doc = "Field `WAITCFG` writer - WAITCFG"]
pub type WAITCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR_SPEC, WAITCFG_A, O>;
impl<'a, const O: u8> WAITCFG_W<'a, O> {
    #[doc = "NWAIT signal is active one data cycle before wait state"]
    #[inline(always)]
    pub fn before_wait_state(self) -> &'a mut W {
        self.variant(WAITCFG_A::BeforeWaitState)
    }
    #[doc = "NWAIT signal is active during wait state"]
    #[inline(always)]
    pub fn during_wait_state(self) -> &'a mut W {
        self.variant(WAITCFG_A::DuringWaitState)
    }
}
#[doc = "Field `WRAPMOD` reader - WRAPMOD"]
pub type WRAPMOD_R = crate::BitReader<bool>;
#[doc = "Field `WRAPMOD` writer - WRAPMOD"]
pub type WRAPMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR_SPEC, bool, O>;
#[doc = "WAITPOL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITPOL_A {
    #[doc = "0: NWAIT active low"]
    ActiveLow = 0,
    #[doc = "1: NWAIT active high"]
    ActiveHigh = 1,
}
impl From<WAITPOL_A> for bool {
    #[inline(always)]
    fn from(variant: WAITPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAITPOL` reader - WAITPOL"]
pub type WAITPOL_R = crate::BitReader<WAITPOL_A>;
impl WAITPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITPOL_A {
        match self.bits {
            false => WAITPOL_A::ActiveLow,
            true => WAITPOL_A::ActiveHigh,
        }
    }
    #[doc = "Checks if the value of the field is `ActiveLow`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == WAITPOL_A::ActiveLow
    }
    #[doc = "Checks if the value of the field is `ActiveHigh`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == WAITPOL_A::ActiveHigh
    }
}
#[doc = "Field `WAITPOL` writer - WAITPOL"]
pub type WAITPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR_SPEC, WAITPOL_A, O>;
impl<'a, const O: u8> WAITPOL_W<'a, O> {
    #[doc = "NWAIT active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(WAITPOL_A::ActiveLow)
    }
    #[doc = "NWAIT active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(WAITPOL_A::ActiveHigh)
    }
}
#[doc = "BURSTEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTEN_A {
    #[doc = "0: Burst mode disabled"]
    Disabled = 0,
    #[doc = "1: Burst mode enabled"]
    Enabled = 1,
}
impl From<BURSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: BURSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BURSTEN` reader - BURSTEN"]
pub type BURSTEN_R = crate::BitReader<BURSTEN_A>;
impl BURSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURSTEN_A {
        match self.bits {
            false => BURSTEN_A::Disabled,
            true => BURSTEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BURSTEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BURSTEN_A::Enabled
    }
}
#[doc = "Field `BURSTEN` writer - BURSTEN"]
pub type BURSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR_SPEC, BURSTEN_A, O>;
impl<'a, const O: u8> BURSTEN_W<'a, O> {
    #[doc = "Burst mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BURSTEN_A::Disabled)
    }
    #[doc = "Burst mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BURSTEN_A::Enabled)
    }
}
#[doc = "FACCEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FACCEN_A {
    #[doc = "0: Corresponding NOR Flash memory access is disabled"]
    Disabled = 0,
    #[doc = "1: Corresponding NOR Flash memory access is enabled"]
    Enabled = 1,
}
impl From<FACCEN_A> for bool {
    #[inline(always)]
    fn from(variant: FACCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FACCEN` reader - FACCEN"]
pub type FACCEN_R = crate::BitReader<FACCEN_A>;
impl FACCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FACCEN_A {
        match self.bits {
            false => FACCEN_A::Disabled,
            true => FACCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FACCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FACCEN_A::Enabled
    }
}
#[doc = "Field `FACCEN` writer - FACCEN"]
pub type FACCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR_SPEC, FACCEN_A, O>;
impl<'a, const O: u8> FACCEN_W<'a, O> {
    #[doc = "Corresponding NOR Flash memory access is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FACCEN_A::Disabled)
    }
    #[doc = "Corresponding NOR Flash memory access is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FACCEN_A::Enabled)
    }
}
#[doc = "MWID\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MWID_A {
    #[doc = "0: Memory data bus width 8 bits"]
    Bits8 = 0,
    #[doc = "1: Memory data bus width 16 bits"]
    Bits16 = 1,
    #[doc = "2: Memory data bus width 32 bits"]
    Bits32 = 2,
}
impl From<MWID_A> for u8 {
    #[inline(always)]
    fn from(variant: MWID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MWID` reader - MWID"]
pub type MWID_R = crate::FieldReader<u8, MWID_A>;
impl MWID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MWID_A> {
        match self.bits {
            0 => Some(MWID_A::Bits8),
            1 => Some(MWID_A::Bits16),
            2 => Some(MWID_A::Bits32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Bits8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == MWID_A::Bits8
    }
    #[doc = "Checks if the value of the field is `Bits16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == MWID_A::Bits16
    }
    #[doc = "Checks if the value of the field is `Bits32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == MWID_A::Bits32
    }
}
#[doc = "Field `MWID` writer - MWID"]
pub type MWID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR_SPEC, u8, MWID_A, 2, O>;
impl<'a, const O: u8> MWID_W<'a, O> {
    #[doc = "Memory data bus width 8 bits"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(MWID_A::Bits8)
    }
    #[doc = "Memory data bus width 16 bits"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(MWID_A::Bits16)
    }
    #[doc = "Memory data bus width 32 bits"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(MWID_A::Bits32)
    }
}
#[doc = "MTYP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MTYP_A {
    #[doc = "0: SRAM memory type"]
    Sram = 0,
    #[doc = "1: PSRAM (CRAM) memory type"]
    Psram = 1,
    #[doc = "2: NOR Flash/OneNAND Flash"]
    Flash = 2,
}
impl From<MTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: MTYP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MTYP` reader - MTYP"]
pub type MTYP_R = crate::FieldReader<u8, MTYP_A>;
impl MTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MTYP_A> {
        match self.bits {
            0 => Some(MTYP_A::Sram),
            1 => Some(MTYP_A::Psram),
            2 => Some(MTYP_A::Flash),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Sram`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == MTYP_A::Sram
    }
    #[doc = "Checks if the value of the field is `Psram`"]
    #[inline(always)]
    pub fn is_psram(&self) -> bool {
        *self == MTYP_A::Psram
    }
    #[doc = "Checks if the value of the field is `Flash`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == MTYP_A::Flash
    }
}
#[doc = "Field `MTYP` writer - MTYP"]
pub type MTYP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR_SPEC, u8, MTYP_A, 2, O>;
impl<'a, const O: u8> MTYP_W<'a, O> {
    #[doc = "SRAM memory type"]
    #[inline(always)]
    pub fn sram(self) -> &'a mut W {
        self.variant(MTYP_A::Sram)
    }
    #[doc = "PSRAM (CRAM) memory type"]
    #[inline(always)]
    pub fn psram(self) -> &'a mut W {
        self.variant(MTYP_A::Psram)
    }
    #[doc = "NOR Flash/OneNAND Flash"]
    #[inline(always)]
    pub fn flash(self) -> &'a mut W {
        self.variant(MTYP_A::Flash)
    }
}
#[doc = "MUXEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUXEN_A {
    #[doc = "0: Address/Data non-multiplexed"]
    Disabled = 0,
    #[doc = "1: Address/Data multiplexed on databus"]
    Enabled = 1,
}
impl From<MUXEN_A> for bool {
    #[inline(always)]
    fn from(variant: MUXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUXEN` reader - MUXEN"]
pub type MUXEN_R = crate::BitReader<MUXEN_A>;
impl MUXEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUXEN_A {
        match self.bits {
            false => MUXEN_A::Disabled,
            true => MUXEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MUXEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MUXEN_A::Enabled
    }
}
#[doc = "Field `MUXEN` writer - MUXEN"]
pub type MUXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR_SPEC, MUXEN_A, O>;
impl<'a, const O: u8> MUXEN_W<'a, O> {
    #[doc = "Address/Data non-multiplexed"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MUXEN_A::Disabled)
    }
    #[doc = "Address/Data multiplexed on databus"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MUXEN_A::Enabled)
    }
}
#[doc = "MBKEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBKEN_A {
    #[doc = "0: Corresponding memory bank is disabled"]
    Disabled = 0,
    #[doc = "1: Corresponding memory bank is enabled"]
    Enabled = 1,
}
impl From<MBKEN_A> for bool {
    #[inline(always)]
    fn from(variant: MBKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBKEN` reader - MBKEN"]
pub type MBKEN_R = crate::BitReader<MBKEN_A>;
impl MBKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBKEN_A {
        match self.bits {
            false => MBKEN_A::Disabled,
            true => MBKEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MBKEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MBKEN_A::Enabled
    }
}
#[doc = "Field `MBKEN` writer - MBKEN"]
pub type MBKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR_SPEC, MBKEN_A, O>;
impl<'a, const O: u8> MBKEN_W<'a, O> {
    #[doc = "Corresponding memory bank is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MBKEN_A::Disabled)
    }
    #[doc = "Corresponding memory bank is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MBKEN_A::Enabled)
    }
}
#[doc = "Field `CCLKEN` reader - CCLKEN"]
pub type CCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `CCLKEN` writer - CCLKEN"]
pub type CCLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 19 - CBURSTRW"]
    #[inline(always)]
    pub fn cburstrw(&self) -> CBURSTRW_R {
        CBURSTRW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 16:18 - CPSIZE"]
    #[inline(always)]
    pub fn cpsize(&self) -> CPSIZE_R {
        CPSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 15 - ASYNCWAIT"]
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - EXTMOD"]
    #[inline(always)]
    pub fn extmod(&self) -> EXTMOD_R {
        EXTMOD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - WAITEN"]
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - WREN"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - WAITCFG"]
    #[inline(always)]
    pub fn waitcfg(&self) -> WAITCFG_R {
        WAITCFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - WRAPMOD"]
    #[inline(always)]
    pub fn wrapmod(&self) -> WRAPMOD_R {
        WRAPMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - WAITPOL"]
    #[inline(always)]
    pub fn waitpol(&self) -> WAITPOL_R {
        WAITPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - BURSTEN"]
    #[inline(always)]
    pub fn bursten(&self) -> BURSTEN_R {
        BURSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 6 - FACCEN"]
    #[inline(always)]
    pub fn faccen(&self) -> FACCEN_R {
        FACCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 4:5 - MWID"]
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 2:3 - MTYP"]
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 1 - MUXEN"]
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - MBKEN"]
    #[inline(always)]
    pub fn mbken(&self) -> MBKEN_R {
        MBKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 20 - CCLKEN"]
    #[inline(always)]
    pub fn cclken(&self) -> CCLKEN_R {
        CCLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 19 - CBURSTRW"]
    #[inline(always)]
    pub fn cburstrw(&mut self) -> CBURSTRW_W<19> {
        CBURSTRW_W::new(self)
    }
    #[doc = "Bits 16:18 - CPSIZE"]
    #[inline(always)]
    pub fn cpsize(&mut self) -> CPSIZE_W<16> {
        CPSIZE_W::new(self)
    }
    #[doc = "Bit 15 - ASYNCWAIT"]
    #[inline(always)]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W<15> {
        ASYNCWAIT_W::new(self)
    }
    #[doc = "Bit 14 - EXTMOD"]
    #[inline(always)]
    pub fn extmod(&mut self) -> EXTMOD_W<14> {
        EXTMOD_W::new(self)
    }
    #[doc = "Bit 13 - WAITEN"]
    #[inline(always)]
    pub fn waiten(&mut self) -> WAITEN_W<13> {
        WAITEN_W::new(self)
    }
    #[doc = "Bit 12 - WREN"]
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W<12> {
        WREN_W::new(self)
    }
    #[doc = "Bit 11 - WAITCFG"]
    #[inline(always)]
    pub fn waitcfg(&mut self) -> WAITCFG_W<11> {
        WAITCFG_W::new(self)
    }
    #[doc = "Bit 10 - WRAPMOD"]
    #[inline(always)]
    pub fn wrapmod(&mut self) -> WRAPMOD_W<10> {
        WRAPMOD_W::new(self)
    }
    #[doc = "Bit 9 - WAITPOL"]
    #[inline(always)]
    pub fn waitpol(&mut self) -> WAITPOL_W<9> {
        WAITPOL_W::new(self)
    }
    #[doc = "Bit 8 - BURSTEN"]
    #[inline(always)]
    pub fn bursten(&mut self) -> BURSTEN_W<8> {
        BURSTEN_W::new(self)
    }
    #[doc = "Bit 6 - FACCEN"]
    #[inline(always)]
    pub fn faccen(&mut self) -> FACCEN_W<6> {
        FACCEN_W::new(self)
    }
    #[doc = "Bits 4:5 - MWID"]
    #[inline(always)]
    pub fn mwid(&mut self) -> MWID_W<4> {
        MWID_W::new(self)
    }
    #[doc = "Bits 2:3 - MTYP"]
    #[inline(always)]
    pub fn mtyp(&mut self) -> MTYP_W<2> {
        MTYP_W::new(self)
    }
    #[doc = "Bit 1 - MUXEN"]
    #[inline(always)]
    pub fn muxen(&mut self) -> MUXEN_W<1> {
        MUXEN_W::new(self)
    }
    #[doc = "Bit 0 - MBKEN"]
    #[inline(always)]
    pub fn mbken(&mut self) -> MBKEN_W<0> {
        MBKEN_W::new(self)
    }
    #[doc = "Bit 20 - CCLKEN"]
    #[inline(always)]
    pub fn cclken(&mut self) -> CCLKEN_W<20> {
        CCLKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM/NOR-Flash chip-select control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr](index.html) module"]
pub struct BCR_SPEC;
impl crate::RegisterSpec for BCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcr::R](R) reader structure"]
impl crate::Readable for BCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcr::W](W) writer structure"]
impl crate::Writable for BCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCR%s to value 0x30d0"]
impl crate::Resettable for BCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30d0
    }
}
