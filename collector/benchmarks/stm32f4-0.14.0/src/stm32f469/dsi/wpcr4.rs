#[doc = "Register `WPCR4` reader"]
pub struct R(crate::R<WPCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPCR4` writer"]
pub struct W(crate::W<WPCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR4_SPEC>;
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
impl From<crate::W<WPCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLPXC` reader - tLPXC for Clock lane"]
pub struct TLPXC_R(crate::FieldReader<u8, u8>);
impl TLPXC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TLPXC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLPXC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLPXC` writer - tLPXC for Clock lane"]
pub struct TLPXC_W<'a> {
    w: &'a mut W,
}
impl<'a> TLPXC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `THSEXIT` reader - tHSEXIT"]
pub struct THSEXIT_R(crate::FieldReader<u8, u8>);
impl THSEXIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        THSEXIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THSEXIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THSEXIT` writer - tHSEXIT"]
pub struct THSEXIT_W<'a> {
    w: &'a mut W,
}
impl<'a> THSEXIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `TLPXD` reader - tLPX for Data lanes"]
pub struct TLPXD_R(crate::FieldReader<u8, u8>);
impl TLPXD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TLPXD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLPXD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLPXD` writer - tLPX for Data lanes"]
pub struct TLPXD_W<'a> {
    w: &'a mut W,
}
impl<'a> TLPXD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `THSZERO` reader - tHS-ZERO"]
pub struct THSZERO_R(crate::FieldReader<u8, u8>);
impl THSZERO_R {
    pub(crate) fn new(bits: u8) -> Self {
        THSZERO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THSZERO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THSZERO` writer - tHS-ZERO"]
pub struct THSZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> THSZERO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - tLPXC for Clock lane"]
    #[inline(always)]
    pub fn tlpxc(&self) -> TLPXC_R {
        TLPXC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - tHSEXIT"]
    #[inline(always)]
    pub fn thsexit(&self) -> THSEXIT_R {
        THSEXIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - tLPX for Data lanes"]
    #[inline(always)]
    pub fn tlpxd(&self) -> TLPXD_R {
        TLPXD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - tHS-ZERO"]
    #[inline(always)]
    pub fn thszero(&self) -> THSZERO_R {
        THSZERO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - tLPXC for Clock lane"]
    #[inline(always)]
    pub fn tlpxc(&mut self) -> TLPXC_W {
        TLPXC_W { w: self }
    }
    #[doc = "Bits 16:23 - tHSEXIT"]
    #[inline(always)]
    pub fn thsexit(&mut self) -> THSEXIT_W {
        THSEXIT_W { w: self }
    }
    #[doc = "Bits 8:15 - tLPX for Data lanes"]
    #[inline(always)]
    pub fn tlpxd(&mut self) -> TLPXD_W {
        TLPXD_W { w: self }
    }
    #[doc = "Bits 0:7 - tHS-ZERO"]
    #[inline(always)]
    pub fn thszero(&mut self) -> THSZERO_W {
        THSZERO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI_WPCR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr4](index.html) module"]
pub struct WPCR4_SPEC;
impl crate::RegisterSpec for WPCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpcr4::R](R) reader structure"]
impl crate::Readable for WPCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpcr4::W](W) writer structure"]
impl crate::Writable for WPCR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WPCR4 to value 0x3133_302a"]
impl crate::Resettable for WPCR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3133_302a
    }
}
