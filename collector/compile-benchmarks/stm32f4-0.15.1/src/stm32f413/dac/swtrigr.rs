#[doc = "Register `SWTRIGR` writer"]
pub struct W(crate::W<SWTRIGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWTRIGR_SPEC>;
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
impl From<crate::W<SWTRIGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWTRIGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DAC channel2 software trigger"]
pub use SWTRIG1_AW as SWTRIG2_AW;
#[doc = "Field `SWTRIG2` writer - DAC channel2 software trigger"]
pub use SWTRIG1_W as SWTRIG2_W;
#[doc = "DAC channel1 software trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTRIG1_AW {
    #[doc = "0: DAC channel X software trigger disabled"]
    Disabled = 0,
    #[doc = "1: DAC channel X software trigger enabled"]
    Enabled = 1,
}
impl From<SWTRIG1_AW> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWTRIG1` writer - DAC channel1 software trigger"]
pub type SWTRIG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWTRIGR_SPEC, SWTRIG1_AW, O>;
impl<'a, const O: u8> SWTRIG1_W<'a, O> {
    #[doc = "DAC channel X software trigger disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWTRIG1_AW::Disabled)
    }
    #[doc = "DAC channel X software trigger enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWTRIG1_AW::Enabled)
    }
}
impl W {
    #[doc = "Bit 1 - DAC channel2 software trigger"]
    #[inline(always)]
    pub fn swtrig2(&mut self) -> SWTRIG2_W<1> {
        SWTRIG2_W::new(self)
    }
    #[doc = "Bit 0 - DAC channel1 software trigger"]
    #[inline(always)]
    pub fn swtrig1(&mut self) -> SWTRIG1_W<0> {
        SWTRIG1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "software trigger register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swtrigr](index.html) module"]
pub struct SWTRIGR_SPEC;
impl crate::RegisterSpec for SWTRIGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swtrigr::W](W) writer structure"]
impl crate::Writable for SWTRIGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWTRIGR to value 0"]
impl crate::Resettable for SWTRIGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
