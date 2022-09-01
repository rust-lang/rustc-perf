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
pub struct CMDSIZE_R(crate::FieldReader<u16, u16>);
impl CMDSIZE_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMDSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDSIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDSIZE` writer - Command Size"]
pub struct CMDSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
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
    pub fn cmdsize(&mut self) -> CMDSIZE_W {
        CMDSIZE_W { w: self }
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
