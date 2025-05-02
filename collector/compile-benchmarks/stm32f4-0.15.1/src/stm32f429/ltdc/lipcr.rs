#[doc = "Register `LIPCR` reader"]
pub struct R(crate::R<LIPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LIPCR` writer"]
pub struct W(crate::W<LIPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIPCR_SPEC>;
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
impl From<crate::W<LIPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LIPOS` reader - Line Interrupt Position"]
pub type LIPOS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LIPOS` writer - Line Interrupt Position"]
pub type LIPOS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, LIPCR_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Line Interrupt Position"]
    #[inline(always)]
    pub fn lipos(&self) -> LIPOS_R {
        LIPOS_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Line Interrupt Position"]
    #[inline(always)]
    pub fn lipos(&mut self) -> LIPOS_W<0> {
        LIPOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line Interrupt Position Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lipcr](index.html) module"]
pub struct LIPCR_SPEC;
impl crate::RegisterSpec for LIPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lipcr::R](R) reader structure"]
impl crate::Readable for LIPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lipcr::W](W) writer structure"]
impl crate::Writable for LIPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LIPCR to value 0"]
impl crate::Resettable for LIPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
