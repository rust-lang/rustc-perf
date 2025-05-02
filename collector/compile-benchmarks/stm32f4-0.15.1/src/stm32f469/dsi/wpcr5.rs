#[doc = "Register `WPCR5` reader"]
pub struct R(crate::R<WPCR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPCR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPCR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPCR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPCR5` writer"]
pub struct W(crate::W<WPCR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR5_SPEC>;
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
impl From<crate::W<WPCR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THSZERO` reader - tCLK-POST"]
pub type THSZERO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THSZERO` writer - tCLK-POST"]
pub type THSZERO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPCR5_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - tCLK-POST"]
    #[inline(always)]
    pub fn thszero(&self) -> THSZERO_R {
        THSZERO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - tCLK-POST"]
    #[inline(always)]
    pub fn thszero(&mut self) -> THSZERO_W<0> {
        THSZERO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Wrapper PHY Configuration Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr5](index.html) module"]
pub struct WPCR5_SPEC;
impl crate::RegisterSpec for WPCR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpcr5::R](R) reader structure"]
impl crate::Readable for WPCR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpcr5::W](W) writer structure"]
impl crate::Writable for WPCR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WPCR5 to value 0"]
impl crate::Resettable for WPCR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
