#[doc = "Register `DCCR` reader"]
pub struct R(crate::R<DCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCCR` writer"]
pub struct W(crate::W<DCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCCR_SPEC>;
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
impl From<crate::W<DCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCALPHA` reader - Default Color Alpha"]
pub struct DCALPHA_R(crate::FieldReader<u8, u8>);
impl DCALPHA_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCALPHA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCALPHA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCALPHA` writer - Default Color Alpha"]
pub struct DCALPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCALPHA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `DCRED` reader - Default Color Red"]
pub struct DCRED_R(crate::FieldReader<u8, u8>);
impl DCRED_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCRED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCRED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCRED` writer - Default Color Red"]
pub struct DCRED_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `DCGREEN` reader - Default Color Green"]
pub struct DCGREEN_R(crate::FieldReader<u8, u8>);
impl DCGREEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCGREEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCGREEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCGREEN` writer - Default Color Green"]
pub struct DCGREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCGREEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `DCBLUE` reader - Default Color Blue"]
pub struct DCBLUE_R(crate::FieldReader<u8, u8>);
impl DCBLUE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCBLUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCBLUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCBLUE` writer - Default Color Blue"]
pub struct DCBLUE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCBLUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Default Color Alpha"]
    #[inline(always)]
    pub fn dcalpha(&self) -> DCALPHA_R {
        DCALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Default Color Red"]
    #[inline(always)]
    pub fn dcred(&self) -> DCRED_R {
        DCRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Default Color Green"]
    #[inline(always)]
    pub fn dcgreen(&self) -> DCGREEN_R {
        DCGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Default Color Blue"]
    #[inline(always)]
    pub fn dcblue(&self) -> DCBLUE_R {
        DCBLUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Default Color Alpha"]
    #[inline(always)]
    pub fn dcalpha(&mut self) -> DCALPHA_W {
        DCALPHA_W { w: self }
    }
    #[doc = "Bits 16:23 - Default Color Red"]
    #[inline(always)]
    pub fn dcred(&mut self) -> DCRED_W {
        DCRED_W { w: self }
    }
    #[doc = "Bits 8:15 - Default Color Green"]
    #[inline(always)]
    pub fn dcgreen(&mut self) -> DCGREEN_W {
        DCGREEN_W { w: self }
    }
    #[doc = "Bits 0:7 - Default Color Blue"]
    #[inline(always)]
    pub fn dcblue(&mut self) -> DCBLUE_W {
        DCBLUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layerx Default Color Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccr](index.html) module"]
pub struct DCCR_SPEC;
impl crate::RegisterSpec for DCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dccr::R](R) reader structure"]
impl crate::Readable for DCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dccr::W](W) writer structure"]
impl crate::Writable for DCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCCR to value 0"]
impl crate::Resettable for DCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
