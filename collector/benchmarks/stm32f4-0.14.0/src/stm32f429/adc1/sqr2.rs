#[doc = "Register `SQR2` reader"]
pub struct R(crate::R<SQR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SQR2` writer"]
pub struct W(crate::W<SQR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR2_SPEC>;
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
impl From<crate::W<SQR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQ12` reader - 12th conversion in regular sequence"]
pub struct SQ12_R(crate::FieldReader<u8, u8>);
impl SQ12_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ12` writer - 12th conversion in regular sequence"]
pub struct SQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | ((value as u32 & 0x1f) << 25);
        self.w
    }
}
#[doc = "Field `SQ11` reader - 11th conversion in regular sequence"]
pub struct SQ11_R(crate::FieldReader<u8, u8>);
impl SQ11_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ11` writer - 11th conversion in regular sequence"]
pub struct SQ11_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "Field `SQ10` reader - 10th conversion in regular sequence"]
pub struct SQ10_R(crate::FieldReader<u8, u8>);
impl SQ10_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ10` writer - 10th conversion in regular sequence"]
pub struct SQ10_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | ((value as u32 & 0x1f) << 15);
        self.w
    }
}
#[doc = "Field `SQ9` reader - 9th conversion in regular sequence"]
pub struct SQ9_R(crate::FieldReader<u8, u8>);
impl SQ9_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ9` writer - 9th conversion in regular sequence"]
pub struct SQ9_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `SQ8` reader - 8th conversion in regular sequence"]
pub struct SQ8_R(crate::FieldReader<u8, u8>);
impl SQ8_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ8` writer - 8th conversion in regular sequence"]
pub struct SQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `SQ7` reader - 7th conversion in regular sequence"]
pub struct SQ7_R(crate::FieldReader<u8, u8>);
impl SQ7_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ7` writer - 7th conversion in regular sequence"]
pub struct SQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:29 - 12th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq12(&self) -> SQ12_R {
        SQ12_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 11th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq11(&self) -> SQ11_R {
        SQ11_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 10th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq10(&self) -> SQ10_R {
        SQ10_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 9th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq9(&self) -> SQ9_R {
        SQ9_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 8th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - 7th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:29 - 12th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq12(&mut self) -> SQ12_W {
        SQ12_W { w: self }
    }
    #[doc = "Bits 20:24 - 11th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq11(&mut self) -> SQ11_W {
        SQ11_W { w: self }
    }
    #[doc = "Bits 15:19 - 10th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq10(&mut self) -> SQ10_W {
        SQ10_W { w: self }
    }
    #[doc = "Bits 10:14 - 9th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq9(&mut self) -> SQ9_W {
        SQ9_W { w: self }
    }
    #[doc = "Bits 5:9 - 8th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq8(&mut self) -> SQ8_W {
        SQ8_W { w: self }
    }
    #[doc = "Bits 0:4 - 7th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq7(&mut self) -> SQ7_W {
        SQ7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "regular sequence register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr2](index.html) module"]
pub struct SQR2_SPEC;
impl crate::RegisterSpec for SQR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sqr2::R](R) reader structure"]
impl crate::Readable for SQR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sqr2::W](W) writer structure"]
impl crate::Writable for SQR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SQR2 to value 0"]
impl crate::Resettable for SQR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
