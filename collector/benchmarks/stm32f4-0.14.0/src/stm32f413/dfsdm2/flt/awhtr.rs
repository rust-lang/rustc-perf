#[doc = "Register `AWHTR` reader"]
pub struct R(crate::R<AWHTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWHTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWHTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWHTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AWHTR` writer"]
pub struct W(crate::W<AWHTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWHTR_SPEC>;
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
impl From<crate::W<AWHTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWHTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AWHT` reader - Analog watchdog high threshold"]
pub struct AWHT_R(crate::FieldReader<u32, u32>);
impl AWHT_R {
    pub(crate) fn new(bits: u32) -> Self {
        AWHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWHT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWHT` writer - Analog watchdog high threshold"]
pub struct AWHT_W<'a> {
    w: &'a mut W,
}
impl<'a> AWHT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Field `BKAWH` reader - Break signal assignment to analog watchdog high threshold event"]
pub struct BKAWH_R(crate::FieldReader<u8, u8>);
impl BKAWH_R {
    pub(crate) fn new(bits: u8) -> Self {
        BKAWH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKAWH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKAWH` writer - Break signal assignment to analog watchdog high threshold event"]
pub struct BKAWH_W<'a> {
    w: &'a mut W,
}
impl<'a> BKAWH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - Analog watchdog high threshold"]
    #[inline(always)]
    pub fn awht(&self) -> AWHT_R {
        AWHT_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event"]
    #[inline(always)]
    pub fn bkawh(&self) -> BKAWH_R {
        BKAWH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - Analog watchdog high threshold"]
    #[inline(always)]
    pub fn awht(&mut self) -> AWHT_W {
        AWHT_W { w: self }
    }
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event"]
    #[inline(always)]
    pub fn bkawh(&mut self) -> BKAWH_W {
        BKAWH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "analog watchdog high threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awhtr](index.html) module"]
pub struct AWHTR_SPEC;
impl crate::RegisterSpec for AWHTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [awhtr::R](R) reader structure"]
impl crate::Readable for AWHTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [awhtr::W](W) writer structure"]
impl crate::Writable for AWHTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AWHTR to value 0"]
impl crate::Resettable for AWHTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
