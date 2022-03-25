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
#[doc = "Field `MODE` reader - DMA2D mode"]
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
#[doc = "Field `MODE` writer - DMA2D mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `CEIE` reader - Configuration Error Interrupt Enable"]
pub struct CEIE_R(crate::FieldReader<bool, bool>);
impl CEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEIE` writer - Configuration Error Interrupt Enable"]
pub struct CEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIE_W<'a> {
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
#[doc = "Field `CTCIE` reader - CLUT transfer complete interrupt enable"]
pub struct CTCIE_R(crate::FieldReader<bool, bool>);
impl CTCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCIE` writer - CLUT transfer complete interrupt enable"]
pub struct CTCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIE_W<'a> {
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
#[doc = "Field `CAEIE` reader - CLUT access error interrupt enable"]
pub struct CAEIE_R(crate::FieldReader<bool, bool>);
impl CAEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAEIE` writer - CLUT access error interrupt enable"]
pub struct CAEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAEIE_W<'a> {
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
#[doc = "Field `TWIE` reader - Transfer watermark interrupt enable"]
pub struct TWIE_R(crate::FieldReader<bool, bool>);
impl TWIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TWIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWIE` writer - Transfer watermark interrupt enable"]
pub struct TWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TWIE_W<'a> {
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
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable"]
pub struct TCIE_R(crate::FieldReader<bool, bool>);
impl TCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable"]
pub struct TCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE_W<'a> {
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
#[doc = "Field `TEIE` reader - Transfer error interrupt enable"]
pub struct TEIE_R(crate::FieldReader<bool, bool>);
impl TEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIE` writer - Transfer error interrupt enable"]
pub struct TEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE_W<'a> {
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
#[doc = "Field `ABORT` reader - Abort"]
pub struct ABORT_R(crate::FieldReader<bool, bool>);
impl ABORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABORT` writer - Abort"]
pub struct ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_W<'a> {
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
#[doc = "Field `SUSP` reader - Suspend"]
pub struct SUSP_R(crate::FieldReader<bool, bool>);
impl SUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSP` writer - Suspend"]
pub struct SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_W<'a> {
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
#[doc = "Field `START` reader - Start"]
pub struct START_R(crate::FieldReader<bool, bool>);
impl START_R {
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - Start"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
    #[doc = "Bits 16:17 - DMA2D mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 13 - Configuration Error Interrupt Enable"]
    #[inline(always)]
    pub fn ceie(&self) -> CEIE_R {
        CEIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CLUT transfer complete interrupt enable"]
    #[inline(always)]
    pub fn ctcie(&self) -> CTCIE_R {
        CTCIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CLUT access error interrupt enable"]
    #[inline(always)]
    pub fn caeie(&self) -> CAEIE_R {
        CAEIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transfer watermark interrupt enable"]
    #[inline(always)]
    pub fn twie(&self) -> TWIE_R {
        TWIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Abort"]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Suspend"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Start"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - DMA2D mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 13 - Configuration Error Interrupt Enable"]
    #[inline(always)]
    pub fn ceie(&mut self) -> CEIE_W {
        CEIE_W { w: self }
    }
    #[doc = "Bit 12 - CLUT transfer complete interrupt enable"]
    #[inline(always)]
    pub fn ctcie(&mut self) -> CTCIE_W {
        CTCIE_W { w: self }
    }
    #[doc = "Bit 11 - CLUT access error interrupt enable"]
    #[inline(always)]
    pub fn caeie(&mut self) -> CAEIE_W {
        CAEIE_W { w: self }
    }
    #[doc = "Bit 10 - Transfer watermark interrupt enable"]
    #[inline(always)]
    pub fn twie(&mut self) -> TWIE_W {
        TWIE_W { w: self }
    }
    #[doc = "Bit 9 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W { w: self }
    }
    #[doc = "Bit 8 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W {
        TEIE_W { w: self }
    }
    #[doc = "Bit 2 - Abort"]
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W {
        ABORT_W { w: self }
    }
    #[doc = "Bit 1 - Suspend"]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W { w: self }
    }
    #[doc = "Bit 0 - Start"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
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
