#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AWDCH` reader - Analog watchdog channel selection"]
pub struct AWDCH_R(crate::FieldReader<u8, u8>);
impl AWDCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        AWDCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWDCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWDCH` writer - Analog watchdog channel selection"]
pub struct AWDCH_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `EXCH` reader - Extremes detector channel selection"]
pub struct EXCH_R(crate::FieldReader<u8, u8>);
impl EXCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXCH` writer - Extremes detector channel selection"]
pub struct EXCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CKABIE` reader - Clock absence interrupt enable"]
pub struct CKABIE_R(crate::FieldReader<bool, bool>);
impl CKABIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKABIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKABIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKABIE` writer - Clock absence interrupt enable"]
pub struct CKABIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKABIE_W<'a> {
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
#[doc = "Field `SCDIE` reader - Short-circuit detector interrupt enable"]
pub struct SCDIE_R(crate::FieldReader<bool, bool>);
impl SCDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCDIE` writer - Short-circuit detector interrupt enable"]
pub struct SCDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCDIE_W<'a> {
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
#[doc = "Field `AWDIE` reader - Analog watchdog interrupt enable"]
pub struct AWDIE_R(crate::FieldReader<bool, bool>);
impl AWDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWDIE` writer - Analog watchdog interrupt enable"]
pub struct AWDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDIE_W<'a> {
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
#[doc = "Field `ROVRIE` reader - Regular data overrun interrupt enable"]
pub struct ROVRIE_R(crate::FieldReader<bool, bool>);
impl ROVRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROVRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROVRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROVRIE` writer - Regular data overrun interrupt enable"]
pub struct ROVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROVRIE_W<'a> {
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
#[doc = "Field `JOVRIE` reader - Injected data overrun interrupt enable"]
pub struct JOVRIE_R(crate::FieldReader<bool, bool>);
impl JOVRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        JOVRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JOVRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JOVRIE` writer - Injected data overrun interrupt enable"]
pub struct JOVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> JOVRIE_W<'a> {
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
#[doc = "Field `REOCIE` reader - Regular end of conversion interrupt enable"]
pub struct REOCIE_R(crate::FieldReader<bool, bool>);
impl REOCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REOCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REOCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REOCIE` writer - Regular end of conversion interrupt enable"]
pub struct REOCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> REOCIE_W<'a> {
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
#[doc = "Field `JEOCIE` reader - Injected end of conversion interrupt enable"]
pub struct JEOCIE_R(crate::FieldReader<bool, bool>);
impl JEOCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        JEOCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEOCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEOCIE` writer - Injected end of conversion interrupt enable"]
pub struct JEOCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> JEOCIE_W<'a> {
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
    #[doc = "Bits 16:23 - Analog watchdog channel selection"]
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Extremes detector channel selection"]
    #[inline(always)]
    pub fn exch(&self) -> EXCH_R {
        EXCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 6 - Clock absence interrupt enable"]
    #[inline(always)]
    pub fn ckabie(&self) -> CKABIE_R {
        CKABIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Short-circuit detector interrupt enable"]
    #[inline(always)]
    pub fn scdie(&self) -> SCDIE_R {
        SCDIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Analog watchdog interrupt enable"]
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Regular data overrun interrupt enable"]
    #[inline(always)]
    pub fn rovrie(&self) -> ROVRIE_R {
        ROVRIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Injected data overrun interrupt enable"]
    #[inline(always)]
    pub fn jovrie(&self) -> JOVRIE_R {
        JOVRIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Regular end of conversion interrupt enable"]
    #[inline(always)]
    pub fn reocie(&self) -> REOCIE_R {
        REOCIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Injected end of conversion interrupt enable"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:23 - Analog watchdog channel selection"]
    #[inline(always)]
    pub fn awdch(&mut self) -> AWDCH_W {
        AWDCH_W { w: self }
    }
    #[doc = "Bits 8:15 - Extremes detector channel selection"]
    #[inline(always)]
    pub fn exch(&mut self) -> EXCH_W {
        EXCH_W { w: self }
    }
    #[doc = "Bit 6 - Clock absence interrupt enable"]
    #[inline(always)]
    pub fn ckabie(&mut self) -> CKABIE_W {
        CKABIE_W { w: self }
    }
    #[doc = "Bit 5 - Short-circuit detector interrupt enable"]
    #[inline(always)]
    pub fn scdie(&mut self) -> SCDIE_W {
        SCDIE_W { w: self }
    }
    #[doc = "Bit 4 - Analog watchdog interrupt enable"]
    #[inline(always)]
    pub fn awdie(&mut self) -> AWDIE_W {
        AWDIE_W { w: self }
    }
    #[doc = "Bit 3 - Regular data overrun interrupt enable"]
    #[inline(always)]
    pub fn rovrie(&mut self) -> ROVRIE_W {
        ROVRIE_W { w: self }
    }
    #[doc = "Bit 2 - Injected data overrun interrupt enable"]
    #[inline(always)]
    pub fn jovrie(&mut self) -> JOVRIE_W {
        JOVRIE_W { w: self }
    }
    #[doc = "Bit 1 - Regular end of conversion interrupt enable"]
    #[inline(always)]
    pub fn reocie(&mut self) -> REOCIE_W {
        REOCIE_W { w: self }
    }
    #[doc = "Bit 0 - Injected end of conversion interrupt enable"]
    #[inline(always)]
    pub fn jeocie(&mut self) -> JEOCIE_W {
        JEOCIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
