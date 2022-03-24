#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHSEL` reader - Channel selection"]
pub struct CHSEL_R(crate::FieldReader<u8, u8>);
impl CHSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL` writer - Channel selection"]
pub struct CHSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | ((value as u32 & 0x07) << 25);
        self.w
    }
}
#[doc = "Memory burst transfer configuration"]
pub type MBURST_A = PBURST_A;
#[doc = "Field `MBURST` reader - Memory burst transfer configuration"]
pub type MBURST_R = PBURST_R;
#[doc = "Field `MBURST` writer - Memory burst transfer configuration"]
pub struct MBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> MBURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MBURST_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Single transfer"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(MBURST_A::SINGLE)
    }
    #[doc = "Incremental burst of 4 beats"]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut W {
        self.variant(MBURST_A::INCR4)
    }
    #[doc = "Incremental burst of 8 beats"]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut W {
        self.variant(MBURST_A::INCR8)
    }
    #[doc = "Incremental burst of 16 beats"]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut W {
        self.variant(MBURST_A::INCR16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | ((value as u32 & 0x03) << 23);
        self.w
    }
}
#[doc = "Peripheral burst transfer configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PBURST_A {
    #[doc = "0: Single transfer"]
    SINGLE = 0,
    #[doc = "1: Incremental burst of 4 beats"]
    INCR4 = 1,
    #[doc = "2: Incremental burst of 8 beats"]
    INCR8 = 2,
    #[doc = "3: Incremental burst of 16 beats"]
    INCR16 = 3,
}
impl From<PBURST_A> for u8 {
    #[inline(always)]
    fn from(variant: PBURST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PBURST` reader - Peripheral burst transfer configuration"]
