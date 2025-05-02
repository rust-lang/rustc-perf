#[doc = "Register `DMAOMR` reader"]
pub struct R(crate::R<DMAOMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAOMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAOMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAOMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAOMR` writer"]
pub struct W(crate::W<DMAOMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAOMR_SPEC>;
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
impl From<crate::W<DMAOMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAOMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Start/stop receive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR_A {
    #[doc = "0: Reception is stopped after transfer of the current frame"]
    Stopped = 0,
    #[doc = "1: Reception is placed in the Running state"]
    Started = 1,
}
impl From<SR_A> for bool {
    #[inline(always)]
    fn from(variant: SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR` reader - Start/stop receive"]
pub type SR_R = crate::BitReader<SR_A>;
impl SR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR_A {
        match self.bits {
            false => SR_A::Stopped,
            true => SR_A::Started,
        }
    }
    #[doc = "Checks if the value of the field is `Stopped`"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == SR_A::Stopped
    }
    #[doc = "Checks if the value of the field is `Started`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == SR_A::Started
    }
}
#[doc = "Field `SR` writer - Start/stop receive"]
pub type SR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, SR_A, O>;
impl<'a, const O: u8> SR_W<'a, O> {
    #[doc = "Reception is stopped after transfer of the current frame"]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut W {
        self.variant(SR_A::Stopped)
    }
    #[doc = "Reception is placed in the Running state"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(SR_A::Started)
    }
}
#[doc = "Field `OSF` reader - Operate on second frame"]
pub type OSF_R = crate::BitReader<bool>;
#[doc = "Field `OSF` writer - Operate on second frame"]
pub type OSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, bool, O>;
#[doc = "Receive threshold control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTC_A {
    #[doc = "0: 64 bytes"]
    Rtc64 = 0,
    #[doc = "1: 32 bytes"]
    Rtc32 = 1,
    #[doc = "2: 96 bytes"]
    Rtc96 = 2,
    #[doc = "3: 128 bytes"]
    Rtc128 = 3,
}
impl From<RTC_A> for u8 {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTC` reader - Receive threshold control"]
pub type RTC_R = crate::FieldReader<u8, RTC_A>;
impl RTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            0 => RTC_A::Rtc64,
            1 => RTC_A::Rtc32,
            2 => RTC_A::Rtc96,
            3 => RTC_A::Rtc128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Rtc64`"]
    #[inline(always)]
    pub fn is_rtc64(&self) -> bool {
        *self == RTC_A::Rtc64
    }
    #[doc = "Checks if the value of the field is `Rtc32`"]
    #[inline(always)]
    pub fn is_rtc32(&self) -> bool {
        *self == RTC_A::Rtc32
    }
    #[doc = "Checks if the value of the field is `Rtc96`"]
    #[inline(always)]
    pub fn is_rtc96(&self) -> bool {
        *self == RTC_A::Rtc96
    }
    #[doc = "Checks if the value of the field is `Rtc128`"]
    #[inline(always)]
    pub fn is_rtc128(&self) -> bool {
        *self == RTC_A::Rtc128
    }
}
#[doc = "Field `RTC` writer - Receive threshold control"]
pub type RTC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DMAOMR_SPEC, u8, RTC_A, 2, O>;
impl<'a, const O: u8> RTC_W<'a, O> {
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn rtc64(self) -> &'a mut W {
        self.variant(RTC_A::Rtc64)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn rtc32(self) -> &'a mut W {
        self.variant(RTC_A::Rtc32)
    }
    #[doc = "96 bytes"]
    #[inline(always)]
    pub fn rtc96(self) -> &'a mut W {
        self.variant(RTC_A::Rtc96)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn rtc128(self) -> &'a mut W {
        self.variant(RTC_A::Rtc128)
    }
}
#[doc = "Forward undersized good frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUGF_A {
    #[doc = "0: Rx FIFO drops all frames of less than 64 bytes"]
    Drop = 0,
    #[doc = "1: Rx FIFO forwards undersized frames"]
    Forward = 1,
}
impl From<FUGF_A> for bool {
    #[inline(always)]
    fn from(variant: FUGF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FUGF` reader - Forward undersized good frames"]
pub type FUGF_R = crate::BitReader<FUGF_A>;
impl FUGF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FUGF_A {
        match self.bits {
            false => FUGF_A::Drop,
            true => FUGF_A::Forward,
        }
    }
    #[doc = "Checks if the value of the field is `Drop`"]
    #[inline(always)]
    pub fn is_drop(&self) -> bool {
        *self == FUGF_A::Drop
    }
    #[doc = "Checks if the value of the field is `Forward`"]
    #[inline(always)]
    pub fn is_forward(&self) -> bool {
        *self == FUGF_A::Forward
    }
}
#[doc = "Field `FUGF` writer - Forward undersized good frames"]
pub type FUGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, FUGF_A, O>;
impl<'a, const O: u8> FUGF_W<'a, O> {
    #[doc = "Rx FIFO drops all frames of less than 64 bytes"]
    #[inline(always)]
    pub fn drop(self) -> &'a mut W {
        self.variant(FUGF_A::Drop)
    }
    #[doc = "Rx FIFO forwards undersized frames"]
    #[inline(always)]
    pub fn forward(self) -> &'a mut W {
        self.variant(FUGF_A::Forward)
    }
}
#[doc = "Forward error frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEF_A {
    #[doc = "0: Rx FIFO drops frames with error status"]
    Drop = 0,
    #[doc = "1: All frames except runt error frames are forwarded to the DMA"]
    Forward = 1,
}
impl From<FEF_A> for bool {
    #[inline(always)]
    fn from(variant: FEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEF` reader - Forward error frames"]
pub type FEF_R = crate::BitReader<FEF_A>;
impl FEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEF_A {
        match self.bits {
            false => FEF_A::Drop,
            true => FEF_A::Forward,
        }
    }
    #[doc = "Checks if the value of the field is `Drop`"]
    #[inline(always)]
    pub fn is_drop(&self) -> bool {
        *self == FEF_A::Drop
    }
    #[doc = "Checks if the value of the field is `Forward`"]
    #[inline(always)]
    pub fn is_forward(&self) -> bool {
        *self == FEF_A::Forward
    }
}
#[doc = "Field `FEF` writer - Forward error frames"]
pub type FEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, FEF_A, O>;
impl<'a, const O: u8> FEF_W<'a, O> {
    #[doc = "Rx FIFO drops frames with error status"]
    #[inline(always)]
    pub fn drop(self) -> &'a mut W {
        self.variant(FEF_A::Drop)
    }
    #[doc = "All frames except runt error frames are forwarded to the DMA"]
    #[inline(always)]
    pub fn forward(self) -> &'a mut W {
        self.variant(FEF_A::Forward)
    }
}
#[doc = "Start/stop transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ST_A {
    #[doc = "0: Transmission is placed in the Stopped state"]
    Stopped = 0,
    #[doc = "1: Transmission is placed in Running state"]
    Started = 1,
}
impl From<ST_A> for bool {
    #[inline(always)]
    fn from(variant: ST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ST` reader - Start/stop transmission"]
pub type ST_R = crate::BitReader<ST_A>;
impl ST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ST_A {
        match self.bits {
            false => ST_A::Stopped,
            true => ST_A::Started,
        }
    }
    #[doc = "Checks if the value of the field is `Stopped`"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == ST_A::Stopped
    }
    #[doc = "Checks if the value of the field is `Started`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == ST_A::Started
    }
}
#[doc = "Field `ST` writer - Start/stop transmission"]
pub type ST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, ST_A, O>;
impl<'a, const O: u8> ST_W<'a, O> {
    #[doc = "Transmission is placed in the Stopped state"]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut W {
        self.variant(ST_A::Stopped)
    }
    #[doc = "Transmission is placed in Running state"]
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(ST_A::Started)
    }
}
#[doc = "Transmit threshold control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TTC_A {
    #[doc = "0: 64 bytes"]
    Ttc64 = 0,
    #[doc = "1: 128 bytes"]
    Ttc128 = 1,
    #[doc = "2: 192 bytes"]
    Ttc192 = 2,
    #[doc = "3: 256 bytes"]
    Ttc256 = 3,
    #[doc = "4: 40 bytes"]
    Ttc40 = 4,
    #[doc = "5: 32 bytes"]
    Ttc32 = 5,
    #[doc = "6: 24 bytes"]
    Ttc24 = 6,
    #[doc = "7: 16 bytes"]
    Ttc16 = 7,
}
impl From<TTC_A> for u8 {
    #[inline(always)]
    fn from(variant: TTC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TTC` reader - Transmit threshold control"]
pub type TTC_R = crate::FieldReader<u8, TTC_A>;
impl TTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TTC_A {
        match self.bits {
            0 => TTC_A::Ttc64,
            1 => TTC_A::Ttc128,
            2 => TTC_A::Ttc192,
            3 => TTC_A::Ttc256,
            4 => TTC_A::Ttc40,
            5 => TTC_A::Ttc32,
            6 => TTC_A::Ttc24,
            7 => TTC_A::Ttc16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Ttc64`"]
    #[inline(always)]
    pub fn is_ttc64(&self) -> bool {
        *self == TTC_A::Ttc64
    }
    #[doc = "Checks if the value of the field is `Ttc128`"]
    #[inline(always)]
    pub fn is_ttc128(&self) -> bool {
        *self == TTC_A::Ttc128
    }
    #[doc = "Checks if the value of the field is `Ttc192`"]
    #[inline(always)]
    pub fn is_ttc192(&self) -> bool {
        *self == TTC_A::Ttc192
    }
    #[doc = "Checks if the value of the field is `Ttc256`"]
    #[inline(always)]
    pub fn is_ttc256(&self) -> bool {
        *self == TTC_A::Ttc256
    }
    #[doc = "Checks if the value of the field is `Ttc40`"]
    #[inline(always)]
    pub fn is_ttc40(&self) -> bool {
        *self == TTC_A::Ttc40
    }
    #[doc = "Checks if the value of the field is `Ttc32`"]
    #[inline(always)]
    pub fn is_ttc32(&self) -> bool {
        *self == TTC_A::Ttc32
    }
    #[doc = "Checks if the value of the field is `Ttc24`"]
    #[inline(always)]
    pub fn is_ttc24(&self) -> bool {
        *self == TTC_A::Ttc24
    }
    #[doc = "Checks if the value of the field is `Ttc16`"]
    #[inline(always)]
    pub fn is_ttc16(&self) -> bool {
        *self == TTC_A::Ttc16
    }
}
#[doc = "Field `TTC` writer - Transmit threshold control"]
pub type TTC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DMAOMR_SPEC, u8, TTC_A, 3, O>;
impl<'a, const O: u8> TTC_W<'a, O> {
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn ttc64(self) -> &'a mut W {
        self.variant(TTC_A::Ttc64)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn ttc128(self) -> &'a mut W {
        self.variant(TTC_A::Ttc128)
    }
    #[doc = "192 bytes"]
    #[inline(always)]
    pub fn ttc192(self) -> &'a mut W {
        self.variant(TTC_A::Ttc192)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn ttc256(self) -> &'a mut W {
        self.variant(TTC_A::Ttc256)
    }
    #[doc = "40 bytes"]
    #[inline(always)]
    pub fn ttc40(self) -> &'a mut W {
        self.variant(TTC_A::Ttc40)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn ttc32(self) -> &'a mut W {
        self.variant(TTC_A::Ttc32)
    }
    #[doc = "24 bytes"]
    #[inline(always)]
    pub fn ttc24(self) -> &'a mut W {
        self.variant(TTC_A::Ttc24)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn ttc16(self) -> &'a mut W {
        self.variant(TTC_A::Ttc16)
    }
}
#[doc = "Flush transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTF_A {
    #[doc = "1: Transmit FIFO controller logic is reset to its default values. Cleared automatically"]
    Flush = 1,
}
impl From<FTF_A> for bool {
    #[inline(always)]
    fn from(variant: FTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTF` reader - Flush transmit FIFO"]
pub type FTF_R = crate::BitReader<FTF_A>;
impl FTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FTF_A> {
        match self.bits {
            true => Some(FTF_A::Flush),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Flush`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == FTF_A::Flush
    }
}
#[doc = "Field `FTF` writer - Flush transmit FIFO"]
pub type FTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, FTF_A, O>;
impl<'a, const O: u8> FTF_W<'a, O> {
    #[doc = "Transmit FIFO controller logic is reset to its default values. Cleared automatically"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(FTF_A::Flush)
    }
}
#[doc = "Transmit store and forward\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSF_A {
    #[doc = "0: Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold"]
    CutThrough = 0,
    #[doc = "1: Transmission starts when a full frame is in the Tx FIFO"]
    StoreForward = 1,
}
impl From<TSF_A> for bool {
    #[inline(always)]
    fn from(variant: TSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSF` reader - Transmit store and forward"]
pub type TSF_R = crate::BitReader<TSF_A>;
impl TSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSF_A {
        match self.bits {
            false => TSF_A::CutThrough,
            true => TSF_A::StoreForward,
        }
    }
    #[doc = "Checks if the value of the field is `CutThrough`"]
    #[inline(always)]
    pub fn is_cut_through(&self) -> bool {
        *self == TSF_A::CutThrough
    }
    #[doc = "Checks if the value of the field is `StoreForward`"]
    #[inline(always)]
    pub fn is_store_forward(&self) -> bool {
        *self == TSF_A::StoreForward
    }
}
#[doc = "Field `TSF` writer - Transmit store and forward"]
pub type TSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, TSF_A, O>;
impl<'a, const O: u8> TSF_W<'a, O> {
    #[doc = "Transmission starts when the frame size in the Tx FIFO exceeds TTC threshold"]
    #[inline(always)]
    pub fn cut_through(self) -> &'a mut W {
        self.variant(TSF_A::CutThrough)
    }
    #[doc = "Transmission starts when a full frame is in the Tx FIFO"]
    #[inline(always)]
    pub fn store_forward(self) -> &'a mut W {
        self.variant(TSF_A::StoreForward)
    }
}
#[doc = "Field `DFRF` reader - Disable flushing of received frames"]
pub type DFRF_R = crate::BitReader<bool>;
#[doc = "Field `DFRF` writer - Disable flushing of received frames"]
pub type DFRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, bool, O>;
#[doc = "Receive store and forward\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSF_A {
    #[doc = "0: Rx FIFO operates in cut-through mode, subject to RTC bits"]
    CutThrough = 0,
    #[doc = "1: Frames are read from Rx FIFO after complete frame has been written"]
    StoreForward = 1,
}
impl From<RSF_A> for bool {
    #[inline(always)]
    fn from(variant: RSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSF` reader - Receive store and forward"]
pub type RSF_R = crate::BitReader<RSF_A>;
impl RSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSF_A {
        match self.bits {
            false => RSF_A::CutThrough,
            true => RSF_A::StoreForward,
        }
    }
    #[doc = "Checks if the value of the field is `CutThrough`"]
    #[inline(always)]
    pub fn is_cut_through(&self) -> bool {
        *self == RSF_A::CutThrough
    }
    #[doc = "Checks if the value of the field is `StoreForward`"]
    #[inline(always)]
    pub fn is_store_forward(&self) -> bool {
        *self == RSF_A::StoreForward
    }
}
#[doc = "Field `RSF` writer - Receive store and forward"]
pub type RSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, RSF_A, O>;
impl<'a, const O: u8> RSF_W<'a, O> {
    #[doc = "Rx FIFO operates in cut-through mode, subject to RTC bits"]
    #[inline(always)]
    pub fn cut_through(self) -> &'a mut W {
        self.variant(RSF_A::CutThrough)
    }
    #[doc = "Frames are read from Rx FIFO after complete frame has been written"]
    #[inline(always)]
    pub fn store_forward(self) -> &'a mut W {
        self.variant(RSF_A::StoreForward)
    }
}
#[doc = "Dropping of TCP/IP checksum error frames disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTCEFD_A {
    #[doc = "0: Drop frames with errors only in the receive checksum offload engine"]
    Enabled = 0,
    #[doc = "1: Do not drop frames that only have errors in the receive checksum offload engine"]
    Disabled = 1,
}
impl From<DTCEFD_A> for bool {
    #[inline(always)]
    fn from(variant: DTCEFD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTCEFD` reader - Dropping of TCP/IP checksum error frames disable"]
pub type DTCEFD_R = crate::BitReader<DTCEFD_A>;
impl DTCEFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTCEFD_A {
        match self.bits {
            false => DTCEFD_A::Enabled,
            true => DTCEFD_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTCEFD_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTCEFD_A::Disabled
    }
}
#[doc = "Field `DTCEFD` writer - Dropping of TCP/IP checksum error frames disable"]
pub type DTCEFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, DTCEFD_A, O>;
impl<'a, const O: u8> DTCEFD_W<'a, O> {
    #[doc = "Drop frames with errors only in the receive checksum offload engine"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DTCEFD_A::Enabled)
    }
    #[doc = "Do not drop frames that only have errors in the receive checksum offload engine"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DTCEFD_A::Disabled)
    }
}
impl R {
    #[doc = "Bit 1 - Start/stop receive"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Forward undersized good frames"]
    #[inline(always)]
    pub fn fugf(&self) -> FUGF_R {
        FUGF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Forward error frames"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - Start/stop transmission"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - Flush transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit store and forward"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable flushing of received frames"]
    #[inline(always)]
    pub fn dfrf(&self) -> DFRF_R {
        DFRF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive store and forward"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Dropping of TCP/IP checksum error frames disable"]
    #[inline(always)]
    pub fn dtcefd(&self) -> DTCEFD_R {
        DTCEFD_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Start/stop receive"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W<1> {
        SR_W::new(self)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    pub fn osf(&mut self) -> OSF_W<2> {
        OSF_W::new(self)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<3> {
        RTC_W::new(self)
    }
    #[doc = "Bit 6 - Forward undersized good frames"]
    #[inline(always)]
    pub fn fugf(&mut self) -> FUGF_W<6> {
        FUGF_W::new(self)
    }
    #[doc = "Bit 7 - Forward error frames"]
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W<7> {
        FEF_W::new(self)
    }
    #[doc = "Bit 13 - Start/stop transmission"]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<13> {
        ST_W::new(self)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    pub fn ttc(&mut self) -> TTC_W<14> {
        TTC_W::new(self)
    }
    #[doc = "Bit 20 - Flush transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&mut self) -> FTF_W<20> {
        FTF_W::new(self)
    }
    #[doc = "Bit 21 - Transmit store and forward"]
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W<21> {
        TSF_W::new(self)
    }
    #[doc = "Bit 24 - Disable flushing of received frames"]
    #[inline(always)]
    pub fn dfrf(&mut self) -> DFRF_W<24> {
        DFRF_W::new(self)
    }
    #[doc = "Bit 25 - Receive store and forward"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<25> {
        RSF_W::new(self)
    }
    #[doc = "Bit 26 - Dropping of TCP/IP checksum error frames disable"]
    #[inline(always)]
    pub fn dtcefd(&mut self) -> DTCEFD_W<26> {
        DTCEFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA operation mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaomr](index.html) module"]
pub struct DMAOMR_SPEC;
impl crate::RegisterSpec for DMAOMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaomr::R](R) reader structure"]
impl crate::Readable for DMAOMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaomr::W](W) writer structure"]
impl crate::Writable for DMAOMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAOMR to value 0"]
impl crate::Resettable for DMAOMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
