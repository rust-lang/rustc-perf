#[doc = "Register `LPCR` reader"]
pub struct R(crate::R<LPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPCR` writer"]
pub struct W(crate::W<LPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPCR_SPEC>;
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
impl From<crate::W<LPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSP` reader - HSYNC Polarity"]
pub type HSP_R = crate::BitReader<bool>;
#[doc = "Field `HSP` writer - HSYNC Polarity"]
pub type HSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, bool, O>;
#[doc = "Field `VSP` reader - VSYNC Polarity"]
pub type VSP_R = crate::BitReader<bool>;
#[doc = "Field `VSP` writer - VSYNC Polarity"]
pub type VSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, bool, O>;
#[doc = "Field `DEP` reader - Data Enable Polarity"]
pub type DEP_R = crate::BitReader<bool>;
#[doc = "Field `DEP` writer - Data Enable Polarity"]
pub type DEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - HSYNC Polarity"]
    #[inline(always)]
    pub fn hsp(&self) -> HSP_R {
        HSP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - VSYNC Polarity"]
    #[inline(always)]
    pub fn vsp(&self) -> VSP_R {
        VSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Data Enable Polarity"]
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - HSYNC Polarity"]
    #[inline(always)]
    pub fn hsp(&mut self) -> HSP_W<2> {
        HSP_W::new(self)
    }
    #[doc = "Bit 1 - VSYNC Polarity"]
    #[inline(always)]
    pub fn vsp(&mut self) -> VSP_W<1> {
        VSP_W::new(self)
    }
    #[doc = "Bit 0 - Data Enable Polarity"]
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W<0> {
        DEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host LTDC Polarity Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpcr](index.html) module"]
pub struct LPCR_SPEC;
impl crate::RegisterSpec for LPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpcr::R](R) reader structure"]
impl crate::Readable for LPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpcr::W](W) writer structure"]
impl crate::Writable for LPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPCR to value 0"]
impl crate::Resettable for LPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