pub struct PBURST_R(crate::FieldReader<u8, PBURST_A>);
impl PBURST_R {
    pub(crate) fn new(bits: u8) -> Self {
        PBURST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBURST_A {
        match self.bits {
            0 => PBURST_A::SINGLE,
            1 => PBURST_A::INCR4,
            2 => PBURST_A::INCR8,
            3 => PBURST_A::INCR16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == PBURST_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `INCR4`"]
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        **self == PBURST_A::INCR4
    }
    #[doc = "Checks if the value of the field is `INCR8`"]
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        **self == PBURST_A::INCR8
    }
    #[doc = "Checks if the value of the field is `INCR16`"]
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        **self == PBURST_A::INCR16
    }
}
impl core::ops::Deref for PBURST_R {
    type Target = crate::FieldReader<u8, PBURST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBURST` writer - Peripheral burst transfer configuration"]
pub struct PBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> PBURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PBURST_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Single transfer"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(PBURST_A::SINGLE)
    }
    #[doc = "Incremental burst of 4 beats"]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut W {
        self.variant(PBURST_A::INCR4)
    }
    #[doc = "Incremental burst of 8 beats"]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut W {
        self.variant(PBURST_A::INCR8)
    }
    #[doc = "Incremental burst of 16 beats"]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut W {
        self.variant(PBURST_A::INCR16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Current target (only in double buffer mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT_A {
    #[doc = "0: The current target memory is Memory 0"]
    MEMORY0 = 0,
    #[doc = "1: The current target memory is Memory 1"]
    MEMORY1 = 1,
}
impl From<CT_A> for bool {
    #[inline(always)]
    fn from(variant: CT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT` reader - Current target (only in double buffer mode)"]
pub struct CT_R(crate::FieldReader<bool, CT_A>);
impl CT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT_A {
        match self.bits {
            false => CT_A::MEMORY0,
            true => CT_A::MEMORY1,
        }
    }
    #[doc = "Checks if the value of the field is `MEMORY0`"]
    #[inline(always)]
    pub fn is_memory0(&self) -> bool {
        **self == CT_A::MEMORY0
    }
    #[doc = "Checks if the value of the field is `MEMORY1`"]
    #[inline(always)]
    pub fn is_memory1(&self) -> bool {
        **self == CT_A::MEMORY1
    }
}
impl core::ops::Deref for CT_R {
    type Target = crate::FieldReader<bool, CT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CT` writer - Current target (only in double buffer mode)"]
pub struct CT_W<'a> {
    w: &'a mut W,
}
impl<'a> CT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The current target memory is Memory 0"]
    #[inline(always)]
    pub fn memory0(self) -> &'a mut W {
        self.variant(CT_A::MEMORY0)
    }
    #[doc = "The current target memory is Memory 1"]
    #[inline(always)]
    pub fn memory1(self) -> &'a mut W {
        self.variant(CT_A::MEMORY1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Double buffer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBM_A {
    #[doc = "0: No buffer switching at the end of transfer"]
    DISABLED = 0,
    #[doc = "1: Memory target switched at the end of the DMA transfer"]
    ENABLED = 1,
}
impl From<DBM_A> for bool {
    #[inline(always)]
    fn from(variant: DBM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBM` reader - Double buffer mode"]
pub struct DBM_R(crate::FieldReader<bool, DBM_A>);
impl DBM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBM_A {
        match self.bits {
            false => DBM_A::DISABLED,
            true => DBM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DBM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DBM_A::ENABLED
    }
}
impl core::ops::Deref for DBM_R {
    type Target = crate::FieldReader<bool, DBM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBM` writer - Double buffer mode"]
pub struct DBM_W<'a> {
    w: &'a mut W,
}
impl<'a> DBM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No buffer switching at the end of transfer"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBM_A::DISABLED)
    }
    #[doc = "Memory target switched at the end of the DMA transfer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBM_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Priority level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PL_A {
    #[doc = "0: Low"]
    LOW = 0,
    #[doc = "1: Medium"]
    MEDIUM = 1,
    #[doc = "2: High"]
    HIGH = 2,
    #[doc = "3: Very high"]
    VERYHIGH = 3,
}
impl From<PL_A> for u8 {
    #[inline(always)]
    fn from(variant: PL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PL` reader - Priority level"]
pub struct PL_R(crate::FieldReader<u8, PL_A>);
impl PL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PL_A {
        match self.bits {
            0 => PL_A::LOW,
            1 => PL_A::MEDIUM,
            2 => PL_A::HIGH,
            3 => PL_A::VERYHIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PL_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        **self == PL_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `VERYHIGH`"]
    #[inline(always)]
    pub fn is_very_high(&self) -> bool {
        **self == PL_A::VERYHIGH
    }
}
impl core::ops::Deref for PL_R {
    type Target = crate::FieldReader<u8, PL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PL` writer - Priority level"]
pub struct PL_W<'a> {
    w: &'a mut W,
}
impl<'a> PL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PL_A::LOW)
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(PL_A::MEDIUM)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PL_A::HIGH)
    }
    #[doc = "Very high"]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(PL_A::VERYHIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Peripheral increment offset size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINCOS_A {
    #[doc = "0: The offset size for the peripheral address calculation is linked to the PSIZE"]
    PSIZE = 0,
    #[doc = "1: The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)"]
    FIXED4 = 1,
}
impl From<PINCOS_A> for bool {
    #[inline(always)]
    fn from(variant: PINCOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINCOS` reader - Peripheral increment offset size"]
pub struct PINCOS_R(crate::FieldReader<bool, PINCOS_A>);
impl PINCOS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINCOS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINCOS_A {
        match self.bits {
            false => PINCOS_A::PSIZE,
            true => PINCOS_A::FIXED4,
        }
    }
    #[doc = "Checks if the value of the field is `PSIZE`"]
    #[inline(always)]
    pub fn is_psize(&self) -> bool {
        **self == PINCOS_A::PSIZE
    }
    #[doc = "Checks if the value of the field is `FIXED4`"]
    #[inline(always)]
    pub fn is_fixed4(&self) -> bool {
        **self == PINCOS_A::FIXED4
    }
}
impl core::ops::Deref for PINCOS_R {
    type Target = crate::FieldReader<bool, PINCOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINCOS` writer - Peripheral increment offset size"]
pub struct PINCOS_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINCOS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The offset size for the peripheral address calculation is linked to the PSIZE"]
    #[inline(always)]
    pub fn psize(self) -> &'a mut W {
        self.variant(PINCOS_A::PSIZE)
    }
    #[doc = "The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)"]
    #[inline(always)]
    pub fn fixed4(self) -> &'a mut W {
        self.variant(PINCOS_A::FIXED4)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Memory data size"]
pub type MSIZE_A = PSIZE_A;
#[doc = "Field `MSIZE` reader - Memory data size"]
pub type MSIZE_R = PSIZE_R;
#[doc = "Field `MSIZE` writer - Memory data size"]
pub struct MSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Byte (8-bit)"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(MSIZE_A::BITS8)
    }
    #[doc = "Half-word (16-bit)"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(MSIZE_A::BITS16)
    }
    #[doc = "Word (32-bit)"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(MSIZE_A::BITS32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Peripheral data size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSIZE_A {
    #[doc = "0: Byte (8-bit)"]
    BITS8 = 0,
    #[doc = "1: Half-word (16-bit)"]
    BITS16 = 1,
    #[doc = "2: Word (32-bit)"]
    BITS32 = 2,
}
impl From<PSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSIZE` reader - Peripheral data size"]
pub struct PSIZE_R(crate::FieldReader<u8, PSIZE_A>);
impl PSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSIZE_A> {
        match self.bits {
            0 => Some(PSIZE_A::BITS8),
            1 => Some(PSIZE_A::BITS16),
            2 => Some(PSIZE_A::BITS32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BITS8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        **self == PSIZE_A::BITS8
    }
    #[doc = "Checks if the value of the field is `BITS16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        **self == PSIZE_A::BITS16
    }
    #[doc = "Checks if the value of the field is `BITS32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        **self == PSIZE_A::BITS32
    }
}
impl core::ops::Deref for PSIZE_R {
    type Target = crate::FieldReader<u8, PSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSIZE` writer - Peripheral data size"]
pub struct PSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Byte (8-bit)"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(PSIZE_A::BITS8)
    }
    #[doc = "Half-word (16-bit)"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(PSIZE_A::BITS16)
    }
    #[doc = "Word (32-bit)"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(PSIZE_A::BITS32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Memory increment mode"]
pub type MINC_A = PINC_A;
#[doc = "Field `MINC` reader - Memory increment mode"]
pub type MINC_R = PINC_R;
#[doc = "Field `MINC` writer - Memory increment mode"]
pub struct MINC_W<'a> {
    w: &'a mut W,
}
impl<'a> MINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MINC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Address pointer is fixed"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(MINC_A::FIXED)
    }
    #[doc = "Address pointer is incremented after each data transfer"]
    #[inline(always)]
    pub fn incremented(self) -> &'a mut W {
        self.variant(MINC_A::INCREMENTED)
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
#[doc = "Peripheral increment mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINC_A {
    #[doc = "0: Address pointer is fixed"]
    FIXED = 0,
    #[doc = "1: Address pointer is incremented after each data transfer"]
    INCREMENTED = 1,
}
impl From<PINC_A> for bool {
    #[inline(always)]
    fn from(variant: PINC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINC` reader - Peripheral increment mode"]
pub struct PINC_R(crate::FieldReader<bool, PINC_A>);
impl PINC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINC_A {
        match self.bits {
            false => PINC_A::FIXED,
            true => PINC_A::INCREMENTED,
        }
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        **self == PINC_A::FIXED
    }
    #[doc = "Checks if the value of the field is `INCREMENTED`"]
    #[inline(always)]
    pub fn is_incremented(&self) -> bool {
        **self == PINC_A::INCREMENTED
    }
}
impl core::ops::Deref for PINC_R {
    type Target = crate::FieldReader<bool, PINC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINC` writer - Peripheral increment mode"]
pub struct PINC_W<'a> {
    w: &'a mut W,
}
impl<'a> PINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Address pointer is fixed"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(PINC_A::FIXED)
    }
    #[doc = "Address pointer is incremented after each data transfer"]
    #[inline(always)]
    pub fn incremented(self) -> &'a mut W {
        self.variant(PINC_A::INCREMENTED)
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
#[doc = "Circular mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIRC_A {
    #[doc = "0: Circular mode disabled"]
    DISABLED = 0,
    #[doc = "1: Circular mode enabled"]
    ENABLED = 1,
}
impl From<CIRC_A> for bool {
    #[inline(always)]
    fn from(variant: CIRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIRC` reader - Circular mode"]
pub struct CIRC_R(crate::FieldReader<bool, CIRC_A>);
impl CIRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CIRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIRC_A {
        match self.bits {
            false => CIRC_A::DISABLED,
            true => CIRC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CIRC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CIRC_A::ENABLED
    }
}
impl core::ops::Deref for CIRC_R {
    type Target = crate::FieldReader<bool, CIRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIRC` writer - Circular mode"]
pub struct CIRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CIRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Circular mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CIRC_A::DISABLED)
    }
    #[doc = "Circular mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CIRC_A::ENABLED)
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
#[doc = "Data transfer direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIR_A {
    #[doc = "0: Peripheral-to-memory"]
    PERIPHERALTOMEMORY = 0,
    #[doc = "1: Memory-to-peripheral"]
    MEMORYTOPERIPHERAL = 1,
    #[doc = "2: Memory-to-memory"]
    MEMORYTOMEMORY = 2,
}
impl From<DIR_A> for u8 {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DIR` reader - Data transfer direction"]
pub struct DIR_R(crate::FieldReader<u8, DIR_A>);
impl DIR_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIR_A> {
        match self.bits {
            0 => Some(DIR_A::PERIPHERALTOMEMORY),
            1 => Some(DIR_A::MEMORYTOPERIPHERAL),
            2 => Some(DIR_A::MEMORYTOMEMORY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPHERALTOMEMORY`"]
    #[inline(always)]
    pub fn is_peripheral_to_memory(&self) -> bool {
        **self == DIR_A::PERIPHERALTOMEMORY
    }
    #[doc = "Checks if the value of the field is `MEMORYTOPERIPHERAL`"]
    #[inline(always)]
    pub fn is_memory_to_peripheral(&self) -> bool {
        **self == DIR_A::MEMORYTOPERIPHERAL
    }
    #[doc = "Checks if the value of the field is `MEMORYTOMEMORY`"]
    #[inline(always)]
    pub fn is_memory_to_memory(&self) -> bool {
        **self == DIR_A::MEMORYTOMEMORY
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<u8, DIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR` writer - Data transfer direction"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Peripheral-to-memory"]
    #[inline(always)]
    pub fn peripheral_to_memory(self) -> &'a mut W {
        self.variant(DIR_A::PERIPHERALTOMEMORY)
    }
    #[doc = "Memory-to-peripheral"]
    #[inline(always)]
    pub fn memory_to_peripheral(self) -> &'a mut W {
        self.variant(DIR_A::MEMORYTOPERIPHERAL)
    }
    #[doc = "Memory-to-memory"]
    #[inline(always)]
    pub fn memory_to_memory(self) -> &'a mut W {
        self.variant(DIR_A::MEMORYTOMEMORY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Peripheral flow controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFCTRL_A {
    #[doc = "0: The DMA is the flow controller"]
    DMA = 0,
    #[doc = "1: The peripheral is the flow controller"]
    PERIPHERAL = 1,
}
impl From<PFCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: PFCTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFCTRL` reader - Peripheral flow controller"]
pub struct PFCTRL_R(crate::FieldReader<bool, PFCTRL_A>);
impl PFCTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PFCTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFCTRL_A {
        match self.bits {
            false => PFCTRL_A::DMA,
            true => PFCTRL_A::PERIPHERAL,
        }
    }
    #[doc = "Checks if the value of the field is `DMA`"]
    #[inline(always)]
    pub fn is_dma(&self) -> bool {
        **self == PFCTRL_A::DMA
    }
    #[doc = "Checks if the value of the field is `PERIPHERAL`"]
    #[inline(always)]
    pub fn is_peripheral(&self) -> bool {
        **self == PFCTRL_A::PERIPHERAL
    }
}
impl core::ops::Deref for PFCTRL_R {
    type Target = crate::FieldReader<bool, PFCTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PFCTRL` writer - Peripheral flow controller"]
pub struct PFCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PFCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PFCTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA is the flow controller"]
    #[inline(always)]
    pub fn dma(self) -> &'a mut W {
        self.variant(PFCTRL_A::DMA)
    }
    #[doc = "The peripheral is the flow controller"]
    #[inline(always)]
    pub fn peripheral(self) -> &'a mut W {
        self.variant(PFCTRL_A::PERIPHERAL)
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
#[doc = "Transfer complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIE_A {
    #[doc = "0: TC interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: TC interrupt enabled"]
    ENABLED = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable"]
pub struct TCIE_R(crate::FieldReader<bool, TCIE_A>);
impl TCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::DISABLED,
            true => TCIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TCIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TCIE_A::ENABLED
    }
}
impl core::ops::Deref for TCIE_R {
    type Target = crate::FieldReader<bool, TCIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable"]
pub struct TCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::DISABLED)
    }
    #[doc = "TC interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Half transfer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIE_A {
    #[doc = "0: HT interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: HT interrupt enabled"]
    ENABLED = 1,
}
impl From<HTIE_A> for bool {
    #[inline(always)]
    fn from(variant: HTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIE` reader - Half transfer interrupt enable"]
pub struct HTIE_R(crate::FieldReader<bool, HTIE_A>);
impl HTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HTIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTIE_A {
        match self.bits {
            false => HTIE_A::DISABLED,
            true => HTIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == HTIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == HTIE_A::ENABLED
    }
}
impl core::ops::Deref for HTIE_R {
    type Target = crate::FieldReader<bool, HTIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTIE` writer - Half transfer interrupt enable"]
pub struct HTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HT interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HTIE_A::DISABLED)
    }
    #[doc = "HT interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HTIE_A::ENABLED)
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
#[doc = "Transfer error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIE_A {
    #[doc = "0: TE interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: TE interrupt enabled"]
    ENABLED = 1,
}
impl From<TEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIE` reader - Transfer error interrupt enable"]
pub struct TEIE_R(crate::FieldReader<bool, TEIE_A>);
impl TEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIE_A {
        match self.bits {
            false => TEIE_A::DISABLED,
            true => TEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TEIE_A::ENABLED
    }
}
impl core::ops::Deref for TEIE_R {
    type Target = crate::FieldReader<bool, TEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIE` writer - Transfer error interrupt enable"]
