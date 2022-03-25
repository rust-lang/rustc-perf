#[doc = "Register `EXTICR3` reader"]
pub struct R(crate::R<EXTICR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR3` writer"]
pub struct W(crate::W<EXTICR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR3_SPEC>;
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
impl From<crate::W<EXTICR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI11` reader - EXTI x configuration (x = 8 to 11)"]
pub struct EXTI11_R(crate::FieldReader<u8, u8>);
impl EXTI11_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI11` writer - EXTI x configuration (x = 8 to 11)"]
pub struct EXTI11_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `EXTI10` reader - EXTI10"]
pub struct EXTI10_R(crate::FieldReader<u8, u8>);
impl EXTI10_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI10` writer - EXTI10"]
pub struct EXTI10_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `EXTI9` reader - EXTI x configuration (x = 8 to 11)"]
pub struct EXTI9_R(crate::FieldReader<u8, u8>);
impl EXTI9_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI9` writer - EXTI x configuration (x = 8 to 11)"]
pub struct EXTI9_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `EXTI8` reader - EXTI x configuration (x = 8 to 11)"]
pub struct EXTI8_R(crate::FieldReader<u8, u8>);
impl EXTI8_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI8` writer - EXTI x configuration (x = 8 to 11)"]
pub struct EXTI8_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - EXTI x configuration (x = 8 to 11)"]
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI10"]
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI x configuration (x = 8 to 11)"]
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EXTI x configuration (x = 8 to 11)"]
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - EXTI x configuration (x = 8 to 11)"]
    #[inline(always)]
    pub fn exti11(&mut self) -> EXTI11_W {
        EXTI11_W { w: self }
    }
    #[doc = "Bits 8:11 - EXTI10"]
    #[inline(always)]
    pub fn exti10(&mut self) -> EXTI10_W {
        EXTI10_W { w: self }
    }
    #[doc = "Bits 4:7 - EXTI x configuration (x = 8 to 11)"]
    #[inline(always)]
    pub fn exti9(&mut self) -> EXTI9_W {
        EXTI9_W { w: self }
    }
    #[doc = "Bits 0:3 - EXTI x configuration (x = 8 to 11)"]
    #[inline(always)]
    pub fn exti8(&mut self) -> EXTI8_W {
        EXTI8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "external interrupt configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr3](index.html) module"]
pub struct EXTICR3_SPEC;
impl crate::RegisterSpec for EXTICR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr3::R](R) reader structure"]
impl crate::Readable for EXTICR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr3::W](W) writer structure"]
impl crate::Writable for EXTICR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTICR3 to value 0"]
impl crate::Resettable for EXTICR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
