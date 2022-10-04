#[doc = "Register `DCR` reader"]
pub struct R(crate::R<DCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCR` writer"]
pub struct W(crate::W<DCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCR_SPEC>;
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
impl From<crate::W<DCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSIZE` reader - FLASH memory size"]
pub struct FSIZE_R(crate::FieldReader<u8, u8>);
impl FSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSIZE` writer - FLASH memory size"]
pub struct FSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `CSHT` reader - Chip select high time"]
pub struct CSHT_R(crate::FieldReader<u8, u8>);
impl CSHT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CSHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSHT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSHT` writer - Chip select high time"]
pub struct CSHT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSHT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `CKMODE` reader - Mode 0 / mode 3"]
pub struct CKMODE_R(crate::FieldReader<bool, bool>);
impl CKMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKMODE` writer - Mode 0 / mode 3"]
pub struct CKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKMODE_W<'a> {
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
    #[doc = "Bits 16:20 - FLASH memory size"]
    #[inline(always)]
    pub fn fsize(&self) -> FSIZE_R {
        FSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Chip select high time"]
    #[inline(always)]
    pub fn csht(&self) -> CSHT_R {
        CSHT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 0 - Mode 0 / mode 3"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:20 - FLASH memory size"]
    #[inline(always)]
    pub fn fsize(&mut self) -> FSIZE_W {
        FSIZE_W { w: self }
    }
    #[doc = "Bits 8:10 - Chip select high time"]
    #[inline(always)]
    pub fn csht(&mut self) -> CSHT_W {
        CSHT_W { w: self }
    }
    #[doc = "Bit 0 - Mode 0 / mode 3"]
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W {
        CKMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr](index.html) module"]
pub struct DCR_SPEC;
impl crate::RegisterSpec for DCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcr::R](R) reader structure"]
impl crate::Readable for DCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcr::W](W) writer structure"]
impl crate::Writable for DCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCR to value 0"]
impl crate::Resettable for DCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
