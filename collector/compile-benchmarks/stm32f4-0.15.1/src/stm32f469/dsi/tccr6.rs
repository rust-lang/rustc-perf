#[doc = "Register `TCCR6` reader"]
pub struct R(crate::R<TCCR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR6` writer"]
pub struct W(crate::W<TCCR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR6_SPEC>;
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
impl From<crate::W<TCCR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BTA_TOCNT` reader - Bus-Turn-Around Timeout Counter"]
pub type BTA_TOCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BTA_TOCNT` writer - Bus-Turn-Around Timeout Counter"]
pub type BTA_TOCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCCR6_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Bus-Turn-Around Timeout Counter"]
    #[inline(always)]
    pub fn bta_tocnt(&self) -> BTA_TOCNT_R {
        BTA_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bus-Turn-Around Timeout Counter"]
    #[inline(always)]
    pub fn bta_tocnt(&mut self) -> BTA_TOCNT_W<0> {
        BTA_TOCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Timeout Counter Configuration Register6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr6](index.html) module"]
pub struct TCCR6_SPEC;
impl crate::RegisterSpec for TCCR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tccr6::R](R) reader structure"]
impl crate::Readable for TCCR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr6::W](W) writer structure"]
impl crate::Writable for TCCR6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCCR6 to value 0"]
impl crate::Resettable for TCCR6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
