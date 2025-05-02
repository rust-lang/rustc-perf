#[doc = "Register `GLPMCFG` reader"]
pub struct R(crate::R<GLPMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLPMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLPMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLPMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLPMCFG` writer"]
pub struct W(crate::W<GLPMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLPMCFG_SPEC>;
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
impl From<crate::W<GLPMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLPMCFG_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [glpmcfg](index.html) module"]
pub struct GLPMCFG_SPEC;
impl crate::RegisterSpec for GLPMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [glpmcfg::R](R) reader structure"]
impl crate::Readable for GLPMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [glpmcfg::W](W) writer structure"]
impl crate::Writable for GLPMCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GLPMCFG to value 0"]
impl crate::Resettable for GLPMCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
