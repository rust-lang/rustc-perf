#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAOUTEN` reader - Enable DMA management of data output phase"]
pub struct DMAOUTEN_R(crate::FieldReader<bool, bool>);
impl DMAOUTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAOUTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAOUTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAOUTEN` writer - Enable DMA management of data output phase"]
pub struct DMAOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAOUTEN_W<'a> {
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
#[doc = "Field `DMAINEN` reader - Enable DMA management of data input phase"]
pub struct DMAINEN_R(crate::FieldReader<bool, bool>);
impl DMAINEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAINEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAINEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAINEN` writer - Enable DMA management of data input phase"]
pub struct DMAINEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAINEN_W<'a> {
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
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub struct ERRIE_R(crate::FieldReader<bool, bool>);
impl ERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
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
#[doc = "Field `CCFIE` reader - CCF flag interrupt enable"]
pub struct CCFIE_R(crate::FieldReader<bool, bool>);
impl CCFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCFIE` writer - CCF flag interrupt enable"]
pub struct CCFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCFIE_W<'a> {
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
#[doc = "Field `ERRC` reader - Error clear"]
pub struct ERRC_R(crate::FieldReader<bool, bool>);
impl ERRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRC` writer - Error clear"]
pub struct ERRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRC_W<'a> {
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
#[doc = "Field `CCFC` reader - Computation Complete Flag Clear"]
pub struct CCFC_R(crate::FieldReader<bool, bool>);
impl CCFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCFC` writer - Computation Complete Flag Clear"]
pub struct CCFC_W<'a> {
    w: &'a mut W,
}
impl<'a> CCFC_W<'a> {
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
#[doc = "Field `CHMOD` reader - AES chaining mode"]
pub struct CHMOD_R(crate::FieldReader<u8, u8>);
impl CHMOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHMOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMOD` writer - AES chaining mode"]
pub struct CHMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `MODE` reader - AES operating mode"]
pub struct MODE_R(crate::FieldReader<u8, u8>);
impl MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - AES operating mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `DATATYPE` reader - Data type selection (for data in and data out to/from the cryptographic block)"]
pub struct DATATYPE_R(crate::FieldReader<u8, u8>);
impl DATATYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATATYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATATYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATATYPE` writer - Data type selection (for data in and data out to/from the cryptographic block)"]
pub struct DATATYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATATYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `EN` reader - AES enable"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - AES enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
    #[doc = "Bit 12 - Enable DMA management of data output phase"]
    #[inline(always)]
    pub fn dmaouten(&self) -> DMAOUTEN_R {
        DMAOUTEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable DMA management of data input phase"]
    #[inline(always)]
    pub fn dmainen(&self) -> DMAINEN_R {
        DMAINEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CCF flag interrupt enable"]
    #[inline(always)]
    pub fn ccfie(&self) -> CCFIE_R {
        CCFIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Error clear"]
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Computation Complete Flag Clear"]
    #[inline(always)]
    pub fn ccfc(&self) -> CCFC_R {
        CCFC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - AES chaining mode"]
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - AES operating mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)"]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - AES enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Enable DMA management of data output phase"]
    #[inline(always)]
    pub fn dmaouten(&mut self) -> DMAOUTEN_W {
        DMAOUTEN_W { w: self }
    }
    #[doc = "Bit 11 - Enable DMA management of data input phase"]
    #[inline(always)]
    pub fn dmainen(&mut self) -> DMAINEN_W {
        DMAINEN_W { w: self }
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    #[doc = "Bit 9 - CCF flag interrupt enable"]
    #[inline(always)]
    pub fn ccfie(&mut self) -> CCFIE_W {
        CCFIE_W { w: self }
    }
    #[doc = "Bit 8 - Error clear"]
    #[inline(always)]
    pub fn errc(&mut self) -> ERRC_W {
        ERRC_W { w: self }
    }
    #[doc = "Bit 7 - Computation Complete Flag Clear"]
    #[inline(always)]
    pub fn ccfc(&mut self) -> CCFC_W {
        CCFC_W { w: self }
    }
    #[doc = "Bits 5:6 - AES chaining mode"]
    #[inline(always)]
    pub fn chmod(&mut self) -> CHMOD_W {
        CHMOD_W { w: self }
    }
    #[doc = "Bits 3:4 - AES operating mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)"]
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W {
        DATATYPE_W { w: self }
    }
    #[doc = "Bit 0 - AES enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
