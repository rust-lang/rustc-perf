#[doc = "Register `ACTRL` reader"]
pub struct R(crate::R<ACTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACTRL` writer"]
pub struct W(crate::W<ACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTRL_SPEC>;
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
impl From<crate::W<ACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISMCYCINT` reader - DISMCYCINT"]
pub type DISMCYCINT_R = crate::BitReader<bool>;
#[doc = "Field `DISMCYCINT` writer - DISMCYCINT"]
pub type DISMCYCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTRL_SPEC, bool, O>;
#[doc = "Field `DISDEFWBUF` reader - DISDEFWBUF"]
pub type DISDEFWBUF_R = crate::BitReader<bool>;
#[doc = "Field `DISDEFWBUF` writer - DISDEFWBUF"]
pub type DISDEFWBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTRL_SPEC, bool, O>;
#[doc = "Field `DISFOLD` reader - DISFOLD"]
pub type DISFOLD_R = crate::BitReader<bool>;
#[doc = "Field `DISFOLD` writer - DISFOLD"]
pub type DISFOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTRL_SPEC, bool, O>;
#[doc = "Field `DISFPCA` reader - DISFPCA"]
pub type DISFPCA_R = crate::BitReader<bool>;
#[doc = "Field `DISFPCA` writer - DISFPCA"]
pub type DISFPCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTRL_SPEC, bool, O>;
#[doc = "Field `DISOOFP` reader - DISOOFP"]
pub type DISOOFP_R = crate::BitReader<bool>;
#[doc = "Field `DISOOFP` writer - DISOOFP"]
pub type DISOOFP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DISMCYCINT"]
    #[inline(always)]
    pub fn dismcycint(&self) -> DISMCYCINT_R {
        DISMCYCINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DISDEFWBUF"]
    #[inline(always)]
    pub fn disdefwbuf(&self) -> DISDEFWBUF_R {
        DISDEFWBUF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DISFOLD"]
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - DISFPCA"]
    #[inline(always)]
    pub fn disfpca(&self) -> DISFPCA_R {
        DISFPCA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DISOOFP"]
    #[inline(always)]
    pub fn disoofp(&self) -> DISOOFP_R {
        DISOOFP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DISMCYCINT"]
    #[inline(always)]
    pub fn dismcycint(&mut self) -> DISMCYCINT_W<0> {
        DISMCYCINT_W::new(self)
    }
    #[doc = "Bit 1 - DISDEFWBUF"]
    #[inline(always)]
    pub fn disdefwbuf(&mut self) -> DISDEFWBUF_W<1> {
        DISDEFWBUF_W::new(self)
    }
    #[doc = "Bit 2 - DISFOLD"]
    #[inline(always)]
    pub fn disfold(&mut self) -> DISFOLD_W<2> {
        DISFOLD_W::new(self)
    }
    #[doc = "Bit 8 - DISFPCA"]
    #[inline(always)]
    pub fn disfpca(&mut self) -> DISFPCA_W<8> {
        DISFPCA_W::new(self)
    }
    #[doc = "Bit 9 - DISOOFP"]
    #[inline(always)]
    pub fn disoofp(&mut self) -> DISOOFP_W<9> {
        DISOOFP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actrl](index.html) module"]
pub struct ACTRL_SPEC;
impl crate::RegisterSpec for ACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [actrl::R](R) reader structure"]
impl crate::Readable for ACTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [actrl::W](W) writer structure"]
impl crate::Writable for ACTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACTRL to value 0"]
impl crate::Resettable for ACTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
