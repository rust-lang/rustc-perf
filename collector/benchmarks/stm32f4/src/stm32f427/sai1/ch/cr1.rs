#[doc = "Reader of register CR1"]
pub type R = crate::R<u32, super::CR1>;
#[doc = "Writer for register CR1"]
pub type W = crate::W<u32, super::CR1>;
#[doc = "Register CR1 `reset()`'s with value 0x40"]
impl crate::ResetValue for super::CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x40
    }
}
#[doc = "Reader of field `MCJDIV`"]
pub type MCJDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCJDIV`"]
pub struct MCJDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCJDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "No divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NODIV_A {
    #[doc = "0: Master clock generator is enabled"]
    MASTERCLOCK = 0,
    #[doc = "1: No divider used in the clock generator (in this case Master Clock Divider bit has no effect)"]
    NODIV = 1,
}
impl From<NODIV_A> for bool {
    #[inline(always)]
    fn from(variant: NODIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NODIV`"]
pub type NODIV_R = crate::R<bool, NODIV_A>;
impl NODIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NODIV_A {
        match self.bits {
            false => NODIV_A::MASTERCLOCK,
            true => NODIV_A::NODIV,
        }
    }
    #[doc = "Checks if the value of the field is `MASTERCLOCK`"]
    #[inline(always)]
    pub fn is_master_clock(&self) -> bool {
        *self == NODIV_A::MASTERCLOCK
    }
    #[doc = "Checks if the value of the field is `NODIV`"]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == NODIV_A::NODIV
    }
}
#[doc = "Write proxy for field `NODIV`"]
pub struct NODIV_W<'a> {
    w: &'a mut W,
}
impl<'a> NODIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NODIV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master clock generator is enabled"]
    #[inline(always)]
    pub fn master_clock(self) -> &'a mut W {
        self.variant(NODIV_A::MASTERCLOCK)
    }
    #[doc = "No divider used in the clock generator (in this case Master Clock Divider bit has no effect)"]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut W {
        self.variant(NODIV_A::NODIV)
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
#[doc = "DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: DMA disabled"]
    DISABLED = 0,
    #[doc = "1: DMA enabled"]
    ENABLED = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, DMAEN_A>;
impl DMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DISABLED,
            true => DMAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::DISABLED)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::ENABLED)
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
#[doc = "Audio block A enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAIEN_A {
    #[doc = "0: SAI audio block disabled"]
    DISABLED = 0,
    #[doc = "1: SAI audio block enabled"]
    ENABLED = 1,
}
impl From<SAIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SAIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAIEN`"]
pub type SAIEN_R = crate::R<bool, SAIEN_A>;
impl SAIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIEN_A {
        match self.bits {
            false => SAIEN_A::DISABLED,
            true => SAIEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAIEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAIEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `SAIEN`"]
pub struct SAIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SAI audio block disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAIEN_A::DISABLED)
    }
    #[doc = "SAI audio block enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAIEN_A::ENABLED)
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
#[doc = "Output drive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDRIV_A {
    #[doc = "0: Audio block output driven when SAIEN is set"]
    ONSTART = 0,
    #[doc = "1: Audio block output driven immediately after the setting of this bit"]
    IMMEDIATELY = 1,
}
impl From<OUTDRIV_A> for bool {
    #[inline(always)]
    fn from(variant: OUTDRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OUTDRIV`"]
