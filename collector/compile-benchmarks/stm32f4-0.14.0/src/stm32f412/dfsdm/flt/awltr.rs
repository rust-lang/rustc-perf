#[doc = "Register `AWLTR` reader"]
pub struct R(crate::R<AWLTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWLTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWLTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWLTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AWLTR` writer"]
pub struct W(crate::W<AWLTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWLTR_SPEC>;
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
impl From<crate::W<AWLTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWLTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AWLT` reader - Analog watchdog low threshold"]
pub struct AWLT_R(crate::FieldReader<u32, u32>);
impl AWLT_R {
    pub(crate) fn new(bits: u32) -> Self {
        AWLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWLT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWLT` writer - Analog watchdog low threshold"]
pub struct AWLT_W<'a> {
    w: &'a mut W,
}
impl<'a> AWLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Field `BKAWL` reader - Break signal assignment to analog watchdog low threshold event"]
pub struct BKAWL_R(crate::FieldReader<u8, u8>);
impl BKAWL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BKAWL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKAWL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKAWL` writer - Break signal assignment to analog watchdog low threshold event"]
pub struct BKAWL_W<'a> {
    w: &'a mut W,
}
impl<'a> BKAWL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - Analog watchdog low threshold"]
    #[inline(always)]
    pub fn awlt(&self) -> AWLT_R {
        AWLT_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event"]
    #[inline(always)]
    pub fn bkawl(&self) -> BKAWL_R {
        BKAWL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - Analog watchdog low threshold"]
    #[inline(always)]
    pub fn awlt(&mut self) -> AWLT_W {
        AWLT_W { w: self }
    }
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event"]
    #[inline(always)]
    pub fn bkawl(&mut self) -> BKAWL_W {
        BKAWL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "analog watchdog low threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awltr](index.html) module"]
pub struct AWLTR_SPEC;
impl crate::RegisterSpec for AWLTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [awltr::R](R) reader structure"]
impl crate::Readable for AWLTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [awltr::W](W) writer structure"]
impl crate::Writable for AWLTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AWLTR to value 0"]
impl crate::Resettable for AWLTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
