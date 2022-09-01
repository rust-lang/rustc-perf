#[doc = "Register `FPCAR` reader"]
pub struct R(crate::R<FPCAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPCAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPCAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPCAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPCAR` writer"]
pub struct W(crate::W<FPCAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPCAR_SPEC>;
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
impl From<crate::W<FPCAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPCAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS` reader - Location of unpopulated floating-point"]
pub struct ADDRESS_R(crate::FieldReader<u32, u32>);
impl ADDRESS_R {
    pub(crate) fn new(bits: u32) -> Self {
        ADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRESS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRESS` writer - Location of unpopulated floating-point"]
pub struct ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - Location of unpopulated floating-point"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:31 - Location of unpopulated floating-point"]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W {
        ADDRESS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Floating-point context address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpcar](index.html) module"]
pub struct FPCAR_SPEC;
impl crate::RegisterSpec for FPCAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpcar::R](R) reader structure"]
impl crate::Readable for FPCAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpcar::W](W) writer structure"]
impl crate::Writable for FPCAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FPCAR to value 0"]
impl crate::Resettable for FPCAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
