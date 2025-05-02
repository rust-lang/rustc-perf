#[doc = "Register `MMCTIMR` reader"]
pub struct R(crate::R<MMCTIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCTIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCTIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCTIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMCTIMR` writer"]
pub struct W(crate::W<MMCTIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCTIMR_SPEC>;
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
impl From<crate::W<MMCTIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCTIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmitted good frames single collision mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGFSCM_A {
    #[doc = "0: Transmitted-good-single-collision half-full interrupt enabled"]
    Unmasked = 0,
    #[doc = "1: Transmitted-good-single-collision half-full interrupt disabled"]
    Masked = 1,
}
impl From<TGFSCM_A> for bool {
    #[inline(always)]
    fn from(variant: TGFSCM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TGFSCM` reader - Transmitted good frames single collision mask"]
pub type TGFSCM_R = crate::BitReader<TGFSCM_A>;
impl TGFSCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TGFSCM_A {
        match self.bits {
            false => TGFSCM_A::Unmasked,
            true => TGFSCM_A::Masked,
        }
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TGFSCM_A::Unmasked
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TGFSCM_A::Masked
    }
}
#[doc = "Field `TGFSCM` writer - Transmitted good frames single collision mask"]
pub type TGFSCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCTIMR_SPEC, TGFSCM_A, O>;
impl<'a, const O: u8> TGFSCM_W<'a, O> {
    #[doc = "Transmitted-good-single-collision half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TGFSCM_A::Unmasked)
    }
    #[doc = "Transmitted-good-single-collision half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TGFSCM_A::Masked)
    }
}
#[doc = "Transmitted good frames more than single collision mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGFMSCM_A {
    #[doc = "0: Transmitted-good-multiple-collision half-full interrupt enabled"]
    Unmasked = 0,
    #[doc = "1: Transmitted-good-multiple-collision half-full interrupt disabled"]
    Masked = 1,
}
impl From<TGFMSCM_A> for bool {
    #[inline(always)]
    fn from(variant: TGFMSCM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TGFMSCM` reader - Transmitted good frames more than single collision mask"]
pub type TGFMSCM_R = crate::BitReader<TGFMSCM_A>;
impl TGFMSCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TGFMSCM_A {
        match self.bits {
            false => TGFMSCM_A::Unmasked,
            true => TGFMSCM_A::Masked,
        }
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TGFMSCM_A::Unmasked
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TGFMSCM_A::Masked
    }
}
#[doc = "Field `TGFMSCM` writer - Transmitted good frames more than single collision mask"]
pub type TGFMSCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCTIMR_SPEC, TGFMSCM_A, O>;
impl<'a, const O: u8> TGFMSCM_W<'a, O> {
    #[doc = "Transmitted-good-multiple-collision half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TGFMSCM_A::Unmasked)
    }
    #[doc = "Transmitted-good-multiple-collision half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TGFMSCM_A::Masked)
    }
}
#[doc = "Transmitted good frames mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGFM_A {
    #[doc = "0: Transmitted-good counter half-full interrupt enabled"]
    Unmasked = 0,
    #[doc = "1: Transmitted-good counter half-full interrupt disabled"]
    Masked = 1,
}
impl From<TGFM_A> for bool {
    #[inline(always)]
    fn from(variant: TGFM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TGFM` reader - Transmitted good frames mask"]
pub type TGFM_R = crate::BitReader<TGFM_A>;
impl TGFM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TGFM_A {
        match self.bits {
            false => TGFM_A::Unmasked,
            true => TGFM_A::Masked,
        }
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TGFM_A::Unmasked
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TGFM_A::Masked
    }
}
#[doc = "Field `TGFM` writer - Transmitted good frames mask"]
pub type TGFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCTIMR_SPEC, TGFM_A, O>;
impl<'a, const O: u8> TGFM_W<'a, O> {
    #[doc = "Transmitted-good counter half-full interrupt enabled"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(TGFM_A::Unmasked)
    }
    #[doc = "Transmitted-good counter half-full interrupt disabled"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TGFM_A::Masked)
    }
}
impl R {
    #[doc = "Bit 14 - Transmitted good frames single collision mask"]
    #[inline(always)]
    pub fn tgfscm(&self) -> TGFSCM_R {
        TGFSCM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmitted good frames more than single collision mask"]
    #[inline(always)]
    pub fn tgfmscm(&self) -> TGFMSCM_R {
        TGFMSCM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frames mask"]
    #[inline(always)]
    pub fn tgfm(&self) -> TGFM_R {
        TGFM_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Transmitted good frames single collision mask"]
    #[inline(always)]
    pub fn tgfscm(&mut self) -> TGFSCM_W<14> {
        TGFSCM_W::new(self)
    }
    #[doc = "Bit 15 - Transmitted good frames more than single collision mask"]
    #[inline(always)]
    pub fn tgfmscm(&mut self) -> TGFMSCM_W<15> {
        TGFMSCM_W::new(self)
    }
    #[doc = "Bit 21 - Transmitted good frames mask"]
    #[inline(always)]
    pub fn tgfm(&mut self) -> TGFM_W<21> {
        TGFM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MMC transmit interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmctimr](index.html) module"]
pub struct MMCTIMR_SPEC;
impl crate::RegisterSpec for MMCTIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmctimr::R](R) reader structure"]
impl crate::Readable for MMCTIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmctimr::W](W) writer structure"]
impl crate::Writable for MMCTIMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMCTIMR to value 0"]
impl crate::Resettable for MMCTIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
