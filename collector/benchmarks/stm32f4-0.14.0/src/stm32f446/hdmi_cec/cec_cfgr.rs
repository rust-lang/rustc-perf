#[doc = "Register `CEC_CFGR` reader"]
pub struct R(crate::R<CEC_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEC_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEC_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEC_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CEC_CFGR` writer"]
pub struct W(crate::W<CEC_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEC_CFGR_SPEC>;
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
impl From<crate::W<CEC_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEC_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSTN` reader - Listen mode"]
pub struct LSTN_R(crate::FieldReader<bool, bool>);
impl LSTN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSTN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTN` writer - Listen mode"]
pub struct LSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `OAR` reader - Own addresses configuration"]
pub struct OAR_R(crate::FieldReader<u16, u16>);
impl OAR_R {
    pub(crate) fn new(bits: u16) -> Self {
        OAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OAR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OAR` writer - Own addresses configuration"]
pub struct OAR_W<'a> {
    w: &'a mut W,
}
impl<'a> OAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | ((value as u32 & 0x7fff) << 16);
        self.w
    }
}
#[doc = "Field `SFTOP` reader - SFT Option Bit"]
pub struct SFTOP_R(crate::FieldReader<bool, bool>);
impl SFTOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SFTOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFTOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFTOP` writer - SFT Option Bit"]
pub struct SFTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTOP_W<'a> {
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
#[doc = "Field `BRDNOGEN` reader - Avoid Error-Bit Generation in Broadcast"]
pub struct BRDNOGEN_R(crate::FieldReader<bool, bool>);
impl BRDNOGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRDNOGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRDNOGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRDNOGEN` writer - Avoid Error-Bit Generation in Broadcast"]
pub struct BRDNOGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRDNOGEN_W<'a> {
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
#[doc = "Field `LBPEGEN` reader - Generate Error-Bit on Long Bit Period Error"]
pub struct LBPEGEN_R(crate::FieldReader<bool, bool>);
impl LBPEGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LBPEGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LBPEGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBPEGEN` writer - Generate Error-Bit on Long Bit Period Error"]
pub struct LBPEGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LBPEGEN_W<'a> {
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
#[doc = "Field `BREGEN` reader - Generate Error-Bit on Bit Rising Error"]
pub struct BREGEN_R(crate::FieldReader<bool, bool>);
impl BREGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BREGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BREGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BREGEN` writer - Generate Error-Bit on Bit Rising Error"]
pub struct BREGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BREGEN_W<'a> {
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
#[doc = "Field `BRESTP` reader - Rx-Stop on Bit Rising Error"]
pub struct BRESTP_R(crate::FieldReader<bool, bool>);
impl BRESTP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRESTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRESTP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRESTP` writer - Rx-Stop on Bit Rising Error"]
pub struct BRESTP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRESTP_W<'a> {
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
#[doc = "Field `RXTOL` reader - Rx-Tolerance"]
pub struct RXTOL_R(crate::FieldReader<bool, bool>);
impl RXTOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTOL` writer - Rx-Tolerance"]
pub struct RXTOL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTOL_W<'a> {
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
#[doc = "Field `SFT` reader - Signal Free Time"]
pub struct SFT_R(crate::FieldReader<u8, u8>);
impl SFT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFT` writer - Signal Free Time"]
pub struct SFT_W<'a> {
    w: &'a mut W,
}
impl<'a> SFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Listen mode"]
    #[inline(always)]
    pub fn lstn(&self) -> LSTN_R {
        LSTN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 16:30 - Own addresses configuration"]
    #[inline(always)]
    pub fn oar(&self) -> OAR_R {
        OAR_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 8 - SFT Option Bit"]
    #[inline(always)]
    pub fn sftop(&self) -> SFTOP_R {
        SFTOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Avoid Error-Bit Generation in Broadcast"]
    #[inline(always)]
    pub fn brdnogen(&self) -> BRDNOGEN_R {
        BRDNOGEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Generate Error-Bit on Long Bit Period Error"]
    #[inline(always)]
    pub fn lbpegen(&self) -> LBPEGEN_R {
        LBPEGEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Generate Error-Bit on Bit Rising Error"]
    #[inline(always)]
    pub fn bregen(&self) -> BREGEN_R {
        BREGEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx-Stop on Bit Rising Error"]
    #[inline(always)]
    pub fn brestp(&self) -> BRESTP_R {
        BRESTP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rx-Tolerance"]
    #[inline(always)]
    pub fn rxtol(&self) -> RXTOL_R {
        RXTOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Signal Free Time"]
    #[inline(always)]
    pub fn sft(&self) -> SFT_R {
        SFT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Listen mode"]
    #[inline(always)]
    pub fn lstn(&mut self) -> LSTN_W {
        LSTN_W { w: self }
    }
    #[doc = "Bits 16:30 - Own addresses configuration"]
    #[inline(always)]
    pub fn oar(&mut self) -> OAR_W {
        OAR_W { w: self }
    }
    #[doc = "Bit 8 - SFT Option Bit"]
    #[inline(always)]
    pub fn sftop(&mut self) -> SFTOP_W {
        SFTOP_W { w: self }
    }
    #[doc = "Bit 7 - Avoid Error-Bit Generation in Broadcast"]
    #[inline(always)]
    pub fn brdnogen(&mut self) -> BRDNOGEN_W {
        BRDNOGEN_W { w: self }
    }
    #[doc = "Bit 6 - Generate Error-Bit on Long Bit Period Error"]
    #[inline(always)]
    pub fn lbpegen(&mut self) -> LBPEGEN_W {
        LBPEGEN_W { w: self }
    }
    #[doc = "Bit 5 - Generate Error-Bit on Bit Rising Error"]
    #[inline(always)]
    pub fn bregen(&mut self) -> BREGEN_W {
        BREGEN_W { w: self }
    }
    #[doc = "Bit 4 - Rx-Stop on Bit Rising Error"]
    #[inline(always)]
    pub fn brestp(&mut self) -> BRESTP_W {
        BRESTP_W { w: self }
    }
    #[doc = "Bit 3 - Rx-Tolerance"]
    #[inline(always)]
    pub fn rxtol(&mut self) -> RXTOL_W {
        RXTOL_W { w: self }
    }
    #[doc = "Bits 0:2 - Signal Free Time"]
    #[inline(always)]
    pub fn sft(&mut self) -> SFT_W {
        SFT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CEC configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_cfgr](index.html) module"]
pub struct CEC_CFGR_SPEC;
impl crate::RegisterSpec for CEC_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cec_cfgr::R](R) reader structure"]
impl crate::Readable for CEC_CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cec_cfgr::W](W) writer structure"]
impl crate::Writable for CEC_CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CEC_CFGR to value 0"]
impl crate::Resettable for CEC_CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