pub type OUTDRIV_R = crate::R<bool, OUTDRIV_A>;
impl OUTDRIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTDRIV_A {
        match self.bits {
            false => OUTDRIV_A::ONSTART,
            true => OUTDRIV_A::IMMEDIATELY,
        }
    }
    #[doc = "Checks if the value of the field is `ONSTART`"]
    #[inline(always)]
    pub fn is_on_start(&self) -> bool {
        *self == OUTDRIV_A::ONSTART
    }
    #[doc = "Checks if the value of the field is `IMMEDIATELY`"]
    #[inline(always)]
    pub fn is_immediately(&self) -> bool {
        *self == OUTDRIV_A::IMMEDIATELY
    }
}
#[doc = "Write proxy for field `OUTDRIV`"]
pub struct OUTDRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDRIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTDRIV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Audio block output driven when SAIEN is set"]
    #[inline(always)]
    pub fn on_start(self) -> &'a mut W {
        self.variant(OUTDRIV_A::ONSTART)
    }
    #[doc = "Audio block output driven immediately after the setting of this bit"]
    #[inline(always)]
    pub fn immediately(self) -> &'a mut W {
        self.variant(OUTDRIV_A::IMMEDIATELY)
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
#[doc = "Mono mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONO_A {
    #[doc = "0: Stereo mode"]
    STEREO = 0,
    #[doc = "1: Mono mode"]
    MONO = 1,
}
impl From<MONO_A> for bool {
    #[inline(always)]
    fn from(variant: MONO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MONO`"]
pub type MONO_R = crate::R<bool, MONO_A>;
impl MONO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONO_A {
        match self.bits {
            false => MONO_A::STEREO,
            true => MONO_A::MONO,
        }
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == MONO_A::STEREO
    }
    #[doc = "Checks if the value of the field is `MONO`"]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        *self == MONO_A::MONO
    }
}
#[doc = "Write proxy for field `MONO`"]
pub struct MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> MONO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stereo mode"]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut W {
        self.variant(MONO_A::STEREO)
    }
    #[doc = "Mono mode"]
    #[inline(always)]
    pub fn mono(self) -> &'a mut W {
        self.variant(MONO_A::MONO)
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
#[doc = "Synchronization enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCEN_A {
    #[doc = "0: audio sub-block in asynchronous mode"]
    ASYNCHRONOUS = 0,
    #[doc = "1: audio sub-block is synchronous with the other internal audio sub-block. In this case, the audio sub-block must be configured in slave mode"]
    INTERNAL = 1,
    #[doc = "2: audio sub-block is synchronous with an external SAI embedded peripheral. In this case the audio sub-block should be configured in Slave mode"]
    EXTERNAL = 2,
}
impl From<SYNCEN_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNCEN`"]
pub type SYNCEN_R = crate::R<u8, SYNCEN_A>;
impl SYNCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYNCEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYNCEN_A::ASYNCHRONOUS),
            1 => Val(SYNCEN_A::INTERNAL),
            2 => Val(SYNCEN_A::EXTERNAL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == SYNCEN_A::ASYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == SYNCEN_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == SYNCEN_A::EXTERNAL
    }
}
#[doc = "Write proxy for field `SYNCEN`"]
pub struct SYNCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "audio sub-block in asynchronous mode"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(SYNCEN_A::ASYNCHRONOUS)
    }
    #[doc = "audio sub-block is synchronous with the other internal audio sub-block. In this case, the audio sub-block must be configured in slave mode"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(SYNCEN_A::INTERNAL)
    }
    #[doc = "audio sub-block is synchronous with an external SAI embedded peripheral. In this case the audio sub-block should be configured in Slave mode"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(SYNCEN_A::EXTERNAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Clock strobing edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKSTR_A {
    #[doc = "0: Data strobing edge is falling edge of SCK"]
    FALLINGEDGE = 0,
    #[doc = "1: Data strobing edge is rising edge of SCK"]
    RISINGEDGE = 1,
}
impl From<CKSTR_A> for bool {
    #[inline(always)]
    fn from(variant: CKSTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CKSTR`"]
pub type CKSTR_R = crate::R<bool, CKSTR_A>;
impl CKSTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKSTR_A {
        match self.bits {
            false => CKSTR_A::FALLINGEDGE,
            true => CKSTR_A::RISINGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CKSTR_A::FALLINGEDGE
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CKSTR_A::RISINGEDGE
    }
}
#[doc = "Write proxy for field `CKSTR`"]
pub struct CKSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKSTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data strobing edge is falling edge of SCK"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CKSTR_A::FALLINGEDGE)
    }
    #[doc = "Data strobing edge is rising edge of SCK"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CKSTR_A::RISINGEDGE)
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
#[doc = "Least significant bit first\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBFIRST_A {
    #[doc = "0: Data are transferred with MSB first"]
    MSBFIRST = 0,
    #[doc = "1: Data are transferred with LSB first"]
    LSBFIRST = 1,
}
impl From<LSBFIRST_A> for bool {
    #[inline(always)]
    fn from(variant: LSBFIRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSBFIRST`"]
pub type LSBFIRST_R = crate::R<bool, LSBFIRST_A>;
impl LSBFIRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSBFIRST_A {
        match self.bits {
            false => LSBFIRST_A::MSBFIRST,
            true => LSBFIRST_A::LSBFIRST,
        }
    }
    #[doc = "Checks if the value of the field is `MSBFIRST`"]
    #[inline(always)]
    pub fn is_msb_first(&self) -> bool {
        *self == LSBFIRST_A::MSBFIRST
    }
    #[doc = "Checks if the value of the field is `LSBFIRST`"]
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        *self == LSBFIRST_A::LSBFIRST
    }
}
#[doc = "Write proxy for field `LSBFIRST`"]
pub struct LSBFIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> LSBFIRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSBFIRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data are transferred with MSB first"]
    #[inline(always)]
    pub fn msb_first(self) -> &'a mut W {
        self.variant(LSBFIRST_A::MSBFIRST)
    }
    #[doc = "Data are transferred with LSB first"]
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut W {
        self.variant(LSBFIRST_A::LSBFIRST)
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
#[doc = "Data size\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DS_A {
    #[doc = "2: 8 bits"]
    BIT8 = 2,
    #[doc = "3: 10 bits"]
    BIT10 = 3,
    #[doc = "4: 16 bits"]
    BIT16 = 4,
    #[doc = "5: 20 bits"]
    BIT20 = 5,
    #[doc = "6: 24 bits"]
    BIT24 = 6,
    #[doc = "7: 32 bits"]
    BIT32 = 7,
}
impl From<DS_A> for u8 {
    #[inline(always)]
    fn from(variant: DS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DS`"]
pub type DS_R = crate::R<u8, DS_A>;
impl DS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DS_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(DS_A::BIT8),
            3 => Val(DS_A::BIT10),
            4 => Val(DS_A::BIT16),
            5 => Val(DS_A::BIT20),
            6 => Val(DS_A::BIT24),
            7 => Val(DS_A::BIT32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIT8`"]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == DS_A::BIT8
    }
    #[doc = "Checks if the value of the field is `BIT10`"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == DS_A::BIT10
    }
    #[doc = "Checks if the value of the field is `BIT16`"]
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == DS_A::BIT16
    }
    #[doc = "Checks if the value of the field is `BIT20`"]
    #[inline(always)]
    pub fn is_bit20(&self) -> bool {
        *self == DS_A::BIT20
    }
    #[doc = "Checks if the value of the field is `BIT24`"]
    #[inline(always)]
    pub fn is_bit24(&self) -> bool {
        *self == DS_A::BIT24
    }
    #[doc = "Checks if the value of the field is `BIT32`"]
    #[inline(always)]
    pub fn is_bit32(&self) -> bool {
        *self == DS_A::BIT32
    }
}
#[doc = "Write proxy for field `DS`"]
pub struct DS_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut W {
        self.variant(DS_A::BIT8)
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut W {
        self.variant(DS_A::BIT10)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn bit16(self) -> &'a mut W {
        self.variant(DS_A::BIT16)
    }
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn bit20(self) -> &'a mut W {
        self.variant(DS_A::BIT20)
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn bit24(self) -> &'a mut W {
        self.variant(DS_A::BIT24)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn bit32(self) -> &'a mut W {
        self.variant(DS_A::BIT32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Protocol configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRTCFG_A {
    #[doc = "0: Free protocol. Free protocol allows to use the powerful configuration of the audio block to address a specific audio protocol"]
    FREE = 0,
    #[doc = "1: SPDIF protocol"]
    SPDIF = 1,
    #[doc = "2: AC’97 protocol"]
    AC97 = 2,
}
impl From<PRTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: PRTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRTCFG`"]
pub type PRTCFG_R = crate::R<u8, PRTCFG_A>;
impl PRTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRTCFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRTCFG_A::FREE),
            1 => Val(PRTCFG_A::SPDIF),
            2 => Val(PRTCFG_A::AC97),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FREE`"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == PRTCFG_A::FREE
    }
    #[doc = "Checks if the value of the field is `SPDIF`"]
    #[inline(always)]
    pub fn is_spdif(&self) -> bool {
        *self == PRTCFG_A::SPDIF
    }
    #[doc = "Checks if the value of the field is `AC97`"]
    #[inline(always)]
    pub fn is_ac97(&self) -> bool {
        *self == PRTCFG_A::AC97
    }
}
#[doc = "Write proxy for field `PRTCFG`"]
pub struct PRTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRTCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Free protocol. Free protocol allows to use the powerful configuration of the audio block to address a specific audio protocol"]
    #[inline(always)]
    pub fn free(self) -> &'a mut W {
        self.variant(PRTCFG_A::FREE)
    }
    #[doc = "SPDIF protocol"]
    #[inline(always)]
    pub fn spdif(self) -> &'a mut W {
        self.variant(PRTCFG_A::SPDIF)
    }
    #[doc = "AC’97 protocol"]
    #[inline(always)]
    pub fn ac97(self) -> &'a mut W {
        self.variant(PRTCFG_A::AC97)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Audio block mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Master transmitter"]
    MASTERTX = 0,
    #[doc = "1: Master receiver"]
    MASTERRX = 1,
    #[doc = "2: Slave transmitter"]
    SLAVETX = 2,
    #[doc = "3: Slave receiver"]
    SLAVERX = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::MASTERTX,
            1 => MODE_A::MASTERRX,
            2 => MODE_A::SLAVETX,
            3 => MODE_A::SLAVERX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MASTERTX`"]
    #[inline(always)]
    pub fn is_master_tx(&self) -> bool {
        *self == MODE_A::MASTERTX
    }
    #[doc = "Checks if the value of the field is `MASTERRX`"]
    #[inline(always)]
    pub fn is_master_rx(&self) -> bool {
        *self == MODE_A::MASTERRX
    }
    #[doc = "Checks if the value of the field is `SLAVETX`"]
    #[inline(always)]
    pub fn is_slave_tx(&self) -> bool {
        *self == MODE_A::SLAVETX
    }
    #[doc = "Checks if the value of the field is `SLAVERX`"]
    #[inline(always)]
    pub fn is_slave_rx(&self) -> bool {
        *self == MODE_A::SLAVERX
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Master transmitter"]
    #[inline(always)]
    pub fn master_tx(self) -> &'a mut W {
        self.variant(MODE_A::MASTERTX)
    }
    #[doc = "Master receiver"]
    #[inline(always)]
    pub fn master_rx(self) -> &'a mut W {
        self.variant(MODE_A::MASTERRX)
    }
    #[doc = "Slave transmitter"]
    #[inline(always)]
    pub fn slave_tx(self) -> &'a mut W {
        self.variant(MODE_A::SLAVETX)
    }
    #[doc = "Slave receiver"]
    #[inline(always)]
    pub fn slave_rx(self) -> &'a mut W {
        self.variant(MODE_A::SLAVERX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:23 - Master clock divider"]
    #[inline(always)]
    pub fn mcjdiv(&self) -> MCJDIV_R {
        MCJDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 19 - No divider"]
    #[inline(always)]
    pub fn nodiv(&self) -> NODIV_R {
        NODIV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Audio block A enable"]
    #[inline(always)]
    pub fn saien(&self) -> SAIEN_R {
        SAIEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Output drive"]
    #[inline(always)]
    pub fn outdriv(&self) -> OUTDRIV_R {
        OUTDRIV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Mono mode"]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Synchronization enable"]
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - Clock strobing edge"]
    #[inline(always)]
    pub fn ckstr(&self) -> CKSTR_R {
        CKSTR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Least significant bit first"]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Data size"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 2:3 - Protocol configuration"]
    #[inline(always)]
    pub fn prtcfg(&self) -> PRTCFG_R {
        PRTCFG_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Audio block mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 20:23 - Master clock divider"]
    #[inline(always)]
    pub fn mcjdiv(&mut self) -> MCJDIV_W {
        MCJDIV_W { w: self }
    }
    #[doc = "Bit 19 - No divider"]
    #[inline(always)]
    pub fn nodiv(&mut self) -> NODIV_W {
        NODIV_W { w: self }
    }
    #[doc = "Bit 17 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 16 - Audio block A enable"]
    #[inline(always)]
    pub fn saien(&mut self) -> SAIEN_W {
        SAIEN_W { w: self }
    }
    #[doc = "Bit 13 - Output drive"]
    #[inline(always)]
    pub fn outdriv(&mut self) -> OUTDRIV_W {
        OUTDRIV_W { w: self }
    }
    #[doc = "Bit 12 - Mono mode"]
    #[inline(always)]
    pub fn mono(&mut self) -> MONO_W {
        MONO_W { w: self }
    }
    #[doc = "Bits 10:11 - Synchronization enable"]
    #[inline(always)]
    pub fn syncen(&mut self) -> SYNCEN_W {
        SYNCEN_W { w: self }
    }
    #[doc = "Bit 9 - Clock strobing edge"]
    #[inline(always)]
    pub fn ckstr(&mut self) -> CKSTR_W {
        CKSTR_W { w: self }
    }
    #[doc = "Bit 8 - Least significant bit first"]
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W {
        LSBFIRST_W { w: self }
    }
    #[doc = "Bits 5:7 - Data size"]
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W {
        DS_W { w: self }
    }
    #[doc = "Bits 2:3 - Protocol configuration"]
    #[inline(always)]
    pub fn prtcfg(&mut self) -> PRTCFG_W {
        PRTCFG_W { w: self }
    }
    #[doc = "Bits 0:1 - Audio block mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
