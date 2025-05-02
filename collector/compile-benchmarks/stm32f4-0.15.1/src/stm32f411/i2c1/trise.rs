#[doc = "Register `TRISE` reader"]
pub struct R(crate::R<TRISE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRISE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRISE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRISE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRISE` writer"]
pub struct W(crate::W<TRISE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRISE_SPEC>;
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
impl From<crate::W<TRISE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRISE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRISE` reader - Maximum rise time in Fast/Standard mode (Master mode)"]
pub type TRISE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRISE` writer - Maximum rise time in Fast/Standard mode (Master mode)"]
pub type TRISE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TRISE_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Maximum rise time in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn trise(&self) -> TRISE_R {
        TRISE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Maximum rise time in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn trise(&mut self) -> TRISE_W<0> {
        TRISE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRISE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trise](index.html) module"]
pub struct TRISE_SPEC;
impl crate::RegisterSpec for TRISE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trise::R](R) reader structure"]
impl crate::Readable for TRISE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trise::W](W) writer structure"]
impl crate::Writable for TRISE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRISE to value 0x02"]
impl crate::Resettable for TRISE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
