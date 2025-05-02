#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOCKDIV` reader - TOCKDIV"]
pub type TOCKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOCKDIV` writer - TOCKDIV"]
pub type TOCKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `TXECKDIV` reader - TXECKDIV"]
pub type TXECKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXECKDIV` writer - TXECKDIV"]
pub type TXECKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 8:15 - TOCKDIV"]
    #[inline(always)]
    pub fn tockdiv(&self) -> TOCKDIV_R {
        TOCKDIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - TXECKDIV"]
    #[inline(always)]
    pub fn txeckdiv(&self) -> TXECKDIV_R {
        TXECKDIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - TOCKDIV"]
    #[inline(always)]
    pub fn tockdiv(&mut self) -> TOCKDIV_W<8> {
        TOCKDIV_W::new(self)
    }
    #[doc = "Bits 0:7 - TXECKDIV"]
    #[inline(always)]
    pub fn txeckdiv(&mut self) -> TXECKDIV_W<0> {
        TXECKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI HOST Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR to value 0x3133_302a"]
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3133_302a
    }
}
