#[doc = "Register `HAINTMSK` reader"]
pub struct R(crate::R<HAINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HAINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HAINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HAINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HAINTMSK` writer"]
pub struct W(crate::W<HAINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HAINTMSK_SPEC>;
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
impl From<crate::W<HAINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HAINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HAINTM` reader - Channel interrupt mask"]
pub struct HAINTM_R(crate::FieldReader<u16, u16>);
impl HAINTM_R {
    pub(crate) fn new(bits: u16) -> Self {
        HAINTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HAINTM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HAINTM` writer - Channel interrupt mask"]
pub struct HAINTM_W<'a> {
    w: &'a mut W,
}
impl<'a> HAINTM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Channel interrupt mask"]
    #[inline(always)]
    pub fn haintm(&self) -> HAINTM_R {
        HAINTM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel interrupt mask"]
    #[inline(always)]
    pub fn haintm(&mut self) -> HAINTM_W {
        HAINTM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS host all channels interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [haintmsk](index.html) module"]
pub struct HAINTMSK_SPEC;
impl crate::RegisterSpec for HAINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [haintmsk::R](R) reader structure"]
impl crate::Readable for HAINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [haintmsk::W](W) writer structure"]
impl crate::Writable for HAINTMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HAINTMSK to value 0"]
impl crate::Resettable for HAINTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
