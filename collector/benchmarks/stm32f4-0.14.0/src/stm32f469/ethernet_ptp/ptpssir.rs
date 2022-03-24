#[doc = "Register `PTPSSIR` reader"]
pub struct R(crate::R<PTPSSIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPSSIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPSSIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPSSIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTPSSIR` writer"]
pub struct W(crate::W<PTPSSIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPSSIR_SPEC>;
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
impl From<crate::W<PTPSSIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPSSIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STSSI` reader - STSSI"]
pub struct STSSI_R(crate::FieldReader<u8, u8>);
impl STSSI_R {
    pub(crate) fn new(bits: u8) -> Self {
        STSSI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STSSI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STSSI` writer - STSSI"]
pub struct STSSI_W<'a> {
    w: &'a mut W,
}
impl<'a> STSSI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - STSSI"]
    #[inline(always)]
    pub fn stssi(&self) -> STSSI_R {
        STSSI_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - STSSI"]
    #[inline(always)]
    pub fn stssi(&mut self) -> STSSI_W {
        STSSI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PTP subsecond increment register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptpssir](index.html) module"]
pub struct PTPSSIR_SPEC;
impl crate::RegisterSpec for PTPSSIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptpssir::R](R) reader structure"]
impl crate::Readable for PTPSSIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptpssir::W](W) writer structure"]
impl crate::Writable for PTPSSIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTPSSIR to value 0"]
impl crate::Resettable for PTPSSIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
