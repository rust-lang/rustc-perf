#[doc = "Register `LCCR` reader"]
pub struct R(crate::R<LCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCCR` writer"]
pub struct W(crate::W<LCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCCR_SPEC>;
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
impl From<crate::W<LCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDSIZE` reader - Command Size"]
pub type CMDSIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CMDSIZE` writer - Command Size"]
pub type CMDSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Command Size"]
    #[inline(always)]
    pub fn cmdsize(&self) -> CMDSIZE_R {
        CMDSIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Command Size"]
    #[inline(always)]
    pub fn cmdsize(&mut self) -> CMDSIZE_W<0> {
        CMDSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host LTDC Command Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lccr](index.html) module"]
pub struct LCCR_SPEC;
impl crate::RegisterSpec for LCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lccr::R](R) reader structure"]
impl crate::Readable for LCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lccr::W](W) writer structure"]
impl crate::Writable for LCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCCR to value 0"]
impl crate::Resettable for LCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
