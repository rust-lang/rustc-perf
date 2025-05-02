#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR` writer"]
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCIE` reader - Digest calculation completion interrupt enable"]
pub type DCIE_R = crate::BitReader<bool>;
#[doc = "Field `DCIE` writer - Digest calculation completion interrupt enable"]
pub type DCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `DINIE` reader - Data input interrupt enable"]
pub type DINIE_R = crate::BitReader<bool>;
#[doc = "Field `DINIE` writer - Data input interrupt enable"]
pub type DINIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Digest calculation completion interrupt enable"]
    #[inline(always)]
    pub fn dcie(&self) -> DCIE_R {
        DCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Data input interrupt enable"]
    #[inline(always)]
    pub fn dinie(&self) -> DINIE_R {
        DINIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Digest calculation completion interrupt enable"]
    #[inline(always)]
    pub fn dcie(&mut self) -> DCIE_W<1> {
        DCIE_W::new(self)
    }
    #[doc = "Bit 0 - Data input interrupt enable"]
    #[inline(always)]
    pub fn dinie(&mut self) -> DINIE_W<0> {
        DINIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr::W](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
