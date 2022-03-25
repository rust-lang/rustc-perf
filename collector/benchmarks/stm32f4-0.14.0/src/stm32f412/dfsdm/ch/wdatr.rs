#[doc = "Register `WDATR` reader"]
pub struct R(crate::R<WDATR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDATR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDATR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDATR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDATR` writer"]
pub struct W(crate::W<WDATR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDATR_SPEC>;
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
impl From<crate::W<WDATR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDATR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDATA` reader - WDATA"]
pub struct WDATA_R(crate::FieldReader<u16, u16>);
impl WDATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        WDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDATA` writer - WDATA"]
pub struct WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W {
        WDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel watchdog filter data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdatr](index.html) module"]
pub struct WDATR_SPEC;
impl crate::RegisterSpec for WDATR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdatr::R](R) reader structure"]
impl crate::Readable for WDATR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdatr::W](W) writer structure"]
impl crate::Writable for WDATR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDATR to value 0"]
impl crate::Resettable for WDATR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
