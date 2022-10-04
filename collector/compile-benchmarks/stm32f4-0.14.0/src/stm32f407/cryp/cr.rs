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
#[doc = "Field `ALGODIR` reader - Algorithm direction"]
pub struct ALGODIR_R(crate::FieldReader<bool, bool>);
impl ALGODIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALGODIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALGODIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALGODIR` writer - Algorithm direction"]
pub struct ALGODIR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALGODIR_W<'a> {
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
#[doc = "Field `ALGOMODE0` reader - Algorithm mode"]
pub struct ALGOMODE0_R(crate::FieldReader<u8, u8>);
impl ALGOMODE0_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALGOMODE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALGOMODE0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALGOMODE0` writer - Algorithm mode"]
pub struct ALGOMODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ALGOMODE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `DATATYPE` reader - Data type selection"]
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
#[doc = "Field `DATATYPE` writer - Data type selection"]
pub struct DATATYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATATYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `KEYSIZE` reader - Key size selection (AES mode only)"]
pub struct KEYSIZE_R(crate::FieldReader<u8, u8>);
impl KEYSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEYSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEYSIZE` writer - Key size selection (AES mode only)"]
pub struct KEYSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `FFLUSH` writer - FIFO flush"]
pub struct FFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> FFLUSH_W<'a> {
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
#[doc = "Field `CRYPEN` reader - Cryptographic processor enable"]
pub struct CRYPEN_R(crate::FieldReader<bool, bool>);
impl CRYPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRYPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYPEN` writer - Cryptographic processor enable"]
pub struct CRYPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPEN_W<'a> {
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
#[doc = "Field `GCM_CCMPH` reader - GCM_CCMPH"]
pub struct GCM_CCMPH_R(crate::FieldReader<u8, u8>);
impl GCM_CCMPH_R {
    pub(crate) fn new(bits: u8) -> Self {
        GCM_CCMPH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCM_CCMPH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCM_CCMPH` writer - GCM_CCMPH"]
pub struct GCM_CCMPH_W<'a> {
    w: &'a mut W,
}
impl<'a> GCM_CCMPH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `ALGOMODE3` reader - ALGOMODE"]
pub struct ALGOMODE3_R(crate::FieldReader<bool, bool>);
impl ALGOMODE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALGOMODE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALGOMODE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALGOMODE3` writer - ALGOMODE"]
pub struct ALGOMODE3_W<'a> {
    w: &'a mut W,
}
impl<'a> ALGOMODE3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Algorithm direction"]
    #[inline(always)]
    pub fn algodir(&self) -> ALGODIR_R {
        ALGODIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Algorithm mode"]
    #[inline(always)]
    pub fn algomode0(&self) -> ALGOMODE0_R {
        ALGOMODE0_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Data type selection"]
    #[inline(always)]
    pub fn datatype(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Key size selection (AES mode only)"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Cryptographic processor enable"]
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - GCM_CCMPH"]
    #[inline(always)]
    pub fn gcm_ccmph(&self) -> GCM_CCMPH_R {
        GCM_CCMPH_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 19 - ALGOMODE"]
    #[inline(always)]
    pub fn algomode3(&self) -> ALGOMODE3_R {
        ALGOMODE3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Algorithm direction"]
    #[inline(always)]
    pub fn algodir(&mut self) -> ALGODIR_W {
        ALGODIR_W { w: self }
    }
    #[doc = "Bits 3:5 - Algorithm mode"]
    #[inline(always)]
    pub fn algomode0(&mut self) -> ALGOMODE0_W {
        ALGOMODE0_W { w: self }
    }
    #[doc = "Bits 6:7 - Data type selection"]
    #[inline(always)]
    pub fn datatype(&mut self) -> DATATYPE_W {
        DATATYPE_W { w: self }
    }
    #[doc = "Bits 8:9 - Key size selection (AES mode only)"]
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W {
        KEYSIZE_W { w: self }
    }
    #[doc = "Bit 14 - FIFO flush"]
    #[inline(always)]
    pub fn fflush(&mut self) -> FFLUSH_W {
        FFLUSH_W { w: self }
    }
    #[doc = "Bit 15 - Cryptographic processor enable"]
    #[inline(always)]
    pub fn crypen(&mut self) -> CRYPEN_W {
        CRYPEN_W { w: self }
    }
    #[doc = "Bits 16:17 - GCM_CCMPH"]
    #[inline(always)]
    pub fn gcm_ccmph(&mut self) -> GCM_CCMPH_W {
        GCM_CCMPH_W { w: self }
    }
    #[doc = "Bit 19 - ALGOMODE"]
    #[inline(always)]
    pub fn algomode3(&mut self) -> ALGOMODE3_W {
        ALGOMODE3_W { w: self }
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
