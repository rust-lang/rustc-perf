#[doc = "Register `JCHGR` reader"]
pub struct R(crate::R<JCHGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JCHGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JCHGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JCHGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JCHGR` writer"]
pub struct W(crate::W<JCHGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JCHGR_SPEC>;
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
impl From<crate::W<JCHGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JCHGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JCHG` reader - Injected channel group selection"]
pub struct JCHG_R(crate::FieldReader<u8, u8>);
impl JCHG_R {
    pub(crate) fn new(bits: u8) -> Self {
        JCHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JCHG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JCHG` writer - Injected channel group selection"]
pub struct JCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> JCHG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Injected channel group selection"]
    #[inline(always)]
    pub fn jchg(&self) -> JCHG_R {
        JCHG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Injected channel group selection"]
    #[inline(always)]
    pub fn jchg(&mut self) -> JCHG_W {
        JCHG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "injected channel group selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jchgr](index.html) module"]
pub struct JCHGR_SPEC;
impl crate::RegisterSpec for JCHGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jchgr::R](R) reader structure"]
impl crate::Readable for JCHGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jchgr::W](W) writer structure"]
impl crate::Writable for JCHGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets JCHGR to value 0x01"]
impl crate::Resettable for JCHGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
