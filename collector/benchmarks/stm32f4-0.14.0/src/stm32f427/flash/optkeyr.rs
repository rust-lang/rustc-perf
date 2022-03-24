#[doc = "Register `OPTKEYR` writer"]
pub struct W(crate::W<OPTKEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTKEYR_SPEC>;
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
impl From<crate::W<OPTKEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTKEYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPTKEY` writer - Option byte key"]
pub struct OPTKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Option byte key"]
    #[inline(always)]
    pub fn optkey(&mut self) -> OPTKEY_W {
        OPTKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash option key register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optkeyr](index.html) module"]
pub struct OPTKEYR_SPEC;
impl crate::RegisterSpec for OPTKEYR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [optkeyr::W](W) writer structure"]
impl crate::Writable for OPTKEYR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPTKEYR to value 0"]
impl crate::Resettable for OPTKEYR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
