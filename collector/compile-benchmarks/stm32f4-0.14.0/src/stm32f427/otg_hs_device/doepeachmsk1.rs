#[doc = "Register `DOEPEACHMSK1` reader"]
pub struct R(crate::R<DOEPEACHMSK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPEACHMSK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPEACHMSK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPEACHMSK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPEACHMSK1` writer"]
pub struct W(crate::W<DOEPEACHMSK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPEACHMSK1_SPEC>;
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
impl From<crate::W<DOEPEACHMSK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPEACHMSK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRCM` reader - Transfer completed interrupt mask"]
pub struct XFRCM_R(crate::FieldReader<bool, bool>);
impl XFRCM_R {
    pub(crate) fn new(bits: bool) -> Self {
        XFRCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFRCM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XFRCM` writer - Transfer completed interrupt mask"]
pub struct XFRCM_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRCM_W<'a> {
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
#[doc = "Field `EPDM` reader - Endpoint disabled interrupt mask"]
pub struct EPDM_R(crate::FieldReader<bool, bool>);
impl EPDM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPDM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPDM` writer - Endpoint disabled interrupt mask"]
pub struct EPDM_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDM_W<'a> {
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
#[doc = "Field `TOM` reader - Timeout condition mask"]
pub struct TOM_R(crate::FieldReader<bool, bool>);
impl TOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOM` writer - Timeout condition mask"]
pub struct TOM_W<'a> {
    w: &'a mut W,
}
impl<'a> TOM_W<'a> {
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
#[doc = "Field `ITTXFEMSK` reader - IN token received when TxFIFO empty mask"]
pub struct ITTXFEMSK_R(crate::FieldReader<bool, bool>);
impl ITTXFEMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITTXFEMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITTXFEMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITTXFEMSK` writer - IN token received when TxFIFO empty mask"]
pub struct ITTXFEMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ITTXFEMSK_W<'a> {
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
#[doc = "Field `INEPNMM` reader - IN token received with EP mismatch mask"]
pub struct INEPNMM_R(crate::FieldReader<bool, bool>);
impl INEPNMM_R {
    pub(crate) fn new(bits: bool) -> Self {
        INEPNMM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPNMM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEPNMM` writer - IN token received with EP mismatch mask"]
pub struct INEPNMM_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPNMM_W<'a> {
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
#[doc = "Field `INEPNEM` reader - IN endpoint NAK effective mask"]
pub struct INEPNEM_R(crate::FieldReader<bool, bool>);
impl INEPNEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        INEPNEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPNEM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEPNEM` writer - IN endpoint NAK effective mask"]
pub struct INEPNEM_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPNEM_W<'a> {
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
#[doc = "Field `TXFURM` reader - OUT packet error mask"]
pub struct TXFURM_R(crate::FieldReader<bool, bool>);
impl TXFURM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFURM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFURM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFURM` writer - OUT packet error mask"]
pub struct TXFURM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFURM_W<'a> {
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
#[doc = "Field `BIM` reader - BNA interrupt mask"]
pub struct BIM_R(crate::FieldReader<bool, bool>);
impl BIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIM` writer - BNA interrupt mask"]
pub struct BIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BIM_W<'a> {
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
#[doc = "Field `BERRM` reader - Bubble error interrupt mask"]
pub struct BERRM_R(crate::FieldReader<bool, bool>);
impl BERRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BERRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BERRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BERRM` writer - Bubble error interrupt mask"]
pub struct BERRM_W<'a> {
    w: &'a mut W,
}
impl<'a> BERRM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `NAKM` reader - NAK interrupt mask"]
pub struct NAKM_R(crate::FieldReader<bool, bool>);
impl NAKM_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAKM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAKM` writer - NAK interrupt mask"]
pub struct NAKM_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKM_W<'a> {
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
#[doc = "Field `NYETM` reader - NYET interrupt mask"]
pub struct NYETM_R(crate::FieldReader<bool, bool>);
impl NYETM_R {
    pub(crate) fn new(bits: bool) -> Self {
        NYETM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NYETM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NYETM` writer - NYET interrupt mask"]
pub struct NYETM_W<'a> {
    w: &'a mut W,
}
impl<'a> NYETM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timeout condition mask"]
    #[inline(always)]
    pub fn tom(&self) -> TOM_R {
        TOM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    pub fn ittxfemsk(&self) -> ITTXFEMSK_R {
        ITTXFEMSK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    pub fn inepnmm(&self) -> INEPNMM_R {
        INEPNMM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    pub fn inepnem(&self) -> INEPNEM_R {
        INEPNEM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    pub fn txfurm(&self) -> TXFURM_R {
        TXFURM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn bim(&self) -> BIM_R {
        BIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Bubble error interrupt mask"]
    #[inline(always)]
    pub fn berrm(&self) -> BERRM_R {
        BERRM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt mask"]
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt mask"]
    #[inline(always)]
    pub fn nyetm(&self) -> NYETM_R {
        NYETM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W {
        XFRCM_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&mut self) -> EPDM_W {
        EPDM_W { w: self }
    }
    #[doc = "Bit 3 - Timeout condition mask"]
    #[inline(always)]
    pub fn tom(&mut self) -> TOM_W {
        TOM_W { w: self }
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    pub fn ittxfemsk(&mut self) -> ITTXFEMSK_W {
        ITTXFEMSK_W { w: self }
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    pub fn inepnmm(&mut self) -> INEPNMM_W {
        INEPNMM_W { w: self }
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    pub fn inepnem(&mut self) -> INEPNEM_W {
        INEPNEM_W { w: self }
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    pub fn txfurm(&mut self) -> TXFURM_W {
        TXFURM_W { w: self }
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn bim(&mut self) -> BIM_W {
        BIM_W { w: self }
    }
    #[doc = "Bit 12 - Bubble error interrupt mask"]
    #[inline(always)]
    pub fn berrm(&mut self) -> BERRM_W {
        BERRM_W { w: self }
    }
    #[doc = "Bit 13 - NAK interrupt mask"]
    #[inline(always)]
    pub fn nakm(&mut self) -> NAKM_W {
        NAKM_W { w: self }
    }
    #[doc = "Bit 14 - NYET interrupt mask"]
    #[inline(always)]
    pub fn nyetm(&mut self) -> NYETM_W {
        NYETM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS device each OUT endpoint-1 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepeachmsk1](index.html) module"]
pub struct DOEPEACHMSK1_SPEC;
impl crate::RegisterSpec for DOEPEACHMSK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepeachmsk1::R](R) reader structure"]
impl crate::Readable for DOEPEACHMSK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepeachmsk1::W](W) writer structure"]
impl crate::Writable for DOEPEACHMSK1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPEACHMSK1 to value 0"]
impl crate::Resettable for DOEPEACHMSK1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
