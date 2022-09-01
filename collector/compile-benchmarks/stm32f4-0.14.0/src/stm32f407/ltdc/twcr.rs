#[doc = "Register `TWCR` reader"]
pub struct R(crate::R<TWCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWCR` writer"]
pub struct W(crate::W<TWCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWCR_SPEC>;
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
impl From<crate::W<TWCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOTALW` reader - Total Width (in units of pixel clock period)"]
pub struct TOTALW_R(crate::FieldReader<u16, u16>);
impl TOTALW_R {
    pub(crate) fn new(bits: u16) -> Self {
        TOTALW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOTALW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOTALW` writer - Total Width (in units of pixel clock period)"]
pub struct TOTALW_W<'a> {
    w: &'a mut W,
}
impl<'a> TOTALW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `TOTALH` reader - Total Height (in units of horizontal scan line)"]
pub struct TOTALH_R(crate::FieldReader<u16, u16>);
impl TOTALH_R {
    pub(crate) fn new(bits: u16) -> Self {
        TOTALH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOTALH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOTALH` writer - Total Height (in units of horizontal scan line)"]
pub struct TOTALH_W<'a> {
    w: &'a mut W,
}
impl<'a> TOTALH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27 - Total Width (in units of pixel clock period)"]
    #[inline(always)]
    pub fn totalw(&self) -> TOTALW_R {
        TOTALW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:10 - Total Height (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn totalh(&self) -> TOTALH_R {
        TOTALH_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - Total Width (in units of pixel clock period)"]
    #[inline(always)]
    pub fn totalw(&mut self) -> TOTALW_W {
        TOTALW_W { w: self }
    }
    #[doc = "Bits 0:10 - Total Height (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn totalh(&mut self) -> TOTALH_W {
        TOTALH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Total Width Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twcr](index.html) module"]
pub struct TWCR_SPEC;
impl crate::RegisterSpec for TWCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twcr::R](R) reader structure"]
impl crate::Readable for TWCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twcr::W](W) writer structure"]
impl crate::Writable for TWCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWCR to value 0"]
impl crate::Resettable for TWCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
