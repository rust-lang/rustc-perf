#[doc = "Reader of register DMASR"]
pub type R = crate::R<u32, super::DMASR>;
#[doc = "Writer for register DMASR"]
pub type W = crate::W<u32, super::DMASR>;
#[doc = "Register DMASR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMASR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TS`"]
pub type TS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS`"]
pub struct TS_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_W<'a> {
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
#[doc = "Reader of field `TPSS`"]
pub type TPSS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TPSS`"]
pub struct TPSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TPSS_W<'a> {
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
#[doc = "Reader of field `TBUS`"]
pub type TBUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBUS`"]
pub struct TBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBUS_W<'a> {
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
#[doc = "Reader of field `TJTS`"]
pub type TJTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TJTS`"]
pub struct TJTS_W<'a> {
    w: &'a mut W,
}
impl<'a> TJTS_W<'a> {
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
#[doc = "Reader of field `ROS`"]
pub type ROS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROS`"]
pub struct ROS_W<'a> {
    w: &'a mut W,
}
impl<'a> ROS_W<'a> {
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
#[doc = "Reader of field `TUS`"]
pub type TUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TUS`"]
pub struct TUS_W<'a> {
    w: &'a mut W,
}
impl<'a> TUS_W<'a> {
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
#[doc = "Reader of field `RS`"]
pub type RS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RS`"]
pub struct RS_W<'a> {
    w: &'a mut W,
}
impl<'a> RS_W<'a> {
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
#[doc = "Reader of field `RBUS`"]
pub type RBUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RBUS`"]
pub struct RBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> RBUS_W<'a> {
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
#[doc = "Reader of field `RPSS`"]
pub type RPSS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPSS`"]
pub struct RPSS_W<'a> {
    w: &'a mut W,
}
impl<'a> RPSS_W<'a> {
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
#[doc = "Reader of field `PWTS`"]
pub type PWTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWTS`"]
pub struct PWTS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWTS_W<'a> {
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
#[doc = "Reader of field `ETS`"]
pub type ETS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETS`"]
pub struct ETS_W<'a> {
    w: &'a mut W,
}
impl<'a> ETS_W<'a> {
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
#[doc = "Reader of field `FBES`"]
pub type FBES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBES`"]
pub struct FBES_W<'a> {
    w: &'a mut W,
}
impl<'a> FBES_W<'a> {
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
#[doc = "Reader of field `ERS`"]
pub type ERS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERS`"]
pub struct ERS_W<'a> {
    w: &'a mut W,
}
impl<'a> ERS_W<'a> {
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
#[doc = "Reader of field `AIS`"]
pub type AIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AIS`"]
pub struct AIS_W<'a> {
    w: &'a mut W,
}
impl<'a> AIS_W<'a> {
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
#[doc = "Reader of field `NIS`"]
pub type NIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NIS`"]
pub struct NIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NIS_W<'a> {
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
#[doc = "Receive process state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RPS_A {
    #[doc = "0: Stopped, reset or Stop Receive command issued"]
    STOPPED = 0,
    #[doc = "1: Running, fetching receive transfer descriptor"]
    RUNNINGFETCHING = 1,
    #[doc = "3: Running, waiting for receive packet"]
    RUNNINGWAITING = 3,
    #[doc = "4: Suspended, receive descriptor unavailable"]
    SUSPENDED = 4,
    #[doc = "7: Running, writing data to host memory buffer"]
    RUNNINGWRITING = 7,
}
impl From<RPS_A> for u8 {
    #[inline(always)]
    fn from(variant: RPS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RPS`"]
pub type RPS_R = crate::R<u8, RPS_A>;
impl RPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RPS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RPS_A::STOPPED),
            1 => Val(RPS_A::RUNNINGFETCHING),
            3 => Val(RPS_A::RUNNINGWAITING),
            4 => Val(RPS_A::SUSPENDED),
            7 => Val(RPS_A::RUNNINGWRITING),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOPPED`"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == RPS_A::STOPPED
    }
    #[doc = "Checks if the value of the field is `RUNNINGFETCHING`"]
    #[inline(always)]
    pub fn is_running_fetching(&self) -> bool {
        *self == RPS_A::RUNNINGFETCHING
    }
    #[doc = "Checks if the value of the field is `RUNNINGWAITING`"]
    #[inline(always)]
    pub fn is_running_waiting(&self) -> bool {
        *self == RPS_A::RUNNINGWAITING
    }
    #[doc = "Checks if the value of the field is `SUSPENDED`"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == RPS_A::SUSPENDED
    }
    #[doc = "Checks if the value of the field is `RUNNINGWRITING`"]
    #[inline(always)]
    pub fn is_running_writing(&self) -> bool {
        *self == RPS_A::RUNNINGWRITING
    }
}
#[doc = "Transmit process state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TPS_A {
    #[doc = "0: Stopped, Reset or Stop Transmit command issued"]
    STOPPED = 0,
    #[doc = "1: Running, fetching transmit transfer descriptor"]
    RUNNINGFETCHING = 1,
    #[doc = "2: Running, waiting for status"]
    RUNNINGWAITING = 2,
    #[doc = "3: Running, reading data from host memory buffer"]
    RUNNINGREADING = 3,
    #[doc = "6: Suspended, transmit descriptor unavailable or transmit buffer underflow"]
    SUSPENDED = 6,
    #[doc = "7: Running, closing transmit descriptor"]
    RUNNING = 7,
}
impl From<TPS_A> for u8 {
    #[inline(always)]
    fn from(variant: TPS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TPS`"]
