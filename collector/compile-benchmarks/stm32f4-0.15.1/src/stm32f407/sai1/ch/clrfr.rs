#[doc = "Register `CLRFR` writer"]
pub struct W(crate::W<CLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRFR_SPEC>;
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
impl From<crate::W<CLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear overrun / underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COVRUDR_AW {
    #[doc = "1: Clears the OVRUDR flag"]
    Clear = 1,
}
impl From<COVRUDR_AW> for bool {
    #[inline(always)]
    fn from(variant: COVRUDR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COVRUDR` writer - Clear overrun / underrun"]
pub type COVRUDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, COVRUDR_AW, O>;
impl<'a, const O: u8> COVRUDR_W<'a, O> {
    #[doc = "Clears the OVRUDR flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COVRUDR_AW::Clear)
    }
}
#[doc = "Mute detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMUTEDET_AW {
    #[doc = "1: Clears the MUTEDET flag"]
    Clear = 1,
}
impl From<CMUTEDET_AW> for bool {
    #[inline(always)]
    fn from(variant: CMUTEDET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMUTEDET` writer - Mute detection flag"]
pub type CMUTEDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CMUTEDET_AW, O>;
impl<'a, const O: u8> CMUTEDET_W<'a, O> {
    #[doc = "Clears the MUTEDET flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMUTEDET_AW::Clear)
    }
}
#[doc = "Clear wrong clock configuration flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWCKCFG_AW {
    #[doc = "1: Clears the WCKCFG flag"]
    Clear = 1,
}
impl From<CWCKCFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CWCKCFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWCKCFG` writer - Clear wrong clock configuration flag"]
pub type CWCKCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CWCKCFG_AW, O>;
impl<'a, const O: u8> CWCKCFG_W<'a, O> {
    #[doc = "Clears the WCKCFG flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWCKCFG_AW::Clear)
    }
}
#[doc = "Clear codec not ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCNRDY_AW {
    #[doc = "1: Clears the CNRDY flag"]
    Clear = 1,
}
impl From<CCNRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: CCNRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCNRDY` writer - Clear codec not ready flag"]
pub type CCNRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CCNRDY_AW, O>;
impl<'a, const O: u8> CCNRDY_W<'a, O> {
    #[doc = "Clears the CNRDY flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCNRDY_AW::Clear)
    }
}
#[doc = "Clear anticipated frame synchronization detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAFSDET_AW {
    #[doc = "1: Clears the AFSDET flag"]
    Clear = 1,
}
impl From<CAFSDET_AW> for bool {
    #[inline(always)]
    fn from(variant: CAFSDET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag"]
pub type CAFSDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CAFSDET_AW, O>;
impl<'a, const O: u8> CAFSDET_W<'a, O> {
    #[doc = "Clears the AFSDET flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CAFSDET_AW::Clear)
    }
}
#[doc = "Clear late frame synchronization detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLFSDET_AW {
    #[doc = "1: Clears the LFSDET flag"]
    Clear = 1,
}
impl From<CLFSDET_AW> for bool {
    #[inline(always)]
    fn from(variant: CLFSDET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLFSDET` writer - Clear late frame synchronization detection flag"]
pub type CLFSDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CLFSDET_AW, O>;
impl<'a, const O: u8> CLFSDET_W<'a, O> {
    #[doc = "Clears the LFSDET flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CLFSDET_AW::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - Clear overrun / underrun"]
    #[inline(always)]
    pub fn covrudr(&mut self) -> COVRUDR_W<0> {
        COVRUDR_W::new(self)
    }
    #[doc = "Bit 1 - Mute detection flag"]
    #[inline(always)]
    pub fn cmutedet(&mut self) -> CMUTEDET_W<1> {
        CMUTEDET_W::new(self)
    }
    #[doc = "Bit 2 - Clear wrong clock configuration flag"]
    #[inline(always)]
    pub fn cwckcfg(&mut self) -> CWCKCFG_W<2> {
        CWCKCFG_W::new(self)
    }
    #[doc = "Bit 4 - Clear codec not ready flag"]
    #[inline(always)]
    pub fn ccnrdy(&mut self) -> CCNRDY_W<4> {
        CCNRDY_W::new(self)
    }
    #[doc = "Bit 5 - Clear anticipated frame synchronization detection flag"]
    #[inline(always)]
    pub fn cafsdet(&mut self) -> CAFSDET_W<5> {
        CAFSDET_W::new(self)
    }
    #[doc = "Bit 6 - Clear late frame synchronization detection flag"]
    #[inline(always)]
    pub fn clfsdet(&mut self) -> CLFSDET_W<6> {
        CLFSDET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI AClear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrfr](index.html) module"]
pub struct CLRFR_SPEC;
impl crate::RegisterSpec for CLRFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clrfr::W](W) writer structure"]
impl crate::Writable for CLRFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLRFR to value 0"]
impl crate::Resettable for CLRFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
