#[doc = "Register `DIEPINT6` reader"]
pub struct R(crate::R<DIEPINT6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPINT6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPINT6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPINT6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPINT6` writer"]
pub struct W(crate::W<DIEPINT6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPINT6_SPEC>;
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
impl From<crate::W<DIEPINT6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPINT6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRC` reader - Transfer completed interrupt"]
pub struct XFRC_R(crate::FieldReader<bool, bool>);
impl XFRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        XFRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XFRC` writer - Transfer completed interrupt"]
pub struct XFRC_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRC_W<'a> {
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
#[doc = "Field `EPDISD` reader - Endpoint disabled interrupt"]
pub struct EPDISD_R(crate::FieldReader<bool, bool>);
impl EPDISD_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPDISD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPDISD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPDISD` writer - Endpoint disabled interrupt"]
pub struct EPDISD_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDISD_W<'a> {
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
#[doc = "Field `TOC` reader - Timeout condition"]
pub struct TOC_R(crate::FieldReader<bool, bool>);
impl TOC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOC` writer - Timeout condition"]
pub struct TOC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOC_W<'a> {
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
#[doc = "Field `ITTXFE` reader - IN token received when TxFIFO is empty"]
pub struct ITTXFE_R(crate::FieldReader<bool, bool>);
impl ITTXFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITTXFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITTXFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITTXFE` writer - IN token received when TxFIFO is empty"]
pub struct ITTXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITTXFE_W<'a> {
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
#[doc = "Field `INEPNE` reader - IN endpoint NAK effective"]
pub struct INEPNE_R(crate::FieldReader<bool, bool>);
impl INEPNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        INEPNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEPNE` writer - IN endpoint NAK effective"]
pub struct INEPNE_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPNE_W<'a> {
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
#[doc = "Field `TXFE` reader - Transmit FIFO empty"]
pub struct TXFE_R(crate::FieldReader<bool, bool>);
impl TXFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFOUDRN` reader - Transmit Fifo Underrun"]
pub struct TXFIFOUDRN_R(crate::FieldReader<bool, bool>);
impl TXFIFOUDRN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFOUDRN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOUDRN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFOUDRN` writer - Transmit Fifo Underrun"]
pub struct TXFIFOUDRN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFOUDRN_W<'a> {
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
#[doc = "Field `BNA` reader - Buffer not available interrupt"]
pub struct BNA_R(crate::FieldReader<bool, bool>);
impl BNA_R {
    pub(crate) fn new(bits: bool) -> Self {
        BNA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BNA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BNA` writer - Buffer not available interrupt"]
pub struct BNA_W<'a> {
    w: &'a mut W,
}
impl<'a> BNA_W<'a> {
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
#[doc = "Field `PKTDRPSTS` reader - Packet dropped status"]
pub struct PKTDRPSTS_R(crate::FieldReader<bool, bool>);
impl PKTDRPSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKTDRPSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKTDRPSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKTDRPSTS` writer - Packet dropped status"]
pub struct PKTDRPSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTDRPSTS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `BERR` reader - Babble error interrupt"]
pub struct BERR_R(crate::FieldReader<bool, bool>);
impl BERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BERR` writer - Babble error interrupt"]
pub struct BERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BERR_W<'a> {
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
#[doc = "Field `NAK` reader - NAK interrupt"]
pub struct NAK_R(crate::FieldReader<bool, bool>);
impl NAK_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAK` writer - NAK interrupt"]
pub struct NAK_W<'a> {
    w: &'a mut W,
}
impl<'a> NAK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timeout condition"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO is empty"]
    #[inline(always)]
    pub fn ittxfe(&self) -> ITTXFE_R {
        ITTXFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    pub fn inepne(&self) -> INEPNE_R {
        INEPNE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit Fifo Underrun"]
    #[inline(always)]
    pub fn txfifoudrn(&self) -> TXFIFOUDRN_R {
        TXFIFOUDRN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Buffer not available interrupt"]
    #[inline(always)]
    pub fn bna(&self) -> BNA_R {
        BNA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Packet dropped status"]
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Babble error interrupt"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W {
        XFRC_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W {
        EPDISD_W { w: self }
    }
    #[doc = "Bit 3 - Timeout condition"]
    #[inline(always)]
    pub fn toc(&mut self) -> TOC_W {
        TOC_W { w: self }
    }
    #[doc = "Bit 4 - IN token received when TxFIFO is empty"]
    #[inline(always)]
    pub fn ittxfe(&mut self) -> ITTXFE_W {
        ITTXFE_W { w: self }
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    pub fn inepne(&mut self) -> INEPNE_W {
        INEPNE_W { w: self }
    }
    #[doc = "Bit 8 - Transmit Fifo Underrun"]
    #[inline(always)]
    pub fn txfifoudrn(&mut self) -> TXFIFOUDRN_W {
        TXFIFOUDRN_W { w: self }
    }
    #[doc = "Bit 9 - Buffer not available interrupt"]
    #[inline(always)]
    pub fn bna(&mut self) -> BNA_W {
        BNA_W { w: self }
    }
    #[doc = "Bit 11 - Packet dropped status"]
    #[inline(always)]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W {
        PKTDRPSTS_W { w: self }
    }
    #[doc = "Bit 12 - Babble error interrupt"]
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W {
        BERR_W { w: self }
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W {
        NAK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG device endpoint-6 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint6](index.html) module"]
pub struct DIEPINT6_SPEC;
impl crate::RegisterSpec for DIEPINT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepint6::R](R) reader structure"]
impl crate::Readable for DIEPINT6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepint6::W](W) writer structure"]
impl crate::Writable for DIEPINT6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEPINT6 to value 0"]
impl crate::Resettable for DIEPINT6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
