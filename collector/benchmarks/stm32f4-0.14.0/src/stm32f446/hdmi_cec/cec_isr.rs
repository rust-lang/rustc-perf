#[doc = "Register `CEC_ISR` reader"]
pub struct R(crate::R<CEC_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEC_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEC_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEC_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CEC_ISR` writer"]
pub struct W(crate::W<CEC_ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEC_ISR_SPEC>;
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
impl From<crate::W<CEC_ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEC_ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXACKE` reader - Tx-Missing Acknowledge Error"]
pub struct TXACKE_R(crate::FieldReader<bool, bool>);
impl TXACKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXACKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXACKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXACKE` writer - Tx-Missing Acknowledge Error"]
pub struct TXACKE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXACKE_W<'a> {
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
#[doc = "Field `TXERR` reader - Tx-Error"]
pub struct TXERR_R(crate::FieldReader<bool, bool>);
impl TXERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXERR` writer - Tx-Error"]
pub struct TXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERR_W<'a> {
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
#[doc = "Field `TXUDR` reader - Tx-Buffer Underrun"]
pub struct TXUDR_R(crate::FieldReader<bool, bool>);
impl TXUDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUDR` writer - Tx-Buffer Underrun"]
pub struct TXUDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUDR_W<'a> {
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
#[doc = "Field `TXEND` reader - End of Transmission"]
pub struct TXEND_R(crate::FieldReader<bool, bool>);
impl TXEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEND` writer - End of Transmission"]
pub struct TXEND_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEND_W<'a> {
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
#[doc = "Field `TXBR` reader - Tx-Byte Request"]
pub struct TXBR_R(crate::FieldReader<bool, bool>);
impl TXBR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBR` writer - Tx-Byte Request"]
pub struct TXBR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBR_W<'a> {
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
#[doc = "Field `ARBLST` reader - Arbitration Lost"]
pub struct ARBLST_R(crate::FieldReader<bool, bool>);
impl ARBLST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARBLST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARBLST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBLST` writer - Arbitration Lost"]
pub struct ARBLST_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBLST_W<'a> {
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
#[doc = "Field `RXACKE` reader - Rx-Missing Acknowledge"]
pub struct RXACKE_R(crate::FieldReader<bool, bool>);
impl RXACKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXACKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXACKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXACKE` writer - Rx-Missing Acknowledge"]
pub struct RXACKE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXACKE_W<'a> {
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
#[doc = "Field `LBPE` reader - Rx-Long Bit Period Error"]
pub struct LBPE_R(crate::FieldReader<bool, bool>);
impl LBPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LBPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LBPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LBPE` writer - Rx-Long Bit Period Error"]
pub struct LBPE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBPE_W<'a> {
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
#[doc = "Field `SBPE` reader - Rx-Short Bit Period Error"]
pub struct SBPE_R(crate::FieldReader<bool, bool>);
impl SBPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBPE` writer - Rx-Short Bit Period Error"]
pub struct SBPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SBPE_W<'a> {
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
#[doc = "Field `BRE` reader - Rx-Bit Rising Error"]
pub struct BRE_R(crate::FieldReader<bool, bool>);
impl BRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRE` writer - Rx-Bit Rising Error"]
pub struct BRE_W<'a> {
    w: &'a mut W,
}
impl<'a> BRE_W<'a> {
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
#[doc = "Field `RXOVR` reader - Rx-Overrun"]
pub struct RXOVR_R(crate::FieldReader<bool, bool>);
impl RXOVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVR` writer - Rx-Overrun"]
pub struct RXOVR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVR_W<'a> {
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
#[doc = "Field `RXEND` reader - End Of Reception"]
pub struct RXEND_R(crate::FieldReader<bool, bool>);
impl RXEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXEND` writer - End Of Reception"]
pub struct RXEND_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEND_W<'a> {
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
#[doc = "Field `RXBR` reader - Rx-Byte Received"]
pub struct RXBR_R(crate::FieldReader<bool, bool>);
impl RXBR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBR` writer - Rx-Byte Received"]
pub struct RXBR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBR_W<'a> {
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
impl R {
    #[doc = "Bit 12 - Tx-Missing Acknowledge Error"]
    #[inline(always)]
    pub fn txacke(&self) -> TXACKE_R {
        TXACKE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx-Error"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Tx-Buffer Underrun"]
    #[inline(always)]
    pub fn txudr(&self) -> TXUDR_R {
        TXUDR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - End of Transmission"]
    #[inline(always)]
    pub fn txend(&self) -> TXEND_R {
        TXEND_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Tx-Byte Request"]
    #[inline(always)]
    pub fn txbr(&self) -> TXBR_R {
        TXBR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Arbitration Lost"]
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx-Missing Acknowledge"]
    #[inline(always)]
    pub fn rxacke(&self) -> RXACKE_R {
        RXACKE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rx-Long Bit Period Error"]
    #[inline(always)]
    pub fn lbpe(&self) -> LBPE_R {
        LBPE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx-Short Bit Period Error"]
    #[inline(always)]
    pub fn sbpe(&self) -> SBPE_R {
        SBPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rx-Bit Rising Error"]
    #[inline(always)]
    pub fn bre(&self) -> BRE_R {
        BRE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx-Overrun"]
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - End Of Reception"]
    #[inline(always)]
    pub fn rxend(&self) -> RXEND_R {
        RXEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Rx-Byte Received"]
    #[inline(always)]
    pub fn rxbr(&self) -> RXBR_R {
        RXBR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Tx-Missing Acknowledge Error"]
    #[inline(always)]
    pub fn txacke(&mut self) -> TXACKE_W {
        TXACKE_W { w: self }
    }
    #[doc = "Bit 11 - Tx-Error"]
    #[inline(always)]
    pub fn txerr(&mut self) -> TXERR_W {
        TXERR_W { w: self }
    }
    #[doc = "Bit 10 - Tx-Buffer Underrun"]
    #[inline(always)]
    pub fn txudr(&mut self) -> TXUDR_W {
        TXUDR_W { w: self }
    }
    #[doc = "Bit 9 - End of Transmission"]
    #[inline(always)]
    pub fn txend(&mut self) -> TXEND_W {
        TXEND_W { w: self }
    }
    #[doc = "Bit 8 - Tx-Byte Request"]
    #[inline(always)]
    pub fn txbr(&mut self) -> TXBR_W {
        TXBR_W { w: self }
    }
    #[doc = "Bit 7 - Arbitration Lost"]
    #[inline(always)]
    pub fn arblst(&mut self) -> ARBLST_W {
        ARBLST_W { w: self }
    }
    #[doc = "Bit 6 - Rx-Missing Acknowledge"]
    #[inline(always)]
    pub fn rxacke(&mut self) -> RXACKE_W {
        RXACKE_W { w: self }
    }
    #[doc = "Bit 5 - Rx-Long Bit Period Error"]
    #[inline(always)]
    pub fn lbpe(&mut self) -> LBPE_W {
        LBPE_W { w: self }
    }
    #[doc = "Bit 4 - Rx-Short Bit Period Error"]
    #[inline(always)]
    pub fn sbpe(&mut self) -> SBPE_W {
        SBPE_W { w: self }
    }
    #[doc = "Bit 3 - Rx-Bit Rising Error"]
    #[inline(always)]
    pub fn bre(&mut self) -> BRE_W {
        BRE_W { w: self }
    }
    #[doc = "Bit 2 - Rx-Overrun"]
    #[inline(always)]
    pub fn rxovr(&mut self) -> RXOVR_W {
        RXOVR_W { w: self }
    }
    #[doc = "Bit 1 - End Of Reception"]
    #[inline(always)]
    pub fn rxend(&mut self) -> RXEND_W {
        RXEND_W { w: self }
    }
    #[doc = "Bit 0 - Rx-Byte Received"]
    #[inline(always)]
    pub fn rxbr(&mut self) -> RXBR_W {
        RXBR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CEC Interrupt and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_isr](index.html) module"]
pub struct CEC_ISR_SPEC;
impl crate::RegisterSpec for CEC_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cec_isr::R](R) reader structure"]
impl crate::Readable for CEC_ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cec_isr::W](W) writer structure"]
impl crate::Writable for CEC_ISR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CEC_ISR to value 0"]
impl crate::Resettable for CEC_ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
