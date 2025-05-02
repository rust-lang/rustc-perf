#[doc = "Register `CLCR` reader"]
pub struct R(crate::R<CLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLCR` writer"]
pub struct W(crate::W<CLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLCR_SPEC>;
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
impl From<crate::W<CLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACR` reader - Automatic Clock lane Control"]
pub type ACR_R = crate::BitReader<bool>;
#[doc = "Field `ACR` writer - Automatic Clock lane Control"]
pub type ACR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLCR_SPEC, bool, O>;
#[doc = "Field `DPCC` reader - D-PHY Clock Control"]
pub type DPCC_R = crate::BitReader<bool>;
#[doc = "Field `DPCC` writer - D-PHY Clock Control"]
pub type DPCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Automatic Clock lane Control"]
    #[inline(always)]
    pub fn acr(&self) -> ACR_R {
        ACR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - D-PHY Clock Control"]
    #[inline(always)]
    pub fn dpcc(&self) -> DPCC_R {
        DPCC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Automatic Clock lane Control"]
    #[inline(always)]
    pub fn acr(&mut self) -> ACR_W<1> {
        ACR_W::new(self)
    }
    #[doc = "Bit 0 - D-PHY Clock Control"]
    #[inline(always)]
    pub fn dpcc(&mut self) -> DPCC_W<0> {
        DPCC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Clock Lane Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clcr](index.html) module"]
pub struct CLCR_SPEC;
impl crate::RegisterSpec for CLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clcr::R](R) reader structure"]
impl crate::Readable for CLCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clcr::W](W) writer structure"]
impl crate::Writable for CLCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLCR to value 0"]
impl crate::Resettable for CLCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