pub type TPS_R = crate::R<u8, TPS_A>;
impl TPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TPS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TPS_A::STOPPED),
            1 => Val(TPS_A::RUNNINGFETCHING),
            2 => Val(TPS_A::RUNNINGWAITING),
            3 => Val(TPS_A::RUNNINGREADING),
            6 => Val(TPS_A::SUSPENDED),
            7 => Val(TPS_A::RUNNING),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOPPED`"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == TPS_A::STOPPED
    }
    #[doc = "Checks if the value of the field is `RUNNINGFETCHING`"]
    #[inline(always)]
    pub fn is_running_fetching(&self) -> bool {
        *self == TPS_A::RUNNINGFETCHING
    }
    #[doc = "Checks if the value of the field is `RUNNINGWAITING`"]
    #[inline(always)]
    pub fn is_running_waiting(&self) -> bool {
        *self == TPS_A::RUNNINGWAITING
    }
    #[doc = "Checks if the value of the field is `RUNNINGREADING`"]
    #[inline(always)]
    pub fn is_running_reading(&self) -> bool {
        *self == TPS_A::RUNNINGREADING
    }
    #[doc = "Checks if the value of the field is `SUSPENDED`"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == TPS_A::SUSPENDED
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == TPS_A::RUNNING
    }
}
#[doc = "Reader of field `EBS`"]
pub type EBS_R = crate::R<u8, u8>;
#[doc = "Reader of field `MMCS`"]
pub type MMCS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PMTS`"]
pub type PMTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSTS`"]
pub type TSTS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Transmit status"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit process stopped status"]
    #[inline(always)]
    pub fn tpss(&self) -> TPSS_R {
        TPSS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable status"]
    #[inline(always)]
    pub fn tbus(&self) -> TBUS_R {
        TBUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit jabber timeout status"]
    #[inline(always)]
    pub fn tjts(&self) -> TJTS_R {
        TJTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive overflow status"]
    #[inline(always)]
    pub fn ros(&self) -> ROS_R {
        ROS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit underflow status"]
    #[inline(always)]
    pub fn tus(&self) -> TUS_R {
        TUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive status"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receive buffer unavailable status"]
    #[inline(always)]
    pub fn rbus(&self) -> RBUS_R {
        RBUS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive process stopped status"]
    #[inline(always)]
    pub fn rpss(&self) -> RPSS_R {
        RPSS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PWTS"]
    #[inline(always)]
    pub fn pwts(&self) -> PWTS_R {
        PWTS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Early transmit status"]
    #[inline(always)]
    pub fn ets(&self) -> ETS_R {
        ETS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fatal bus error status"]
    #[inline(always)]
    pub fn fbes(&self) -> FBES_R {
        FBES_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Early receive status"]
    #[inline(always)]
    pub fn ers(&self) -> ERS_R {
        ERS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary"]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Normal interrupt summary"]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - Receive process state"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Transmit process state"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - Error bits status"]
    #[inline(always)]
    pub fn ebs(&self) -> EBS_R {
        EBS_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bit 27 - MMC status"]
    #[inline(always)]
    pub fn mmcs(&self) -> MMCS_R {
        MMCS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PMT status"]
    #[inline(always)]
    pub fn pmts(&self) -> PMTS_R {
        PMTS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Time stamp trigger status"]
    #[inline(always)]
    pub fn tsts(&self) -> TSTS_R {
        TSTS_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit status"]
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W {
        TS_W { w: self }
    }
    #[doc = "Bit 1 - Transmit process stopped status"]
    #[inline(always)]
    pub fn tpss(&mut self) -> TPSS_W {
        TPSS_W { w: self }
    }
    #[doc = "Bit 2 - Transmit buffer unavailable status"]
    #[inline(always)]
    pub fn tbus(&mut self) -> TBUS_W {
        TBUS_W { w: self }
    }
    #[doc = "Bit 3 - Transmit jabber timeout status"]
    #[inline(always)]
    pub fn tjts(&mut self) -> TJTS_W {
        TJTS_W { w: self }
    }
    #[doc = "Bit 4 - Receive overflow status"]
    #[inline(always)]
    pub fn ros(&mut self) -> ROS_W {
        ROS_W { w: self }
    }
    #[doc = "Bit 5 - Transmit underflow status"]
    #[inline(always)]
    pub fn tus(&mut self) -> TUS_W {
        TUS_W { w: self }
    }
    #[doc = "Bit 6 - Receive status"]
    #[inline(always)]
    pub fn rs(&mut self) -> RS_W {
        RS_W { w: self }
    }
    #[doc = "Bit 7 - Receive buffer unavailable status"]
    #[inline(always)]
    pub fn rbus(&mut self) -> RBUS_W {
        RBUS_W { w: self }
    }
    #[doc = "Bit 8 - Receive process stopped status"]
    #[inline(always)]
    pub fn rpss(&mut self) -> RPSS_W {
        RPSS_W { w: self }
    }
    #[doc = "Bit 9 - PWTS"]
    #[inline(always)]
    pub fn pwts(&mut self) -> PWTS_W {
        PWTS_W { w: self }
    }
    #[doc = "Bit 10 - Early transmit status"]
    #[inline(always)]
    pub fn ets(&mut self) -> ETS_W {
        ETS_W { w: self }
    }
    #[doc = "Bit 13 - Fatal bus error status"]
    #[inline(always)]
    pub fn fbes(&mut self) -> FBES_W {
        FBES_W { w: self }
    }
    #[doc = "Bit 14 - Early receive status"]
    #[inline(always)]
    pub fn ers(&mut self) -> ERS_W {
        ERS_W { w: self }
    }
    #[doc = "Bit 15 - Abnormal interrupt summary"]
    #[inline(always)]
    pub fn ais(&mut self) -> AIS_W {
        AIS_W { w: self }
    }
    #[doc = "Bit 16 - Normal interrupt summary"]
    #[inline(always)]
    pub fn nis(&mut self) -> NIS_W {
        NIS_W { w: self }
    }
}
