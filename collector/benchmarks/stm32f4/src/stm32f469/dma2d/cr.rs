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
#[doc = "DMA2D mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Memory-to-memory (FG fetch only)"]
    MEMORYTOMEMORY = 0,
    #[doc = "1: Memory-to-memory with PFC (FG fetch only with FG PFC active)"]
    MEMORYTOMEMORYPFC = 1,
    #[doc = "2: Memory-to-memory with blending (FG and BG fetch with PFC and blending)"]
    MEMORYTOMEMORYPFCBLENDING = 2,
    #[doc = "3: Register-to-memory"]
    REGISTERTOMEMORY = 3,
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
            0 => MODE_A::MEMORYTOMEMORY,
            1 => MODE_A::MEMORYTOMEMORYPFC,
            2 => MODE_A::MEMORYTOMEMORYPFCBLENDING,
            3 => MODE_A::REGISTERTOMEMORY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MEMORYTOMEMORY`"]
    #[inline(always)]
    pub fn is_memory_to_memory(&self) -> bool {
        *self == MODE_A::MEMORYTOMEMORY
    }
    #[doc = "Checks if the value of the field is `MEMORYTOMEMORYPFC`"]
    #[inline(always)]
    pub fn is_memory_to_memory_pfc(&self) -> bool {
        *self == MODE_A::MEMORYTOMEMORYPFC
    }
    #[doc = "Checks if the value of the field is `MEMORYTOMEMORYPFCBLENDING`"]
    #[inline(always)]
    pub fn is_memory_to_memory_pfcblending(&self) -> bool {
        *self == MODE_A::MEMORYTOMEMORYPFCBLENDING
    }
    #[doc = "Checks if the value of the field is `REGISTERTOMEMORY`"]
    #[inline(always)]
    pub fn is_register_to_memory(&self) -> bool {
        *self == MODE_A::REGISTERTOMEMORY
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
    #[doc = "Memory-to-memory (FG fetch only)"]
    #[inline(always)]
    pub fn memory_to_memory(self) -> &'a mut W {
        self.variant(MODE_A::MEMORYTOMEMORY)
    }
    #[doc = "Memory-to-memory with PFC (FG fetch only with FG PFC active)"]
    #[inline(always)]
    pub fn memory_to_memory_pfc(self) -> &'a mut W {
        self.variant(MODE_A::MEMORYTOMEMORYPFC)
    }
    #[doc = "Memory-to-memory with blending (FG and BG fetch with PFC and blending)"]
    #[inline(always)]
    pub fn memory_to_memory_pfcblending(self) -> &'a mut W {
        self.variant(MODE_A::MEMORYTOMEMORYPFCBLENDING)
    }
    #[doc = "Register-to-memory"]
    #[inline(always)]
    pub fn register_to_memory(self) -> &'a mut W {
        self.variant(MODE_A::REGISTERTOMEMORY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Configuration Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEIE_A {
    #[doc = "0: CE interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: CE interrupt enabled"]
    ENABLED = 1,
}
impl From<CEIE_A> for bool {
    #[inline(always)]
    fn from(variant: CEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEIE`"]
pub type CEIE_R = crate::R<bool, CEIE_A>;
impl CEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEIE_A {
        match self.bits {
            false => CEIE_A::DISABLED,
            true => CEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CEIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `CEIE`"]
pub struct CEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CEIE_A::DISABLED)
    }
    #[doc = "CE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CEIE_A::ENABLED)
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
#[doc = "CLUT transfer complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIE_A {
    #[doc = "0: CTC interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: CTC interrupt enabled"]
    ENABLED = 1,
}
impl From<CTCIE_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCIE`"]
pub type CTCIE_R = crate::R<bool, CTCIE_A>;
impl CTCIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIE_A {
        match self.bits {
            false => CTCIE_A::DISABLED,
            true => CTCIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTCIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTCIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `CTCIE`"]
pub struct CTCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CTC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTCIE_A::DISABLED)
    }
    #[doc = "CTC interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTCIE_A::ENABLED)
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
#[doc = "CLUT access error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAEIE_A {
    #[doc = "0: CAE interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: CAE interrupt enabled"]
    ENABLED = 1,
}
impl From<CAEIE_A> for bool {
    #[inline(always)]
    fn from(variant: CAEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAEIE`"]
