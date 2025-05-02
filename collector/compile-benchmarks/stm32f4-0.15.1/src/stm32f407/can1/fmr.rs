#[doc = "Register `FMR` reader"]
pub struct R(crate::R<FMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMR` writer"]
pub struct W(crate::W<FMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMR_SPEC>;
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
impl From<crate::W<FMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN2SB` reader - CAN2SB"]
pub type CAN2SB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAN2SB` writer - CAN2SB"]
pub type CAN2SB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMR_SPEC, u8, u8, 6, O>;
#[doc = "Field `FINIT` reader - FINIT"]
pub type FINIT_R = crate::BitReader<bool>;
#[doc = "Field `FINIT` writer - FINIT"]
pub type FINIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 8:13 - CAN2SB"]
    #[inline(always)]
    pub fn can2sb(&self) -> CAN2SB_R {
        CAN2SB_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 0 - FINIT"]
    #[inline(always)]
    pub fn finit(&self) -> FINIT_R {
        FINIT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:13 - CAN2SB"]
    #[inline(always)]
    pub fn can2sb(&mut self) -> CAN2SB_W<8> {
        CAN2SB_W::new(self)
    }
    #[doc = "Bit 0 - FINIT"]
    #[inline(always)]
    pub fn finit(&mut self) -> FINIT_W<0> {
        FINIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "filter master register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmr](index.html) module"]
pub struct FMR_SPEC;
impl crate::RegisterSpec for FMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmr::R](R) reader structure"]
impl crate::Readable for FMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmr::W](W) writer structure"]
impl crate::Writable for FMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMR to value 0x2a1c_0e01"]
impl crate::Resettable for FMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2a1c_0e01
    }
}
