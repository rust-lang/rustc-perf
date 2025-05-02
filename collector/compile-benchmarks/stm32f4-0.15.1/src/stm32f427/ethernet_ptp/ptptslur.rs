#[doc = "Register `PTPTSLUR` reader"]
pub struct R(crate::R<PTPTSLUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPTSLUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPTSLUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPTSLUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTPTSLUR` writer"]
pub struct W(crate::W<PTPTSLUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPTSLUR_SPEC>;
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
impl From<crate::W<PTPTSLUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPTSLUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSUSS` reader - TSUSS"]
pub type TSUSS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSUSS` writer - TSUSS"]
pub type TSUSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTPTSLUR_SPEC, u32, u32, 31, O>;
#[doc = "Field `TSUPNS` reader - TSUPNS"]
pub type TSUPNS_R = crate::BitReader<bool>;
#[doc = "Field `TSUPNS` writer - TSUPNS"]
pub type TSUPNS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPTSLUR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:30 - TSUSS"]
    #[inline(always)]
    pub fn tsuss(&self) -> TSUSS_R {
        TSUSS_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - TSUPNS"]
    #[inline(always)]
    pub fn tsupns(&self) -> TSUPNS_R {
        TSUPNS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - TSUSS"]
    #[inline(always)]
    pub fn tsuss(&mut self) -> TSUSS_W<0> {
        TSUSS_W::new(self)
    }
    #[doc = "Bit 31 - TSUPNS"]
    #[inline(always)]
    pub fn tsupns(&mut self) -> TSUPNS_W<31> {
        TSUPNS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PTP time stamp low update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptptslur](index.html) module"]
pub struct PTPTSLUR_SPEC;
impl crate::RegisterSpec for PTPTSLUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptptslur::R](R) reader structure"]
impl crate::Readable for PTPTSLUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptptslur::W](W) writer structure"]
impl crate::Writable for PTPTSLUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTPTSLUR to value 0"]
impl crate::Resettable for PTPTSLUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
