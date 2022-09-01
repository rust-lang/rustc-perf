#[doc = "Register `DMASR` reader"]
pub struct R(crate::R<DMASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMASR` writer"]
pub struct W(crate::W<DMASR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASR_SPEC>;
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
impl From<crate::W<DMASR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMASR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS` reader - Transmit status"]
pub struct TS_R(crate::FieldReader<bool, bool>);
impl TS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS` writer - Transmit status"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `TPSS` reader - Transmit process stopped status"]
pub struct TPSS_R(crate::FieldReader<bool, bool>);
impl TPSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TPSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPSS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPSS` writer - Transmit process stopped status"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TBUS` reader - Transmit buffer unavailable status"]
pub struct TBUS_R(crate::FieldReader<bool, bool>);
impl TBUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBUS` writer - Transmit buffer unavailable status"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TJTS` reader - Transmit jabber timeout status"]
pub struct TJTS_R(crate::FieldReader<bool, bool>);
impl TJTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TJTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TJTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TJTS` writer - Transmit jabber timeout status"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `ROS` reader - Receive overflow status"]
pub struct ROS_R(crate::FieldReader<bool, bool>);
impl ROS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROS` writer - Receive overflow status"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TUS` reader - Transmit underflow status"]
pub struct TUS_R(crate::FieldReader<bool, bool>);
impl TUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TUS` writer - Transmit underflow status"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RS` reader - Receive status"]
pub struct RS_R(crate::FieldReader<bool, bool>);
impl RS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS` writer - Receive status"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RBUS` reader - Receive buffer unavailable status"]
pub struct RBUS_R(crate::FieldReader<bool, bool>);
impl RBUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBUS` writer - Receive buffer unavailable status"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RPSS` reader - Receive process stopped status"]
pub struct RPSS_R(crate::FieldReader<bool, bool>);
impl RPSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPSS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPSS` writer - Receive process stopped status"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `PWTS` reader - PWTS"]
pub struct PWTS_R(crate::FieldReader<bool, bool>);
impl PWTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWTS` writer - PWTS"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `ETS` reader - Early transmit status"]
pub struct ETS_R(crate::FieldReader<bool, bool>);
impl ETS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETS` writer - Early transmit status"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `FBES` reader - Fatal bus error status"]
pub struct FBES_R(crate::FieldReader<bool, bool>);
impl FBES_R {
    pub(crate) fn new(bits: bool) -> Self {
        FBES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBES` writer - Fatal bus error status"]
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `ERS` reader - Early receive status"]
pub struct ERS_R(crate::FieldReader<bool, bool>);
impl ERS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERS` writer - Early receive status"]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `AIS` reader - Abnormal interrupt summary"]
pub struct AIS_R(crate::FieldReader<bool, bool>);
impl AIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        AIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIS` writer - Abnormal interrupt summary"]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `NIS` reader - Normal interrupt summary"]
pub struct NIS_R(crate::FieldReader<bool, bool>);
impl NIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        NIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NIS` writer - Normal interrupt summary"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
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
#[doc = "Field `RPS` reader - Receive process state"]
pub struct RPS_R(crate::FieldReader<u8, RPS_A>);
impl RPS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RPS_A> {
        match self.bits {
            0 => Some(RPS_A::STOPPED),
            1 => Some(RPS_A::RUNNINGFETCHING),
            3 => Some(RPS_A::RUNNINGWAITING),
            4 => Some(RPS_A::SUSPENDED),
            7 => Some(RPS_A::RUNNINGWRITING),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STOPPED`"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        **self == RPS_A::STOPPED
    }
    #[doc = "Checks if the value of the field is `RUNNINGFETCHING`"]
    #[inline(always)]
    pub fn is_running_fetching(&self) -> bool {
        **self == RPS_A::RUNNINGFETCHING
    }
    #[doc = "Checks if the value of the field is `RUNNINGWAITING`"]
    #[inline(always)]
    pub fn is_running_waiting(&self) -> bool {
        **self == RPS_A::RUNNINGWAITING
    }
    #[doc = "Checks if the value of the field is `SUSPENDED`"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        **self == RPS_A::SUSPENDED
    }
    #[doc = "Checks if the value of the field is `RUNNINGWRITING`"]
    #[inline(always)]
    pub fn is_running_writing(&self) -> bool {
        **self == RPS_A::RUNNINGWRITING
    }
}
impl core::ops::Deref for RPS_R {
    type Target = crate::FieldReader<u8, RPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `TPS` reader - Transmit process state"]
pub struct TPS_R(crate::FieldReader<u8, TPS_A>);
impl TPS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TPS_A> {
        match self.bits {
            0 => Some(TPS_A::STOPPED),
            1 => Some(TPS_A::RUNNINGFETCHING),
            2 => Some(TPS_A::RUNNINGWAITING),
            3 => Some(TPS_A::RUNNINGREADING),
            6 => Some(TPS_A::SUSPENDED),
            7 => Some(TPS_A::RUNNING),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STOPPED`"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        **self == TPS_A::STOPPED
    }
    #[doc = "Checks if the value of the field is `RUNNINGFETCHING`"]
    #[inline(always)]
    pub fn is_running_fetching(&self) -> bool {
        **self == TPS_A::RUNNINGFETCHING
    }
    #[doc = "Checks if the value of the field is `RUNNINGWAITING`"]
    #[inline(always)]
    pub fn is_running_waiting(&self) -> bool {
        **self == TPS_A::RUNNINGWAITING
    }
    #[doc = "Checks if the value of the field is `RUNNINGREADING`"]
    #[inline(always)]
    pub fn is_running_reading(&self) -> bool {
        **self == TPS_A::RUNNINGREADING
    }
    #[doc = "Checks if the value of the field is `SUSPENDED`"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        **self == TPS_A::SUSPENDED
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        **self == TPS_A::RUNNING
    }
}
impl core::ops::Deref for TPS_R {
    type Target = crate::FieldReader<u8, TPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EBS` reader - Error bits status"]
pub struct EBS_R(crate::FieldReader<u8, u8>);
impl EBS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EBS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMCS` reader - MMC status"]
pub struct MMCS_R(crate::FieldReader<bool, bool>);
impl MMCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMTS` reader - PMT status"]
pub struct PMTS_R(crate::FieldReader<bool, bool>);
impl PMTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTS` reader - Time stamp trigger status"]
pub struct TSTS_R(crate::FieldReader<bool, bool>);
impl TSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasr](index.html) module"]
pub struct DMASR_SPEC;
impl crate::RegisterSpec for DMASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmasr::R](R) reader structure"]
impl crate::Readable for DMASR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmasr::W](W) writer structure"]
impl crate::Writable for DMASR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMASR to value 0"]
impl crate::Resettable for DMASR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
