#[doc = "Reader of register DMAOMR"]
pub type R = crate::R<u32, super::DMAOMR>;
#[doc = "Writer for register DMAOMR"]
pub type W = crate::W<u32, super::DMAOMR>;
#[doc = "Register DMAOMR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAOMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Start/stop receive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR_A {
    #[doc = "0: Reception is stopped after transfer of the current frame"]
    STOPPED = 0,
    #[doc = "1: Reception is placed in the Running state"]
    STARTED = 1,
}
impl From<SR_A> for bool {
    #[inline(always)]
    fn from(variant: SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SR`"]
pub type SR_R = crate::R<bool, SR_A>;
impl SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR_A {
        match self.bits {
            false => SR_A::STOPPED,
            true => SR_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `STOPPED`"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == SR_A::STOPPED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == SR_A::STARTED
    }
}
#[doc = "Write proxy for field `SR`"]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reception is stopped after transfer of the current frame"]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut W {
        self.variant(SR_A::STOPPED)
    }
    #[doc = "Reception is placed in the Running state"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(SR_A::STARTED)
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
#[doc = "Reader of field `OSF`"]
pub type OSF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSF`"]
pub struct OSF_W<'a> {
    w: &'a mut W,
}
impl<'a> OSF_W<'a> {
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
#[doc = "Receive threshold control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTC_A {
    #[doc = "0: 64 bytes"]
    RTC64 = 0,
    #[doc = "1: 32 bytes"]
    RTC32 = 1,
    #[doc = "2: 96 bytes"]
    RTC96 = 2,
    #[doc = "3: 128 bytes"]
    RTC128 = 3,
}
impl From<RTC_A> for u8 {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<u8, RTC_A>;
impl RTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            0 => RTC_A::RTC64,
            1 => RTC_A::RTC32,
            2 => RTC_A::RTC96,
            3 => RTC_A::RTC128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTC64`"]
    #[inline(always)]
    pub fn is_rtc64(&self) -> bool {
        *self == RTC_A::RTC64
    }
    #[doc = "Checks if the value of the field is `RTC32`"]
    #[inline(always)]
    pub fn is_rtc32(&self) -> bool {
        *self == RTC_A::RTC32
    }
    #[doc = "Checks if the value of the field is `RTC96`"]
    #[inline(always)]
    pub fn is_rtc96(&self) -> bool {
        *self == RTC_A::RTC96
    }
    #[doc = "Checks if the value of the field is `RTC128`"]
    #[inline(always)]
    pub fn is_rtc128(&self) -> bool {
        *self == RTC_A::RTC128
    }
}
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn rtc64(self) -> &'a mut W {
        self.variant(RTC_A::RTC64)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn rtc32(self) -> &'a mut W {
        self.variant(RTC_A::RTC32)
    }
    #[doc = "96 bytes"]
    #[inline(always)]
    pub fn rtc96(self) -> &'a mut W {
        self.variant(RTC_A::RTC96)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn rtc128(self) -> &'a mut W {
        self.variant(RTC_A::RTC128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Forward undersized good frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUGF_A {
    #[doc = "0: Rx FIFO drops all frames of less than 64 bytes"]
    DROP = 0,
    #[doc = "1: Rx FIFO forwards undersized frames"]
    FORWARD = 1,
}
impl From<FUGF_A> for bool {
    #[inline(always)]
    fn from(variant: FUGF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FUGF`"]
pub type FUGF_R = crate::R<bool, FUGF_A>;
impl FUGF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FUGF_A {
        match self.bits {
            false => FUGF_A::DROP,
            true => FUGF_A::FORWARD,
        }
    }
    #[doc = "Checks if the value of the field is `DROP`"]
    #[inline(always)]
    pub fn is_drop(&self) -> bool {
        *self == FUGF_A::DROP
    }
    #[doc = "Checks if the value of the field is `FORWARD`"]
    #[inline(always)]
    pub fn is_forward(&self) -> bool {
        *self == FUGF_A::FORWARD
    }
}
#[doc = "Write proxy for field `FUGF`"]
pub struct FUGF_W<'a> {
    w: &'a mut W,
}
impl<'a> FUGF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUGF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rx FIFO drops all frames of less than 64 bytes"]
    #[inline(always)]
    pub fn drop(self) -> &'a mut W {
        self.variant(FUGF_A::DROP)
    }
    #[doc = "Rx FIFO forwards undersized frames"]
    #[inline(always)]
    pub fn forward(self) -> &'a mut W {
        self.variant(FUGF_A::FORWARD)
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
#[doc = "Forward error frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEF_A {
    #[doc = "0: Rx FIFO drops frames with error status"]
    DROP = 0,
    #[doc = "1: All frames except runt error frames are forwarded to the DMA"]
    FORWARD = 1,
}
impl From<FEF_A> for bool {
    #[inline(always)]
    fn from(variant: FEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FEF`"]
pub type FEF_R = crate::R<bool, FEF_A>;
impl FEF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEF_A {
        match self.bits {
            false => FEF_A::DROP,
            true => FEF_A::FORWARD,
        }
    }
    #[doc = "Checks if the value of the field is `DROP`"]
    #[inline(always)]
    pub fn is_drop(&self) -> bool {
        *self == FEF_A::DROP
    }
    #[doc = "Checks if the value of the field is `FORWARD`"]
    #[inline(always)]
    pub fn is_forward(&self) -> bool {
        *self == FEF_A::FORWARD
    }
}
#[doc = "Write proxy for field `FEF`"]
pub struct FEF_W<'a> {
    w: &'a mut W,
}
impl<'a> FEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rx FIFO drops frames with error status"]
    #[inline(always)]
    pub fn drop(self) -> &'a mut W {
        self.variant(FEF_A::DROP)
    }
    #[doc = "All frames except runt error frames are forwarded to the DMA"]
    #[inline(always)]
    pub fn forward(self) -> &'a mut W {
        self.variant(FEF_A::FORWARD)
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
#[doc = "Start/stop transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ST_A {
    #[doc = "0: Transmission is placed in the Stopped state"]
    STOPPED = 0,
    #[doc = "1: Transmission is placed in Running state"]
    STARTED = 1,
}
impl From<ST_A> for bool {
    #[inline(always)]
    fn from(variant: ST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ST`"]
pub type ST_R = crate::R<bool, ST_A>;
impl ST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ST_A {
        match self.bits {
            false => ST_A::STOPPED,
            true => ST_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `STOPPED`"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == ST_A::STOPPED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == ST_A::STARTED
    }
}
#[doc = "Write proxy for field `ST`"]
pub struct ST_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmission is placed in the Stopped state"]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut W {
        self.variant(ST_A::STOPPED)
    }
    #[doc = "Transmission is placed in Running state"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(ST_A::STARTED)
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
#[doc = "Transmit threshold control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TTC_A {
    #[doc = "0: 64 bytes"]
    TTC64 = 0,
    #[doc = "1: 128 bytes"]
    TTC128 = 1,
    #[doc = "2: 192 bytes"]
    TTC192 = 2,
    #[doc = "3: 256 bytes"]
    TTC256 = 3,
    #[doc = "4: 40 bytes"]
    TTC40 = 4,
    #[doc = "5: 32 bytes"]
    TTC32 = 5,
    #[doc = "6: 24 bytes"]
    TTC24 = 6,
    #[doc = "7: 16 bytes"]
    TTC16 = 7,
}
impl From<TTC_A> for u8 {
    #[inline(always)]
    fn from(variant: TTC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TTC`"]
pub type TTC_R = crate::R<u8, TTC_A>;
impl TTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TTC_A {
        match self.bits {
            0 => TTC_A::TTC64,
            1 => TTC_A::TTC128,
            2 => TTC_A::TTC192,
            3 => TTC_A::TTC256,
            4 => TTC_A::TTC40,
            5 => TTC_A::TTC32,
            6 => TTC_A::TTC24,
            7 => TTC_A::TTC16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TTC64`"]
    #[inline(always)]
    pub fn is_ttc64(&self) -> bool {
        *self == TTC_A::TTC64
    }
    #[doc = "Checks if the value of the field is `TTC128`"]
    #[inline(always)]
    pub fn is_ttc128(&self) -> bool {
        *self == TTC_A::TTC128
    }
    #[doc = "Checks if the value of the field is `TTC192`"]
    #[inline(always)]
    pub fn is_ttc192(&self) -> bool {
        *self == TTC_A::TTC192
    }
    #[doc = "Checks if the value of the field is `TTC256`"]
    #[inline(always)]
    pub fn is_ttc256(&self) -> bool {
        *self == TTC_A::TTC256
    }
    #[doc = "Checks if the value of the field is `TTC40`"]
    #[inline(always)]
    pub fn is_ttc40(&self) -> bool {
        *self == TTC_A::TTC40
    }
    #[doc = "Checks if the value of the field is `TTC32`"]
    #[inline(always)]
    pub fn is_ttc32(&self) -> bool {
        *self == TTC_A::TTC32
    }
    #[doc = "Checks if the value of the field is `TTC24`"]
    #[inline(always)]
    pub fn is_ttc24(&self) -> bool {
        *self == TTC_A::TTC24
    }
    #[doc = "Checks if the value of the field is `TTC16`"]
    #[inline(always)]
    pub fn is_ttc16(&self) -> bool {
        *self == TTC_A::TTC16
    }
}
#[doc = "Write proxy for field `TTC`"]
pub struct TTC_W<'a> {
    w: &'a mut W,
}
impl<'a> TTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TTC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn ttc64(self) -> &'a mut W {
        self.variant(TTC_A::TTC64)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn ttc128(self) -> &'a mut W {
        self.variant(TTC_A::TTC128)
    }
    #[doc = "192 bytes"]
    #[inline(always)]
    pub fn ttc192(self) -> &'a mut W {
        self.variant(TTC_A::TTC192)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn ttc256(self) -> &'a mut W {
        self.variant(TTC_A::TTC256)
    }
    #[doc = "40 bytes"]
    #[inline(always)]
    pub fn ttc40(self) -> &'a mut W {
        self.variant(TTC_A::TTC40)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn ttc32(self) -> &'a mut W {
        self.variant(TTC_A::TTC32)
    }
    #[doc = "24 bytes"]
    #[inline(always)]
    pub fn ttc24(self) -> &'a mut W {
        self.variant(TTC_A::TTC24)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn ttc16(self) -> &'a mut W {
        self.variant(TTC_A::TTC16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
#[doc = "Flush transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTF_A {
    #[doc = "1: Transmit FIFO controller logic is reset to its default values. Cleared automatically"]
    FLUSH = 1,
}
impl From<FTF_A> for bool {
    #[inline(always)]
    fn from(variant: FTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTF`"]
pub type FTF_R = crate::R<bool, FTF_A>;
impl FTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FTF_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(FTF_A::FLUSH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == FTF_A::FLUSH
    }
}
#[doc = "Write proxy for field `FTF`"]
pub struct FTF_W<'a> {
    w: &'a mut W,
}
impl<'a> FTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmit FIFO controller logic is reset to its default values. Cleared automatically"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(FTF_A::FLUSH)
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
#[doc = "Transmit store and forward\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSF_A {
    #[doc = "0: Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold"]
    CUTTHROUGH = 0,
    #[doc = "1: Transmission starts when a full frame is in the Tx FIFO"]
    STOREFORWARD = 1,
}
impl From<TSF_A> for bool {
    #[inline(always)]
    fn from(variant: TSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSF`"]
pub type TSF_R = crate::R<bool, TSF_A>;
impl TSF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSF_A {
        match self.bits {
            false => TSF_A::CUTTHROUGH,
            true => TSF_A::STOREFORWARD,
        }
    }
    #[doc = "Checks if the value of the field is `CUTTHROUGH`"]
    #[inline(always)]
    pub fn is_cut_through(&self) -> bool {
        *self == TSF_A::CUTTHROUGH
    }
    #[doc = "Checks if the value of the field is `STOREFORWARD`"]
    #[inline(always)]
    pub fn is_store_forward(&self) -> bool {
        *self == TSF_A::STOREFORWARD
    }
}
#[doc = "Write proxy for field `TSF`"]
pub struct TSF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold"]
    #[inline(always)]
    pub fn cut_through(self) -> &'a mut W {
        self.variant(TSF_A::CUTTHROUGH)
    }
    #[doc = "Transmission starts when a full frame is in the Tx FIFO"]
    #[inline(always)]
    pub fn store_forward(self) -> &'a mut W {
        self.variant(TSF_A::STOREFORWARD)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `DFRF`"]
