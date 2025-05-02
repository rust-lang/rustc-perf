#[doc = "Register `FGOR` reader"]
pub struct R(crate::R<FGOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGOR` writer"]
pub struct W(crate::W<FGOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGOR_SPEC>;
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
impl From<crate::W<FGOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LO` reader - Line offset"]
pub type LO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LO` writer - Line offset"]
pub type LO_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FGOR_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Line offset"]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Line offset"]
    #[inline(always)]
    pub fn lo(&mut self) -> LO_W<0> {
        LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "foreground offset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgor](index.html) module"]
pub struct FGOR_SPEC;
impl crate::RegisterSpec for FGOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgor::R](R) reader structure"]
impl crate::Readable for FGOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fgor::W](W) writer structure"]
impl crate::Writable for FGOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FGOR to value 0"]
impl crate::Resettable for FGOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
