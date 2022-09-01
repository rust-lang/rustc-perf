#[doc = "Register `PTPTTHR` reader"]
pub struct R(crate::R<PTPTTHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPTTHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPTTHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPTTHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTPTTHR` writer"]
pub struct W(crate::W<PTPTTHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPTTHR_SPEC>;
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
impl From<crate::W<PTPTTHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPTTHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTSH` reader - 0"]
pub struct TTSH_R(crate::FieldReader<u32, u32>);
impl TTSH_R {
    pub(crate) fn new(bits: u32) -> Self {
        TTSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTSH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTSH` writer - 0"]
pub struct TTSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TTSH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 0"]
    #[inline(always)]
    pub fn ttsh(&self) -> TTSH_R {
        TTSH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 0"]
    #[inline(always)]
    pub fn ttsh(&mut self) -> TTSH_W {
        TTSH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PTP target time high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptptthr](index.html) module"]
pub struct PTPTTHR_SPEC;
impl crate::RegisterSpec for PTPTTHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptptthr::R](R) reader structure"]
impl crate::Readable for PTPTTHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptptthr::W](W) writer structure"]
impl crate::Writable for PTPTTHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTPTTHR to value 0"]
impl crate::Resettable for PTPTTHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
