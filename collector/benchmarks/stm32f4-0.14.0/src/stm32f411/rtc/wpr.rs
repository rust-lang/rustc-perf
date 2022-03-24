#[doc = "Register `WPR` writer"]
pub struct W(crate::W<WPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPR_SPEC>;
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
impl From<crate::W<WPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` writer - Write protection key"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Write protection key"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "write protection register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpr](index.html) module"]
pub struct WPR_SPEC;
impl crate::RegisterSpec for WPR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wpr::W](W) writer structure"]
impl crate::Writable for WPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WPR to value 0"]
impl crate::Resettable for WPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