pub type CAEIE_R = crate::R<bool, CAEIE_A>;
impl CAEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAEIE_A {
        match self.bits {
            false => CAEIE_A::DISABLED,
            true => CAEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAEIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `CAEIE`"]
pub struct CAEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CAE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAEIE_A::DISABLED)
    }
    #[doc = "CAE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAEIE_A::ENABLED)
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
#[doc = "Transfer watermark interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWIE_A {
    #[doc = "0: TW interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: TW interrupt enabled"]
    ENABLED = 1,
}
impl From<TWIE_A> for bool {
    #[inline(always)]
    fn from(variant: TWIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TWIE`"]
pub type TWIE_R = crate::R<bool, TWIE_A>;
impl TWIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWIE_A {
        match self.bits {
            false => TWIE_A::DISABLED,
            true => TWIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TWIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TWIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `TWIE`"]
pub struct TWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TWIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TW interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TWIE_A::DISABLED)
    }
    #[doc = "TW interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TWIE_A::ENABLED)
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
#[doc = "Reader of field `TCIE`"]
pub type TCIE_R = crate::R<bool, TCIE_A>;
impl TCIE_R {
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
        *self == TCIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `TCIE`"]
pub struct TCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
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
#[doc = "Reader of field `TEIE`"]
pub type TEIE_R = crate::R<bool, TEIE_A>;
impl TEIE_R {
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
        *self == TEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `TEIE`"]
pub struct TEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Abort\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABORT_A {
    #[doc = "1: Transfer abort requested"]
    ABORTREQUEST = 1,
}
impl From<ABORT_A> for bool {
    #[inline(always)]
    fn from(variant: ABORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ABORT`"]
pub type ABORT_R = crate::R<bool, ABORT_A>;
impl ABORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ABORT_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(ABORT_A::ABORTREQUEST),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ABORTREQUEST`"]
    #[inline(always)]
    pub fn is_abort_request(&self) -> bool {
        *self == ABORT_A::ABORTREQUEST
    }
}
#[doc = "Write proxy for field `ABORT`"]
pub struct ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABORT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transfer abort requested"]
    #[inline(always)]
    pub fn abort_request(self) -> &'a mut W {
        self.variant(ABORT_A::ABORTREQUEST)
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
#[doc = "Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSP_A {
    #[doc = "0: Transfer not suspended"]
    NOTSUSPENDED = 0,
    #[doc = "1: Transfer suspended"]
    SUSPENDED = 1,
}
impl From<SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: SUSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SUSP`"]
pub type SUSP_R = crate::R<bool, SUSP_A>;
impl SUSP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSP_A {
        match self.bits {
            false => SUSP_A::NOTSUSPENDED,
            true => SUSP_A::SUSPENDED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSUSPENDED`"]
    #[inline(always)]
    pub fn is_not_suspended(&self) -> bool {
        *self == SUSP_A::NOTSUSPENDED
    }
    #[doc = "Checks if the value of the field is `SUSPENDED`"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == SUSP_A::SUSPENDED
    }
}
#[doc = "Write proxy for field `SUSP`"]
pub struct SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUSP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transfer not suspended"]
    #[inline(always)]
    pub fn not_suspended(self) -> &'a mut W {
        self.variant(SUSP_A::NOTSUSPENDED)
    }
    #[doc = "Transfer suspended"]
    #[inline(always)]
    pub fn suspended(self) -> &'a mut W {
        self.variant(SUSP_A::SUSPENDED)
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
#[doc = "Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_A {
    #[doc = "1: Launch the DMA2D"]
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
    pub fn variant(&self) -> crate::Variant<bool, START_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(START_A::START),
            i => Res(i),
        }
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
    #[doc = "Launch the DMA2D"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17 - DMA2D mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 13 - Configuration Error Interrupt Enable"]
    #[inline(always)]
    pub fn ceie(&self) -> CEIE_R {
        CEIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CLUT transfer complete interrupt enable"]
    #[inline(always)]
    pub fn ctcie(&self) -> CTCIE_R {
        CTCIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CLUT access error interrupt enable"]
    #[inline(always)]
    pub fn caeie(&self) -> CAEIE_R {
        CAEIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transfer watermark interrupt enable"]
    #[inline(always)]
    pub fn twie(&self) -> TWIE_R {
        TWIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Abort"]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Suspend"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Start"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - DMA2D mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 13 - Configuration Error Interrupt Enable"]
    #[inline(always)]
    pub fn ceie(&mut self) -> CEIE_W {
        CEIE_W { w: self }
    }
    #[doc = "Bit 12 - CLUT transfer complete interrupt enable"]
    #[inline(always)]
    pub fn ctcie(&mut self) -> CTCIE_W {
        CTCIE_W { w: self }
    }
    #[doc = "Bit 11 - CLUT access error interrupt enable"]
    #[inline(always)]
    pub fn caeie(&mut self) -> CAEIE_W {
        CAEIE_W { w: self }
    }
    #[doc = "Bit 10 - Transfer watermark interrupt enable"]
    #[inline(always)]
    pub fn twie(&mut self) -> TWIE_W {
        TWIE_W { w: self }
    }
    #[doc = "Bit 9 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W { w: self }
    }
    #[doc = "Bit 8 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W {
        TEIE_W { w: self }
    }
    #[doc = "Bit 2 - Abort"]
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W {
        ABORT_W { w: self }
    }
    #[doc = "Bit 1 - Suspend"]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W { w: self }
    }
    #[doc = "Bit 0 - Start"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
}
