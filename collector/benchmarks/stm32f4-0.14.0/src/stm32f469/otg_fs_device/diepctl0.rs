#[doc = "Register `DIEPCTL0` reader"]
pub struct R(crate::R<DIEPCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPCTL0` writer"]
pub struct W(crate::W<DIEPCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPCTL0_SPEC>;
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
impl From<crate::W<DIEPCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPSIZ` reader - Maximum packet size"]
pub struct MPSIZ_R(crate::FieldReader<u8, u8>);
impl MPSIZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        MPSIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPSIZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPSIZ` writer - Maximum packet size"]
pub struct MPSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> MPSIZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `USBAEP` reader - USB active endpoint"]
pub struct USBAEP_R(crate::FieldReader<bool, bool>);
impl USBAEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBAEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBAEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAKSTS` reader - NAK status"]
pub struct NAKSTS_R(crate::FieldReader<bool, bool>);
impl NAKSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAKSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTYP` reader - Endpoint type"]
pub struct EPTYP_R(crate::FieldReader<u8, u8>);
impl EPTYP_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPTYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTYP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL` reader - STALL handshake"]
pub struct STALL_R(crate::FieldReader<bool, bool>);
impl STALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALL` writer - STALL handshake"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `TXFNUM` reader - TxFIFO number"]
pub struct TXFNUM_R(crate::FieldReader<u8, u8>);
impl TXFNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXFNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFNUM` writer - TxFIFO number"]
pub struct TXFNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | ((value as u32 & 0x0f) << 22);
        self.w
    }
}
#[doc = "Field `CNAK` writer - Clear NAK"]
pub struct CNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CNAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `SNAK` writer - Set NAK"]
pub struct SNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> SNAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `EPDIS` reader - Endpoint disable"]
pub struct EPDIS_R(crate::FieldReader<bool, bool>);
impl EPDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPENA` reader - Endpoint enable"]
pub struct EPENA_R(crate::FieldReader<bool, bool>);
impl EPENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Maximum packet size"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 15 - USB active endpoint"]
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17 - NAK status"]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:25 - TxFIFO number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Maximum packet size"]
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MPSIZ_W {
        MPSIZ_W { w: self }
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    #[doc = "Bits 22:25 - TxFIFO number"]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W {
        TXFNUM_W { w: self }
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W {
        CNAK_W { w: self }
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W {
        SNAK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepctl0](index.html) module"]
pub struct DIEPCTL0_SPEC;
impl crate::RegisterSpec for DIEPCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepctl0::R](R) reader structure"]
impl crate::Readable for DIEPCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepctl0::W](W) writer structure"]
impl crate::Writable for DIEPCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEPCTL0 to value 0"]
impl crate::Resettable for DIEPCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
