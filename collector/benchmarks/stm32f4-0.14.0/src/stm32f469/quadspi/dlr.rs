#[doc = "Register `DLR` reader"]
pub struct R(crate::R<DLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLR` writer"]
pub struct W(crate::W<DLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLR_SPEC>;
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
impl From<crate::W<DLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DL` reader - Data length"]
pub struct DL_R(crate::FieldReader<u32, u32>);
impl DL_R {
    pub(crate) fn new(bits: u32) -> Self {
        DL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DL` writer - Data length"]
pub struct DL_W<'a> {
    w: &'a mut W,
}
impl<'a> DL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data length"]
    #[inline(always)]
    pub fn dl(&self) -> DL_R {
        DL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data length"]
    #[inline(always)]
    pub fn dl(&mut self) -> DL_W {
        DL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "data length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlr](index.html) module"]
pub struct DLR_SPEC;
impl crate::RegisterSpec for DLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlr::R](R) reader structure"]
impl crate::Readable for DLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlr::W](W) writer structure"]
impl crate::Writable for DLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLR to value 0"]
impl crate::Resettable for DLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
