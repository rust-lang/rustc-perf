#[doc = "Register `OPTCR1` reader"]
pub struct R(crate::R<OPTCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTCR1` writer"]
pub struct W(crate::W<OPTCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTCR1_SPEC>;
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
impl From<crate::W<OPTCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `nWRP` reader - Not write protect"]
pub type NWRP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `nWRP` writer - Not write protect"]
pub type NWRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTCR1_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 16:27 - Not write protect"]
    #[inline(always)]
    pub fn n_wrp(&self) -> NWRP_R {
        NWRP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - Not write protect"]
    #[inline(always)]
    pub fn n_wrp(&mut self) -> NWRP_W<16> {
        NWRP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash option control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optcr1](index.html) module"]
pub struct OPTCR1_SPEC;
impl crate::RegisterSpec for OPTCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optcr1::R](R) reader structure"]
impl crate::Readable for OPTCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optcr1::W](W) writer structure"]
impl crate::Writable for OPTCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPTCR1 to value 0x0fff_0000"]
impl crate::Resettable for OPTCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_0000
    }
}
