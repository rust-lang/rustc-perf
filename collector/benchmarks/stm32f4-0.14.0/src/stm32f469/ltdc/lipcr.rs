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
pub struct LIPOS_R(crate::FieldReader<u16, u16>);
impl LIPOS_R {
    pub(crate) fn new(bits: u16) -> Self {
        LIPOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LIPOS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIPOS` writer - Line Interrupt Position"]
pub struct LIPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> LIPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
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
    pub fn lipos(&mut self) -> LIPOS_W {
        LIPOS_W { w: self }
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
