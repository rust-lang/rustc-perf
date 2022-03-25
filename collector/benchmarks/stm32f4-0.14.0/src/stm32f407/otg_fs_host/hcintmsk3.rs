#[doc = "Register `HCINTMSK3` reader"]
pub struct R(crate::R<HCINTMSK3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINTMSK3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINTMSK3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINTMSK3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCINTMSK3` writer"]
pub struct W(crate::W<HCINTMSK3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINTMSK3_SPEC>;
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
impl From<crate::W<HCINTMSK3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINTMSK3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRCM` reader - Transfer completed mask"]
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
#[doc = "Field `XFRCM` writer - Transfer completed mask"]
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
#[doc = "Field `CHHM` reader - Channel halted mask"]
pub struct CHHM_R(crate::FieldReader<bool, bool>);
impl CHHM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHHM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHHM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHHM` writer - Channel halted mask"]
pub struct CHHM_W<'a> {
    w: &'a mut W,
}
impl<'a> CHHM_W<'a> {
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
#[doc = "Field `STALLM` reader - STALL response received interrupt mask"]
pub struct STALLM_R(crate::FieldReader<bool, bool>);
impl STALLM_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALLM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALLM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALLM` writer - STALL response received interrupt mask"]
pub struct STALLM_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLM_W<'a> {
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
#[doc = "Field `NAKM` reader - NAK response received interrupt mask"]
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
#[doc = "Field `NAKM` writer - NAK response received interrupt mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `ACKM` reader - ACK response received/transmitted interrupt mask"]
pub struct ACKM_R(crate::FieldReader<bool, bool>);
impl ACKM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACKM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKM` writer - ACK response received/transmitted interrupt mask"]
pub struct ACKM_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKM_W<'a> {
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
#[doc = "Field `NYET` reader - response received interrupt mask"]
pub struct NYET_R(crate::FieldReader<bool, bool>);
impl NYET_R {
    pub(crate) fn new(bits: bool) -> Self {
        NYET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NYET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NYET` writer - response received interrupt mask"]
pub struct NYET_W<'a> {
    w: &'a mut W,
}
impl<'a> NYET_W<'a> {
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
#[doc = "Field `TXERRM` reader - Transaction error mask"]
pub struct TXERRM_R(crate::FieldReader<bool, bool>);
impl TXERRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXERRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXERRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXERRM` writer - Transaction error mask"]
pub struct TXERRM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERRM_W<'a> {
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
#[doc = "Field `BBERRM` reader - Babble error mask"]
pub struct BBERRM_R(crate::FieldReader<bool, bool>);
impl BBERRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BBERRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBERRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BBERRM` writer - Babble error mask"]
pub struct BBERRM_W<'a> {
    w: &'a mut W,
}
impl<'a> BBERRM_W<'a> {
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
#[doc = "Field `FRMORM` reader - Frame overrun mask"]
pub struct FRMORM_R(crate::FieldReader<bool, bool>);
impl FRMORM_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRMORM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRMORM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRMORM` writer - Frame overrun mask"]
pub struct FRMORM_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMORM_W<'a> {
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
#[doc = "Field `DTERRM` reader - Data toggle error mask"]
pub struct DTERRM_R(crate::FieldReader<bool, bool>);
impl DTERRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTERRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTERRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTERRM` writer - Data toggle error mask"]
pub struct DTERRM_W<'a> {
    w: &'a mut W,
}
impl<'a> DTERRM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    pub fn chhm(&self) -> CHHM_R {
        CHHM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - STALL response received interrupt mask"]
    #[inline(always)]
    pub fn stallm(&self) -> STALLM_R {
        STALLM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    pub fn ackm(&self) -> ACKM_R {
        ACKM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - response received interrupt mask"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transaction error mask"]
    #[inline(always)]
    pub fn txerrm(&self) -> TXERRM_R {
        TXERRM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Babble error mask"]
    #[inline(always)]
    pub fn bberrm(&self) -> BBERRM_R {
        BBERRM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    pub fn frmorm(&self) -> FRMORM_R {
        FRMORM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    pub fn dterrm(&self) -> DTERRM_R {
        DTERRM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W {
        XFRCM_W { w: self }
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    pub fn chhm(&mut self) -> CHHM_W {
        CHHM_W { w: self }
    }
    #[doc = "Bit 3 - STALL response received interrupt mask"]
    #[inline(always)]
    pub fn stallm(&mut self) -> STALLM_W {
        STALLM_W { w: self }
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    pub fn nakm(&mut self) -> NAKM_W {
        NAKM_W { w: self }
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    pub fn ackm(&mut self) -> ACKM_W {
        ACKM_W { w: self }
    }
    #[doc = "Bit 6 - response received interrupt mask"]
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W {
        NYET_W { w: self }
    }
    #[doc = "Bit 7 - Transaction error mask"]
    #[inline(always)]
    pub fn txerrm(&mut self) -> TXERRM_W {
        TXERRM_W { w: self }
    }
    #[doc = "Bit 8 - Babble error mask"]
    #[inline(always)]
    pub fn bberrm(&mut self) -> BBERRM_W {
        BBERRM_W { w: self }
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    pub fn frmorm(&mut self) -> FRMORM_W {
        FRMORM_W { w: self }
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    pub fn dterrm(&mut self) -> DTERRM_W {
        DTERRM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk3](index.html) module"]
pub struct HCINTMSK3_SPEC;
impl crate::RegisterSpec for HCINTMSK3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcintmsk3::R](R) reader structure"]
impl crate::Readable for HCINTMSK3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcintmsk3::W](W) writer structure"]
impl crate::Writable for HCINTMSK3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCINTMSK3 to value 0"]
impl crate::Resettable for HCINTMSK3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
