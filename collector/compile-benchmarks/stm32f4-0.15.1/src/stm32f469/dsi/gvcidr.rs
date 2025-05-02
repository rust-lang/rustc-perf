#[doc = "Register `GVCIDR` reader"]
pub struct R(crate::R<GVCIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GVCIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GVCIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GVCIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GVCIDR` writer"]
pub struct W(crate::W<GVCIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GVCIDR_SPEC>;
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
impl From<crate::W<GVCIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GVCIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCID` reader - Virtual Channel ID"]
pub type VCID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VCID` writer - Virtual Channel ID"]
pub type VCID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GVCIDR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Virtual Channel ID"]
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Virtual Channel ID"]
    #[inline(always)]
    pub fn vcid(&mut self) -> VCID_W<0> {
        VCID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Generic VCID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gvcidr](index.html) module"]
pub struct GVCIDR_SPEC;
impl crate::RegisterSpec for GVCIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gvcidr::R](R) reader structure"]
impl crate::Readable for GVCIDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gvcidr::W](W) writer structure"]
impl crate::Writable for GVCIDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GVCIDR to value 0"]
impl crate::Resettable for GVCIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
