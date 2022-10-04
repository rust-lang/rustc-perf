#[doc = "Register `DIEPINT0` reader"]
pub struct R(crate::R<DIEPINT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPINT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPINT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPINT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPINT0` writer"]
pub struct W(crate::W<DIEPINT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPINT0_SPEC>;
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
impl From<crate::W<DIEPINT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPINT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFE` reader - TXFE"]
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
#[doc = "Field `INEPNE` reader - INEPNE"]
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
#[doc = "Field `INEPNE` writer - INEPNE"]
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
#[doc = "Field `ITTXFE` reader - ITTXFE"]
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
#[doc = "Field `ITTXFE` writer - ITTXFE"]
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
#[doc = "Field `TOC` reader - TOC"]
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
#[doc = "Field `TOC` writer - TOC"]
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
#[doc = "Field `EPDISD` reader - EPDISD"]
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
#[doc = "Field `EPDISD` writer - EPDISD"]
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
#[doc = "Field `XFRC` reader - XFRC"]
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
#[doc = "Field `XFRC` writer - XFRC"]
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
impl R {
    #[doc = "Bit 7 - TXFE"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - INEPNE"]
    #[inline(always)]
    pub fn inepne(&self) -> INEPNE_R {
        INEPNE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ITTXFE"]
    #[inline(always)]
    pub fn ittxfe(&self) -> ITTXFE_R {
        ITTXFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TOC"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - INEPNE"]
    #[inline(always)]
    pub fn inepne(&mut self) -> INEPNE_W {
        INEPNE_W { w: self }
    }
    #[doc = "Bit 4 - ITTXFE"]
    #[inline(always)]
    pub fn ittxfe(&mut self) -> ITTXFE_W {
        ITTXFE_W { w: self }
    }
    #[doc = "Bit 3 - TOC"]
    #[inline(always)]
    pub fn toc(&mut self) -> TOC_W {
        TOC_W { w: self }
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W {
        EPDISD_W { w: self }
    }
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W {
        XFRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device endpoint-x interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint0](index.html) module"]
pub struct DIEPINT0_SPEC;
impl crate::RegisterSpec for DIEPINT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepint0::R](R) reader structure"]
impl crate::Readable for DIEPINT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepint0::W](W) writer structure"]
impl crate::Writable for DIEPINT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEPINT0 to value 0x80"]
impl crate::Resettable for DIEPINT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