pub struct TEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEIE_A::DISABLED)
    }
    #[doc = "TE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEIE_A::ENABLED)
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
#[doc = "Direct mode error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMEIE_A {
    #[doc = "0: DME interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: DME interrupt enabled"]
    ENABLED = 1,
}
impl From<DMEIE_A> for bool {
    #[inline(always)]
    fn from(variant: DMEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMEIE` reader - Direct mode error interrupt enable"]
pub struct DMEIE_R(crate::FieldReader<bool, DMEIE_A>);
impl DMEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMEIE_A {
        match self.bits {
            false => DMEIE_A::DISABLED,
            true => DMEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMEIE_A::ENABLED
    }
}
impl core::ops::Deref for DMEIE_R {
    type Target = crate::FieldReader<bool, DMEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMEIE` writer - Direct mode error interrupt enable"]
pub struct DMEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DME interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMEIE_A::DISABLED)
    }
    #[doc = "DME interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMEIE_A::ENABLED)
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
#[doc = "Stream enable / flag stream ready when read low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: Stream disabled"]
    DISABLED = 0,
    #[doc = "1: Stream enabled"]
    ENABLED = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Stream enable / flag stream ready when read low"]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DISABLED,
            true => EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EN_A::ENABLED
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Stream enable / flag stream ready when read low"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stream disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::DISABLED)
    }
    #[doc = "Stream enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::ENABLED)
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
impl R {
    #[doc = "Bits 25:27 - Channel selection"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 23:24 - Memory burst transfer configuration"]
    #[inline(always)]
    pub fn mburst(&self) -> MBURST_R {
        MBURST_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bits 21:22 - Peripheral burst transfer configuration"]
    #[inline(always)]
    pub fn pburst(&self) -> PBURST_R {
        PBURST_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Current target (only in double buffer mode)"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Double buffer mode"]
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Priority level"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Peripheral increment offset size"]
    #[inline(always)]
    pub fn pincos(&self) -> PINCOS_R {
        PINCOS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - Memory data size"]
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - Peripheral data size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Memory increment mode"]
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Peripheral increment mode"]
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Circular mode"]
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Data transfer direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Peripheral flow controller"]
    #[inline(always)]
    pub fn pfctrl(&self) -> PFCTRL_R {
        PFCTRL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Half transfer interrupt enable"]
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Direct mode error interrupt enable"]
    #[inline(always)]
    pub fn dmeie(&self) -> DMEIE_R {
        DMEIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Stream enable / flag stream ready when read low"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 25:27 - Channel selection"]
    #[inline(always)]
    pub fn chsel(&mut self) -> CHSEL_W {
        CHSEL_W { w: self }
    }
    #[doc = "Bits 23:24 - Memory burst transfer configuration"]
    #[inline(always)]
    pub fn mburst(&mut self) -> MBURST_W {
        MBURST_W { w: self }
    }
    #[doc = "Bits 21:22 - Peripheral burst transfer configuration"]
    #[inline(always)]
    pub fn pburst(&mut self) -> PBURST_W {
        PBURST_W { w: self }
    }
    #[doc = "Bit 19 - Current target (only in double buffer mode)"]
    #[inline(always)]
    pub fn ct(&mut self) -> CT_W {
        CT_W { w: self }
    }
    #[doc = "Bit 18 - Double buffer mode"]
    #[inline(always)]
    pub fn dbm(&mut self) -> DBM_W {
        DBM_W { w: self }
    }
    #[doc = "Bits 16:17 - Priority level"]
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W {
        PL_W { w: self }
    }
    #[doc = "Bit 15 - Peripheral increment offset size"]
    #[inline(always)]
    pub fn pincos(&mut self) -> PINCOS_W {
        PINCOS_W { w: self }
    }
    #[doc = "Bits 13:14 - Memory data size"]
    #[inline(always)]
    pub fn msize(&mut self) -> MSIZE_W {
        MSIZE_W { w: self }
    }
    #[doc = "Bits 11:12 - Peripheral data size"]
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W {
        PSIZE_W { w: self }
    }
    #[doc = "Bit 10 - Memory increment mode"]
    #[inline(always)]
    pub fn minc(&mut self) -> MINC_W {
        MINC_W { w: self }
    }
    #[doc = "Bit 9 - Peripheral increment mode"]
    #[inline(always)]
    pub fn pinc(&mut self) -> PINC_W {
        PINC_W { w: self }
    }
    #[doc = "Bit 8 - Circular mode"]
    #[inline(always)]
    pub fn circ(&mut self) -> CIRC_W {
        CIRC_W { w: self }
    }
    #[doc = "Bits 6:7 - Data transfer direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 5 - Peripheral flow controller"]
    #[inline(always)]
    pub fn pfctrl(&mut self) -> PFCTRL_W {
        PFCTRL_W { w: self }
    }
    #[doc = "Bit 4 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W { w: self }
    }
    #[doc = "Bit 3 - Half transfer interrupt enable"]
    #[inline(always)]
    pub fn htie(&mut self) -> HTIE_W {
        HTIE_W { w: self }
    }
    #[doc = "Bit 2 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W {
        TEIE_W { w: self }
    }
    #[doc = "Bit 1 - Direct mode error interrupt enable"]
    #[inline(always)]
    pub fn dmeie(&mut self) -> DMEIE_W {
        DMEIE_W { w: self }
    }
    #[doc = "Bit 0 - Stream enable / flag stream ready when read low"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "stream x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
