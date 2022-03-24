#[doc = "Register `IER1` reader"]
pub struct R(crate::R<IER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER1` writer"]
pub struct W(crate::W<IER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER1_SPEC>;
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
impl From<crate::W<IER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPRXEIE` reader - Generic Payload Receive Error Interrupt Enable"]
pub struct GPRXEIE_R(crate::FieldReader<bool, bool>);
impl GPRXEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPRXEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPRXEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPRXEIE` writer - Generic Payload Receive Error Interrupt Enable"]
pub struct GPRXEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPRXEIE_W<'a> {
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
#[doc = "Field `GPRDEIE` reader - Generic Payload Read Error Interrupt Enable"]
pub struct GPRDEIE_R(crate::FieldReader<bool, bool>);
impl GPRDEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPRDEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPRDEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPRDEIE` writer - Generic Payload Read Error Interrupt Enable"]
pub struct GPRDEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPRDEIE_W<'a> {
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
#[doc = "Field `GPTXEIE` reader - Generic Payload Transmit Error Interrupt Enable"]
pub struct GPTXEIE_R(crate::FieldReader<bool, bool>);
impl GPTXEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPTXEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPTXEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPTXEIE` writer - Generic Payload Transmit Error Interrupt Enable"]
pub struct GPTXEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTXEIE_W<'a> {
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
#[doc = "Field `GPWREIE` reader - Generic Payload Write Error Interrupt Enable"]
pub struct GPWREIE_R(crate::FieldReader<bool, bool>);
impl GPWREIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPWREIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPWREIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPWREIE` writer - Generic Payload Write Error Interrupt Enable"]
pub struct GPWREIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWREIE_W<'a> {
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
#[doc = "Field `GCWREIE` reader - Generic Command Write Error Interrupt Enable"]
pub struct GCWREIE_R(crate::FieldReader<bool, bool>);
impl GCWREIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GCWREIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCWREIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCWREIE` writer - Generic Command Write Error Interrupt Enable"]
pub struct GCWREIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GCWREIE_W<'a> {
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
#[doc = "Field `LPWREIE` reader - LTDC Payload Write Error Interrupt Enable"]
pub struct LPWREIE_R(crate::FieldReader<bool, bool>);
impl LPWREIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPWREIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPWREIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPWREIE` writer - LTDC Payload Write Error Interrupt Enable"]
pub struct LPWREIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPWREIE_W<'a> {
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
#[doc = "Field `EOTPEIE` reader - EoTp Error Interrupt Enable"]
pub struct EOTPEIE_R(crate::FieldReader<bool, bool>);
impl EOTPEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOTPEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOTPEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOTPEIE` writer - EoTp Error Interrupt Enable"]
pub struct EOTPEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOTPEIE_W<'a> {
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
#[doc = "Field `PSEIE` reader - Packet Size Error Interrupt Enable"]
pub struct PSEIE_R(crate::FieldReader<bool, bool>);
impl PSEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSEIE` writer - Packet Size Error Interrupt Enable"]
pub struct PSEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEIE_W<'a> {
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
#[doc = "Field `CRCEIE` reader - CRC Error Interrupt Enable"]
pub struct CRCEIE_R(crate::FieldReader<bool, bool>);
impl CRCEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCEIE` writer - CRC Error Interrupt Enable"]
pub struct CRCEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEIE_W<'a> {
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
#[doc = "Field `ECCMEIE` reader - ECC Multi-bit Error Interrupt Enable"]
pub struct ECCMEIE_R(crate::FieldReader<bool, bool>);
impl ECCMEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCMEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCMEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCMEIE` writer - ECC Multi-bit Error Interrupt Enable"]
pub struct ECCMEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCMEIE_W<'a> {
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
#[doc = "Field `ECCSEIE` reader - ECC Single-bit Error Interrupt Enable"]
pub struct ECCSEIE_R(crate::FieldReader<bool, bool>);
impl ECCSEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCSEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCSEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCSEIE` writer - ECC Single-bit Error Interrupt Enable"]
pub struct ECCSEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCSEIE_W<'a> {
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
#[doc = "Field `TOLPRXIE` reader - Timeout Low-Power Reception Interrupt Enable"]
pub struct TOLPRXIE_R(crate::FieldReader<bool, bool>);
impl TOLPRXIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOLPRXIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOLPRXIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOLPRXIE` writer - Timeout Low-Power Reception Interrupt Enable"]
pub struct TOLPRXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOLPRXIE_W<'a> {
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
#[doc = "Field `TOHSTXIE` reader - Timeout High-Speed Transmission Interrupt Enable"]
pub struct TOHSTXIE_R(crate::FieldReader<bool, bool>);
impl TOHSTXIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOHSTXIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOHSTXIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOHSTXIE` writer - Timeout High-Speed Transmission Interrupt Enable"]
pub struct TOHSTXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOHSTXIE_W<'a> {
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
    #[doc = "Bit 12 - Generic Payload Receive Error Interrupt Enable"]
    #[inline(always)]
    pub fn gprxeie(&self) -> GPRXEIE_R {
        GPRXEIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Generic Payload Read Error Interrupt Enable"]
    #[inline(always)]
    pub fn gprdeie(&self) -> GPRDEIE_R {
        GPRDEIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Generic Payload Transmit Error Interrupt Enable"]
    #[inline(always)]
    pub fn gptxeie(&self) -> GPTXEIE_R {
        GPTXEIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Generic Payload Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn gpwreie(&self) -> GPWREIE_R {
        GPWREIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Generic Command Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn gcwreie(&self) -> GCWREIE_R {
        GCWREIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LTDC Payload Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn lpwreie(&self) -> LPWREIE_R {
        LPWREIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EoTp Error Interrupt Enable"]
    #[inline(always)]
    pub fn eotpeie(&self) -> EOTPEIE_R {
        EOTPEIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Packet Size Error Interrupt Enable"]
    #[inline(always)]
    pub fn pseie(&self) -> PSEIE_R {
        PSEIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ECC Multi-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn eccmeie(&self) -> ECCMEIE_R {
        ECCMEIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ECC Single-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn eccseie(&self) -> ECCSEIE_R {
        ECCSEIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timeout Low-Power Reception Interrupt Enable"]
    #[inline(always)]
    pub fn tolprxie(&self) -> TOLPRXIE_R {
        TOLPRXIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Timeout High-Speed Transmission Interrupt Enable"]
    #[inline(always)]
    pub fn tohstxie(&self) -> TOHSTXIE_R {
        TOHSTXIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Generic Payload Receive Error Interrupt Enable"]
    #[inline(always)]
    pub fn gprxeie(&mut self) -> GPRXEIE_W {
        GPRXEIE_W { w: self }
    }
    #[doc = "Bit 11 - Generic Payload Read Error Interrupt Enable"]
    #[inline(always)]
    pub fn gprdeie(&mut self) -> GPRDEIE_W {
        GPRDEIE_W { w: self }
    }
    #[doc = "Bit 10 - Generic Payload Transmit Error Interrupt Enable"]
    #[inline(always)]
    pub fn gptxeie(&mut self) -> GPTXEIE_W {
        GPTXEIE_W { w: self }
    }
    #[doc = "Bit 9 - Generic Payload Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn gpwreie(&mut self) -> GPWREIE_W {
        GPWREIE_W { w: self }
    }
    #[doc = "Bit 8 - Generic Command Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn gcwreie(&mut self) -> GCWREIE_W {
        GCWREIE_W { w: self }
    }
    #[doc = "Bit 7 - LTDC Payload Write Error Interrupt Enable"]
    #[inline(always)]
    pub fn lpwreie(&mut self) -> LPWREIE_W {
        LPWREIE_W { w: self }
    }
    #[doc = "Bit 6 - EoTp Error Interrupt Enable"]
    #[inline(always)]
    pub fn eotpeie(&mut self) -> EOTPEIE_W {
        EOTPEIE_W { w: self }
    }
    #[doc = "Bit 5 - Packet Size Error Interrupt Enable"]
    #[inline(always)]
    pub fn pseie(&mut self) -> PSEIE_W {
        PSEIE_W { w: self }
    }
    #[doc = "Bit 4 - CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn crceie(&mut self) -> CRCEIE_W {
        CRCEIE_W { w: self }
    }
    #[doc = "Bit 3 - ECC Multi-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn eccmeie(&mut self) -> ECCMEIE_W {
        ECCMEIE_W { w: self }
    }
    #[doc = "Bit 2 - ECC Single-bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn eccseie(&mut self) -> ECCSEIE_W {
        ECCSEIE_W { w: self }
    }
    #[doc = "Bit 1 - Timeout Low-Power Reception Interrupt Enable"]
    #[inline(always)]
    pub fn tolprxie(&mut self) -> TOLPRXIE_W {
        TOLPRXIE_W { w: self }
    }
    #[doc = "Bit 0 - Timeout High-Speed Transmission Interrupt Enable"]
    #[inline(always)]
    pub fn tohstxie(&mut self) -> TOHSTXIE_W {
        TOHSTXIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Interrupt Enable Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier1](index.html) module"]
pub struct IER1_SPEC;
impl crate::RegisterSpec for IER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier1::R](R) reader structure"]
impl crate::Readable for IER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier1::W](W) writer structure"]
impl crate::Writable for IER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER1 to value 0"]
impl crate::Resettable for IER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
