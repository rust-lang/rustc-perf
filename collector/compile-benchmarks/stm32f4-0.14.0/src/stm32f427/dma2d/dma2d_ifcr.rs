#[doc = "Register `DMA2D_IFCR` reader"]
pub struct R(crate::R<DMA2D_IFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA2D_IFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA2D_IFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA2D_IFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA2D_IFCR` writer"]
pub struct W(crate::W<DMA2D_IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA2D_IFCR_SPEC>;
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
impl From<crate::W<DMA2D_IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA2D_IFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTEIF` reader - Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register"]
pub struct CTEIF_R(crate::FieldReader<bool, bool>);
impl CTEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTEIF` writer - Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register"]
pub struct CTEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF_W<'a> {
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
#[doc = "Field `CTCIF` reader - Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register"]
pub struct CTCIF_R(crate::FieldReader<bool, bool>);
impl CTCIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTCIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCIF` writer - Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register"]
pub struct CTCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF_W<'a> {
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
#[doc = "Field `CTWIF` reader - Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register"]
pub struct CTWIF_R(crate::FieldReader<bool, bool>);
impl CTWIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTWIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTWIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTWIF` writer - Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register"]
pub struct CTWIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTWIF_W<'a> {
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
#[doc = "Field `CAECIF` reader - Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register"]
pub struct CAECIF_R(crate::FieldReader<bool, bool>);
impl CAECIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAECIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAECIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAECIF` writer - Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register"]
pub struct CAECIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CAECIF_W<'a> {
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
#[doc = "Field `CCTCIF` reader - Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register"]
pub struct CCTCIF_R(crate::FieldReader<bool, bool>);
impl CCTCIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCTCIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCTCIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCTCIF` writer - Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register"]
pub struct CCTCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF_W<'a> {
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
#[doc = "Field `CCEIF` reader - Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register"]
pub struct CCEIF_R(crate::FieldReader<bool, bool>);
impl CCEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCEIF` writer - Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register"]
pub struct CCEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CCEIF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn cteif(&self) -> CTEIF_R {
        CTEIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn ctcif(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn ctwif(&self) -> CTWIF_R {
        CTWIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn caecif(&self) -> CAECIF_R {
        CAECIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn cctcif(&self) -> CCTCIF_R {
        CCTCIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn cceif(&self) -> CCEIF_R {
        CCEIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn cteif(&mut self) -> CTEIF_W {
        CTEIF_W { w: self }
    }
    #[doc = "Bit 1 - Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn ctcif(&mut self) -> CTCIF_W {
        CTCIF_W { w: self }
    }
    #[doc = "Bit 2 - Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn ctwif(&mut self) -> CTWIF_W {
        CTWIF_W { w: self }
    }
    #[doc = "Bit 3 - Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn caecif(&mut self) -> CAECIF_W {
        CAECIF_W { w: self }
    }
    #[doc = "Bit 4 - Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn cctcif(&mut self) -> CCTCIF_W {
        CCTCIF_W { w: self }
    }
    #[doc = "Bit 5 - Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register"]
    #[inline(always)]
    pub fn cceif(&mut self) -> CCEIF_W {
        CCEIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA2D interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2d_ifcr](index.html) module"]
pub struct DMA2D_IFCR_SPEC;
impl crate::RegisterSpec for DMA2D_IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma2d_ifcr::R](R) reader structure"]
impl crate::Readable for DMA2D_IFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma2d_ifcr::W](W) writer structure"]
impl crate::Writable for DMA2D_IFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA2D_IFCR to value 0"]
impl crate::Resettable for DMA2D_IFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
