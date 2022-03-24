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
#[doc = "Field `TS` reader - TS"]
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
#[doc = "Field `TS` writer - TS"]
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
#[doc = "Field `TPSS` reader - TPSS"]
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
#[doc = "Field `TPSS` writer - TPSS"]
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
#[doc = "Field `TBUS` reader - TBUS"]
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
#[doc = "Field `TBUS` writer - TBUS"]
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
#[doc = "Field `TJTS` reader - TJTS"]
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
#[doc = "Field `TJTS` writer - TJTS"]
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
#[doc = "Field `ROS` reader - ROS"]
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
#[doc = "Field `ROS` writer - ROS"]
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
#[doc = "Field `TUS` reader - TUS"]
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
#[doc = "Field `TUS` writer - TUS"]
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
#[doc = "Field `RS` reader - RS"]
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
#[doc = "Field `RS` writer - RS"]
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
#[doc = "Field `RBUS` reader - RBUS"]
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
#[doc = "Field `RBUS` writer - RBUS"]
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
#[doc = "Field `RPSS` reader - RPSS"]
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
#[doc = "Field `RPSS` writer - RPSS"]
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
#[doc = "Field `ETS` reader - ETS"]
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
#[doc = "Field `ETS` writer - ETS"]
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
#[doc = "Field `FBES` reader - FBES"]
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
#[doc = "Field `FBES` writer - FBES"]
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
#[doc = "Field `ERS` reader - ERS"]
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
#[doc = "Field `ERS` writer - ERS"]
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
#[doc = "Field `AIS` reader - AIS"]
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
#[doc = "Field `AIS` writer - AIS"]
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
#[doc = "Field `NIS` reader - NIS"]
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
#[doc = "Field `NIS` writer - NIS"]
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
#[doc = "Field `RPS` reader - RPS"]
pub struct RPS_R(crate::FieldReader<u8, u8>);
impl RPS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPS` reader - TPS"]
pub struct TPS_R(crate::FieldReader<u8, u8>);
impl TPS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EBS` reader - EBS"]
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
#[doc = "Field `MMCS` reader - MMCS"]
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
#[doc = "Field `PMTS` reader - PMTS"]
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
#[doc = "Field `TSTS` reader - TSTS"]
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
    #[doc = "Bit 0 - TS"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TPSS"]
    #[inline(always)]
    pub fn tpss(&self) -> TPSS_R {
        TPSS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TBUS"]
    #[inline(always)]
    pub fn tbus(&self) -> TBUS_R {
        TBUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TJTS"]
    #[inline(always)]
    pub fn tjts(&self) -> TJTS_R {
        TJTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ROS"]
    #[inline(always)]
    pub fn ros(&self) -> ROS_R {
        ROS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TUS"]
    #[inline(always)]
    pub fn tus(&self) -> TUS_R {
        TUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RS"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RBUS"]
    #[inline(always)]
    pub fn rbus(&self) -> RBUS_R {
        RBUS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RPSS"]
    #[inline(always)]
    pub fn rpss(&self) -> RPSS_R {
        RPSS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PWTS"]
    #[inline(always)]
    pub fn pwts(&self) -> PWTS_R {
        PWTS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ETS"]
    #[inline(always)]
    pub fn ets(&self) -> ETS_R {
        ETS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - FBES"]
    #[inline(always)]
    pub fn fbes(&self) -> FBES_R {
        FBES_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ERS"]
    #[inline(always)]
    pub fn ers(&self) -> ERS_R {
        ERS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AIS"]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - NIS"]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - RPS"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - TPS"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - EBS"]
    #[inline(always)]
    pub fn ebs(&self) -> EBS_R {
        EBS_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bit 27 - MMCS"]
    #[inline(always)]
    pub fn mmcs(&self) -> MMCS_R {
        MMCS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PMTS"]
    #[inline(always)]
    pub fn pmts(&self) -> PMTS_R {
        PMTS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TSTS"]
    #[inline(always)]
    pub fn tsts(&self) -> TSTS_R {
        TSTS_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TS"]
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W {
        TS_W { w: self }
    }
    #[doc = "Bit 1 - TPSS"]
    #[inline(always)]
    pub fn tpss(&mut self) -> TPSS_W {
        TPSS_W { w: self }
    }
    #[doc = "Bit 2 - TBUS"]
    #[inline(always)]
    pub fn tbus(&mut self) -> TBUS_W {
        TBUS_W { w: self }
    }
    #[doc = "Bit 3 - TJTS"]
    #[inline(always)]
    pub fn tjts(&mut self) -> TJTS_W {
        TJTS_W { w: self }
    }
    #[doc = "Bit 4 - ROS"]
    #[inline(always)]
    pub fn ros(&mut self) -> ROS_W {
        ROS_W { w: self }
    }
    #[doc = "Bit 5 - TUS"]
    #[inline(always)]
    pub fn tus(&mut self) -> TUS_W {
        TUS_W { w: self }
    }
    #[doc = "Bit 6 - RS"]
    #[inline(always)]
    pub fn rs(&mut self) -> RS_W {
        RS_W { w: self }
    }
    #[doc = "Bit 7 - RBUS"]
    #[inline(always)]
    pub fn rbus(&mut self) -> RBUS_W {
        RBUS_W { w: self }
    }
    #[doc = "Bit 8 - RPSS"]
    #[inline(always)]
    pub fn rpss(&mut self) -> RPSS_W {
        RPSS_W { w: self }
    }
    #[doc = "Bit 9 - PWTS"]
    #[inline(always)]
    pub fn pwts(&mut self) -> PWTS_W {
        PWTS_W { w: self }
    }
    #[doc = "Bit 10 - ETS"]
    #[inline(always)]
    pub fn ets(&mut self) -> ETS_W {
        ETS_W { w: self }
    }
    #[doc = "Bit 13 - FBES"]
    #[inline(always)]
    pub fn fbes(&mut self) -> FBES_W {
        FBES_W { w: self }
    }
    #[doc = "Bit 14 - ERS"]
    #[inline(always)]
    pub fn ers(&mut self) -> ERS_W {
        ERS_W { w: self }
    }
    #[doc = "Bit 15 - AIS"]
    #[inline(always)]
    pub fn ais(&mut self) -> AIS_W {
        AIS_W { w: self }
    }
    #[doc = "Bit 16 - NIS"]
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