pub type DFRF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFRF`"]
pub struct DFRF_W<'a> {
    w: &'a mut W,
}
impl<'a> DFRF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Receive store and forward\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSF_A {
    #[doc = "0: Rx FIFO operates in cut-through mode, subject to RTC bits"]
    CUTTHROUGH = 0,
    #[doc = "1: Frames are read from Rx FIFO after complete frame has been written"]
    STOREFORWARD = 1,
}
impl From<RSF_A> for bool {
    #[inline(always)]
    fn from(variant: RSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSF`"]
pub type RSF_R = crate::R<bool, RSF_A>;
impl RSF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSF_A {
        match self.bits {
            false => RSF_A::CUTTHROUGH,
            true => RSF_A::STOREFORWARD,
        }
    }
    #[doc = "Checks if the value of the field is `CUTTHROUGH`"]
    #[inline(always)]
    pub fn is_cut_through(&self) -> bool {
        *self == RSF_A::CUTTHROUGH
    }
    #[doc = "Checks if the value of the field is `STOREFORWARD`"]
    #[inline(always)]
    pub fn is_store_forward(&self) -> bool {
        *self == RSF_A::STOREFORWARD
    }
}
#[doc = "Write proxy for field `RSF`"]
pub struct RSF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rx FIFO operates in cut-through mode, subject to RTC bits"]
    #[inline(always)]
    pub fn cut_through(self) -> &'a mut W {
        self.variant(RSF_A::CUTTHROUGH)
    }
    #[doc = "Frames are read from Rx FIFO after complete frame has been written"]
    #[inline(always)]
    pub fn store_forward(self) -> &'a mut W {
        self.variant(RSF_A::STOREFORWARD)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Dropping of TCP/IP checksum error frames disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTCEFD_A {
    #[doc = "0: Drop frames with errors only in the receive checksum offload engine"]
    ENABLED = 0,
    #[doc = "1: Do not drop frames that only have errors in the receive checksum offload engine"]
    DISABLED = 1,
}
impl From<DTCEFD_A> for bool {
    #[inline(always)]
    fn from(variant: DTCEFD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DTCEFD`"]
pub type DTCEFD_R = crate::R<bool, DTCEFD_A>;
impl DTCEFD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTCEFD_A {
        match self.bits {
            false => DTCEFD_A::ENABLED,
            true => DTCEFD_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTCEFD_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTCEFD_A::DISABLED
    }
}
#[doc = "Write proxy for field `DTCEFD`"]
pub struct DTCEFD_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCEFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTCEFD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Drop frames with errors only in the receive checksum offload engine"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DTCEFD_A::ENABLED)
    }
    #[doc = "Do not drop frames that only have errors in the receive checksum offload engine"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DTCEFD_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Start/stop receive"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Forward undersized good frames"]
    #[inline(always)]
    pub fn fugf(&self) -> FUGF_R {
        FUGF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Forward error frames"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Start/stop transmission"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Flush transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Transmit store and forward"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Disable flushing of received frames"]
    #[inline(always)]
    pub fn dfrf(&self) -> DFRF_R {
        DFRF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Receive store and forward"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Dropping of TCP/IP checksum error frames disable"]
    #[inline(always)]
    pub fn dtcefd(&self) -> DTCEFD_R {
        DTCEFD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Start/stop receive"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    pub fn osf(&mut self) -> OSF_W {
        OSF_W { w: self }
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 6 - Forward undersized good frames"]
    #[inline(always)]
    pub fn fugf(&mut self) -> FUGF_W {
        FUGF_W { w: self }
    }
    #[doc = "Bit 7 - Forward error frames"]
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W {
        FEF_W { w: self }
    }
    #[doc = "Bit 13 - Start/stop transmission"]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W {
        ST_W { w: self }
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    pub fn ttc(&mut self) -> TTC_W {
        TTC_W { w: self }
    }
    #[doc = "Bit 20 - Flush transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&mut self) -> FTF_W {
        FTF_W { w: self }
    }
    #[doc = "Bit 21 - Transmit store and forward"]
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W {
        TSF_W { w: self }
    }
    #[doc = "Bit 24 - Disable flushing of received frames"]
    #[inline(always)]
    pub fn dfrf(&mut self) -> DFRF_W {
        DFRF_W { w: self }
    }
    #[doc = "Bit 25 - Receive store and forward"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W {
        RSF_W { w: self }
    }
    #[doc = "Bit 26 - Dropping of TCP/IP checksum error frames disable"]
    #[inline(always)]
    pub fn dtcefd(&mut self) -> DTCEFD_W {
        DTCEFD_W { w: self }
    }
}
